//! A Tracing [Layer][`FlameLayer`] for generating a folded stack trace for generating flamegraphs
//! and flamecharts with [`inferno`]
//!
//! # Overview
//!
//! [`tracing`] is a framework for instrumenting Rust programs to collect
//! scoped, structured, and async-aware diagnostics. `tracing-flame` provides helpers
//! for consuming `tracing` instrumentation that can later be visualized as a
//! flamegraph/flamechart. Flamegraphs/flamecharts are useful for identifying performance
//! issues bottlenecks in an application. For more details, see Brendan Gregg's [post]
//! on flamegraphs.
//!
//! [post]: http://www.brendangregg.com/flamegraphs.html
//!
//! ## Usage
//!
//! This crate is meant to be used in a two step process:
//!
//! 1. Capture textual representation of the spans that are entered and exited
//!    with [`FlameLayer`].
//! 2. Feed the textual representation into `inferno-flamegraph` to generate the
//!    flamegraph or flamechart.
//!
//! *Note*: when using a buffered writer as the writer for a `FlameLayer`, it is necessary to
//! ensure that the buffer has been flushed before the data is passed into
//! [`inferno-flamegraph`]. For more details on how to flush the internal writer
//! of the `FlameLayer`, see the docs for [`FlushGuard`].
//!
//! ## Layer Setup
//!
//! ```rust
//! use std::{fs::File, io::BufWriter};
//! use tracing_flame::FlameLayer;
//! use tracing_subscriber::{registry::Registry, prelude::*, fmt};
//!
//! fn setup_global_subscriber() -> impl Drop {
//!     let fmt_layer = fmt::Layer::default();
//!
//!     let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();
//!
//!     let subscriber = Registry::default()
//!         .with(fmt_layer)
//!         .with(flame_layer);
//!
//!     tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");
//!     _guard
//! }
//!
//! // your code here ..
//! ```
//!
//! As an alternative, you can provide _any_ type that implements `std::io::Write` to
//! `FlameLayer::new`.
//!
//! ## Generating the Image
//!
//! To convert the textual representation of a flamegraph to a visual one, first install `inferno`:
//!
//! ```console
//! cargo install inferno
//! ```
//!
//! Then, pass the file created by `FlameLayer` into `inferno-flamegraph`:
//!
//! ```console
//! # flamegraph
//! cat tracing.folded | inferno-flamegraph > tracing-flamegraph.svg
//!
//! # flamechart
//! cat tracing.folded | inferno-flamegraph --flamechart > tracing-flamechart.svg
//! ```
//!
//! ## Differences between `flamegraph`s and `flamechart`s
//!
//! By default, `inferno-flamegraph` creates flamegraphs. Flamegraphs operate by
//! that collapsing identical stack frames and sorting them on the frame's names.
//!
//! This behavior is great for multithreaded programs and long-running programs
//! where the same frames occur _many_ times, for short durations, because it reduces
//! noise in the graph and gives the reader a better idea of the
//! overall time spent in each part of the application.
//!
//! However, it is sometimes desirable to preserve the _exact_ ordering of events
//! as they were emitted by `tracing-flame`, so that it is clear when each
//! span is entered relative to others and get an accurate visual trace of
//! the execution of your program. This representation is best created with a
//! _flamechart_, which _does not_ sort or collapse identical stack frames.
//!
//! [`tracing`]: https://docs.rs/tracing
//! [`inferno`]: https://docs.rs/inferno
//! [`FlameLayer`]: struct.FlameLayer.html
//! [`FlushGuard`]: struct.FlushGuard.html
//! [`inferno-flamegraph`]: https://docs.rs/inferno/0.9.5/inferno/index.html#producing-a-flame-graph
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

pub use error::Error;

use error::Kind;
use lazy_static::lazy_static;
use std::cell::Cell;
use std::fmt;
use std::fmt::Write as _;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tracing::span;
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::SpanRef;
use tracing_subscriber::Layer;

mod error;

lazy_static! {
    static ref START: Instant = Instant::now();
}

thread_local! {
    static LAST_EVENT: Cell<Instant> = Cell::new(*START);

    static THREAD_NAME: String = {
        let thread = std::thread::current();
        let mut thread_name = format!("{:?}", thread.id());
        if let Some(name) = thread.name() {
            thread_name += "-";
            thread_name += name;
        }
        thread_name
    };
}

/// A `Layer` that records span open/close events as folded flamegraph stack
/// samples.
///
/// The output of `FlameLayer` emulates the output of commands like `perf` once
/// they've been collapsed by `inferno-flamegraph`. The output of this layer
/// should look similar to the output of the following commands:
///
/// ```sh
/// perf record --call-graph dwarf target/release/mybin
/// perf script | inferno-collapse-perf > stacks.folded
/// ```
///
/// # Sample Counts
///
/// Because `tracing-flame` doesn't use sampling, the number at the end of each
/// folded stack trace does not represent a number of samples of that stack.
/// Instead, the numbers on each line are the number of nanoseconds since the
/// last event in the same thread.
///
/// # Dropping and Flushing
///
/// If you use a global subscriber the drop implementations on your various
/// layers will not get called when your program exits. This means that if
/// you're using a buffered writer as the inner writer for the `FlameLayer`
/// you're not guaranteed to see all the events that have been emitted in the
/// file by default.
///
/// To ensure all data is flushed when the program exits, `FlameLayer` exposes
/// the [`flush_on_drop`] function, which returns a [`FlushGuard`]. The `FlushGuard`
/// will flush the writer when it is dropped. If necessary, it can also be used to manually
/// flush the writer.
///
/// [`flush_on_drop`]: struct.FlameLayer.html#method.flush_on_drop
/// [`FlushGuard`]: struct.FlushGuard.html
#[derive(Debug)]
pub struct FlameLayer<S, W> {
    out: Arc<Mutex<W>>,
    _inner: PhantomData<S>,
}

/// An RAII guard for managing flushing a global writer that is
/// otherwise inaccessible.
///
/// This type is only needed when using
/// `tracing::subscriber::set_global_default`, which prevents the drop
/// implementation of layers from running when the program exits.
#[must_use]
#[derive(Debug)]
pub struct FlushGuard<W>
where
    W: Write + 'static,
{
    out: Arc<Mutex<W>>,
}

impl<S, W> FlameLayer<S, W>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: Write + 'static,
{
    /// Returns a new `FlameLayer` that outputs all folded stack samples to the
    /// provided writer.
    pub fn new(writer: W) -> Self {
        // Initialize the start used by all threads when initializing the
        // LAST_EVENT when constructing the layer
        let _unused = *START;
        Self {
            out: Arc::new(Mutex::new(writer)),
            _inner: PhantomData,
        }
    }

    /// Returns a `FlushGuard` which will flush the `FlameLayer`'s writer when
    /// it is dropped, or when `flush` is manually invoked on the guard.
    pub fn flush_on_drop(&self) -> FlushGuard<W> {
        FlushGuard {
            out: self.out.clone(),
        }
    }
}

impl<W> FlushGuard<W>
where
    W: Write + 'static,
{
    /// Flush the internal writer of the `FlameLayer`, ensuring that all
    /// intermediately buffered contents reach their destination.
    pub fn flush(&self) -> Result<(), Error> {
        let mut guard = match self.out.lock() {
            Ok(guard) => guard,
            Err(e) => {
                if !std::thread::panicking() {
                    panic!("{}", e);
                } else {
                    return Ok(());
                }
            }
        };

        guard.flush().map_err(Kind::FlushFile).map_err(Error)
    }
}

impl<W> Drop for FlushGuard<W>
where
    W: Write + 'static,
{
    fn drop(&mut self) {
        match self.flush() {
            Ok(_) => (),
            Err(e) => e.report(),
        }
    }
}

impl<S> FlameLayer<S, BufWriter<File>>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    /// Constructs a `FlameLayer` that outputs to a `BufWriter` to the given path, and a
    /// `FlushGuard` to ensure the writer is flushed.
    pub fn with_file(path: impl AsRef<Path>) -> Result<(Self, FlushGuard<BufWriter<File>>), Error> {
        let path = path.as_ref();
        let file = File::create(path)
            .map_err(|source| Kind::CreateFile {
                path: path.into(),
                source,
            })
            .map_err(Error)?;
        let writer = BufWriter::new(file);
        let layer = Self::new(writer);
        let guard = layer.flush_on_drop();
        Ok((layer, guard))
    }
}

impl<S, W> Layer<S> for FlameLayer<S, W>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: Write + 'static,
{
    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        let samples = self.time_since_last_event();

        let first = ctx.span(id).expect("expected: span id exists in registry");
        let parents = first.from_root();

        let mut stack = String::new();

        THREAD_NAME.with(|name| stack += name.as_str());

        for parent in parents {
            stack += "; ";
            write(&mut stack, parent).expect("expected: write to String never fails");
        }

        write!(&mut stack, " {}", samples.as_nanos())
            .expect("expected: write to String never fails");

        let _ = writeln!(*self.out.lock().unwrap(), "{}", stack);
    }

    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        let panicking = std::thread::panicking();
        macro_rules! expect {
            ($e:expr, $msg:literal) => {
                if panicking {
                    return;
                } else {
                    $e.expect($msg)
                }
            };
            ($e:expr) => {
                if panicking {
                    return;
                } else {
                    $e.unwrap()
                }
            };
        }

        let samples = self.time_since_last_event();
        let first = expect!(ctx.span(&id), "expected: span id exists in registry");
        let parents = first.from_root();

        let mut stack = String::new();
        THREAD_NAME.with(|name| stack += name.as_str());
        stack += "; ";

        for parent in parents {
            expect!(
                write(&mut stack, parent),
                "expected: write to String never fails"
            );
            stack += "; ";
        }

        expect!(
            write(&mut stack, first),
            "expected: write to String never fails"
        );
        expect!(
            write!(&mut stack, " {}", samples.as_nanos()),
            "expected: write to String never fails"
        );

        let _ = writeln!(*expect!(self.out.lock()), "{}", stack);
    }
}

impl<S, W> FlameLayer<S, W>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: Write + 'static,
{
    fn time_since_last_event(&self) -> Duration {
        let now = Instant::now();

        let prev = LAST_EVENT.with(|e| {
            let prev = e.get();
            e.set(now);
            prev
        });

        now - prev
    }
}

fn write<S>(dest: &mut String, span: SpanRef<'_, S>) -> fmt::Result
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    if let Some(module_path) = span.metadata().module_path() {
        write!(dest, "{}::", module_path)?;
    }

    write!(dest, "{}", span.name())?;

    if let Some(file) = span.metadata().file() {
        write!(dest, ":{}", file)?;
    }

    if let Some(line) = span.metadata().line() {
        write!(dest, ":{}", line)?;
    }

    Ok(())
}
