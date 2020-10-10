//! A scoped, structured logging and diagnostics system.
//!
//! # Overview
//!
//! `tracing` is a framework for instrumenting Rust programs to collect
//! structured, event-based diagnostic information.
//!
//! In asynchronous systems like Tokio, interpreting traditional log messages can
//! often be quite challenging. Since individual tasks are multiplexed on the same
//! thread, associated events and log lines are intermixed making it difficult to
//! trace the logic flow. `tracing` expands upon logging-style diagnostics by
//! allowing libraries and applications to record structured events with additional
//! information about *temporality* and *causality* — unlike a log message, a span
//! in `tracing` has a beginning and end time, may be entered and exited by the
//! flow of execution, and may exist within a nested tree of similar spans. In
//! addition, `tracing` spans are *structured*, with the ability to record typed
//! data as well as textual messages.
//!
//! The `tracing` crate provides the APIs necessary for instrumenting libraries
//! and applications to emit trace data.
//!
//! *Compiler support: [requires `rustc` 1.42+][msrv]*
//!
//! [msrv]: #supported-rust-versions
//! # Core Concepts
//!
//! The core of `tracing`'s API is composed of _spans_, _events_ and
//! _collectors_. We'll cover these in turn.
//!
//! ## Spans
//!
//! To record the flow of execution through a program, `tracing` introduces the
//! concept of [spans]. Unlike a log line that represents a _moment in
//! time_, a span represents a _period of time_ with a beginning and an end. When a
//! program begins executing in a context or performing a unit of work, it
//! _enters_ that context's span, and when it stops executing in that context,
//! it _exits_ the span. The span in which a thread is currently executing is
//! referred to as that thread's _current_ span.
//!
//! For example:
//! ```
//! use tracing::{span, Level};
//! # fn main() {
//! let span = span!(Level::TRACE, "my_span");
//! // `enter` returns a RAII guard which, when dropped, exits the span. this
//! // indicates that we are in the span for the current lexical scope.
//! let _enter = span.enter();
//! // perform some work in the context of `my_span`...
//! # }
//!```
//!
//! The [`span` module][span]'s documentation provides further details on how to
//! use spans.
//!
//! <div class="information">
//!     <div class="tooltip compile_fail" style="">&#x26a0; &#xfe0f;<span class="tooltiptext">Warning</span></div>
//! </div><div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//!     <strong>Warning</strong>: In asynchronous code that uses async/await syntax,
//!     <code>Span::enter</code> may produce incorrect traces if the returned drop
//!     guard is held across an await point. See
//!     <a href="span/struct.Span.html#in-asynchronous-code">the method documentation</a>
//!     for details.
//! </pre></div>
//!
//! ## Events
//!
//! An [`Event`] represents a _moment_ in time. It signifies something that
//! happened while a trace was being recorded. `Event`s are comparable to the log
//! records emitted by unstructured logging code, but unlike a typical log line,
//! an `Event` may occur within the context of a span.
//!
//! For example:
//! ```
//! use tracing::{event, span, Level};
//!
//! # fn main() {
//! // records an event outside of any span context:
//! event!(Level::INFO, "something happened");
//!
//! let span = span!(Level::INFO, "my_span");
//! let _guard = span.enter();
//!
//! // records an event within "my_span".
//! event!(Level::DEBUG, "something happened inside my_span");
//! # }
//!```
//!
//! In general, events should be used to represent points in time _within_ a
//! span — a request returned with a given status code, _n_ new items were
//! taken from a queue, and so on.
//!
//! The [`Event` struct][`Event`] documentation provides further details on using
//! events.
//!
//! ## Collectors
//!
//! As `Span`s and `Event`s occur, they are recorded or aggregated by
//! implementations of the [`Collector`] trait. `Collector`s are notified
//! when an `Event` takes place and when a `Span` is entered or exited. These
//! notifications are represented by the following `Collector` trait methods:
//!
//! + [`event`][Collector::event], called when an `Event` takes place,
//! + [`enter`], called when execution enters a `Span`,
//! + [`exit`], called when execution exits a `Span`
//!
//! In addition, collectors may implement the [`enabled`] function to _filter_
//! the notifications they receive based on [metadata] describing each `Span`
//! or `Event`. If a call to `Collector::enabled` returns `false` for a given
//! set of metadata, that `Collector` will *not* be notified about the
//! corresponding `Span` or `Event`. For performance reasons, if no currently
//! active collectors express interest in a given set of metadata by returning
//! `true`, then the corresponding `Span` or `Event` will never be constructed.
//!
//! # Usage
//!
//! First, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tracing = "0.1"
//! ```
//!
//! *Compiler support: requires rustc 1.39+*
//!
//! ## Recording Spans and Events
//!
//! Spans and events are recorded using macros.
//!
//! ### Spans
//!
//! The [`span!`] macro expands to a [`Span` struct][`Span`] which is used to
//! record a span. The [`Span::enter`] method on that struct records that the
//! span has been entered, and returns a [RAII] guard object, which will exit
//! the span when dropped.
//!
//! For example:
//!
//! ```rust
//! use tracing::{span, Level};
//! # fn main() {
//! // Construct a new span named "my span" with trace log level.
//! let span = span!(Level::TRACE, "my span");
//!
//! // Enter the span, returning a guard object.
//! let _enter = span.enter();
//!
//! // Any trace events that occur before the guard is dropped will occur
//! // within the span.
//!
//! // Dropping the guard will exit the span.
//! # }
//! ```
//!
//! The [`#[instrument]`][instrument] attribute provides an easy way to
//! add `tracing` spans to functions. A function annotated with `#[instrument]`
//! will create and enter a span with that function's name every time the
//! function is called, with arguments to that function will be recorded as
//! fields using `fmt::Debug`.
//!
//! For example:
//! ```ignore
//! # // this doctest is ignored because we don't have a way to say
//! # // that it should only be run with cfg(feature = "attributes")
//! use tracing::{Level, event, instrument};
//!
//! #[instrument]
//! pub fn my_function(my_arg: usize) {
//!     // This event will be recorded inside a span named `my_function` with the
//!     // field `my_arg`.
//!     event!(Level::INFO, "inside my_function!");
//!     // ...
//! }
//! # fn main() {}
//! ```
//!
//!
//! You can find more examples showing how to use this crate [here][examples].
//!
//! [RAII]: https://github.com/rust-unofficial/patterns/blob/master/patterns/RAII.md
//! [examples]: https://github.com/tokio-rs/tracing/tree/master/examples
//!
//! ### Events
//!
//! [`Event`]s are recorded using the [`event!`] macro:
//!
//! ```rust
//! # fn main() {
//! use tracing::{event, Level};
//! event!(Level::INFO, "something has happened!");
//! # }
//! ```
//!
//! ## Using the Macros
//!
//! The [`span!`] and [`event!`] macros use fairly similar syntax, with some
//! exceptions.
//!
//! ### Configuring Attributes
//!
//! Both macros require a [`Level`] specifying the verbosity of the span or
//! event. Optionally, the [target] and [parent span] may be overridden. If the
//! target and parent span are not overridden, they will default to the
//! module path where the macro was invoked and the current span (as determined
//! by the collector), respectively.
//!
//! For example:
//!
//! ```
//! # use tracing::{span, event, Level};
//! # fn main() {
//! span!(target: "app_spans", Level::TRACE, "my span");
//! event!(target: "app_events", Level::INFO, "something has happened!");
//! # }
//! ```
//! ```
//! # use tracing::{span, event, Level};
//! # fn main() {
//! let span = span!(Level::TRACE, "my span");
//! event!(parent: &span, Level::INFO, "something has happened!");
//! # }
//! ```
//!
//! The span macros also take a string literal after the level, to set the name
//! of the span.
//!
//! ### Recording Fields
//!
//! Structured fields on spans and events are specified using the syntax
//! `field_name = field_value`. Fields are separated by commas.
//!
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! // records an event with two fields:
//! //  - "answer", with the value 42
//! //  - "question", with the value "life, the universe and everything"
//! event!(Level::INFO, answer = 42, question = "life, the universe, and everything");
//! # }
//! ```
//!
//! As shorthand, local variables may be used as field values without an
//! assignment, similar to [struct initializers]. For example:
//!
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! let user = "ferris";
//!
//! span!(Level::TRACE, "login", user);
//! // is equivalent to:
//! span!(Level::TRACE, "login", user = user);
//! # }
//!```
//!
//! Field names can include dots, but should not be terminated by them:
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! let user = "ferris";
//! let email = "ferris@rust-lang.org";
//! span!(Level::TRACE, "login", user, user.email = email);
//! # }
//!```
//!
//! Since field names can include dots, fields on local structs can be used
//! using the local variable shorthand:
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! # struct User {
//! #    name: &'static str,
//! #    email: &'static str,
//! # }
//! let user = User {
//!     name: "ferris",
//!     email: "ferris@rust-lang.org",
//! };
//! // the span will have the fields `user.name = "ferris"` and
//! // `user.email = "ferris@rust-lang.org"`.
//! span!(Level::TRACE, "login", user.name, user.email);
//! # }
//!```
//!
//! Fields with names that are not Rust identifiers, or with names that are Rust reserved words,
//! may be created using quoted string literals. However, this may not be used with the local
//! variable shorthand.
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! // records an event with fields whose names are not Rust identifiers
//! //  - "guid:x-request-id", containing a `:`, with the value "abcdef"
//! //  - "type", which is a reserved word, with the value "request"
//! span!(Level::TRACE, "api", "guid:x-request-id" = "abcdef", "type" = "request");
//! # }
//!```
//!
//! The `?` sigil is shorthand that specifies a field should be recorded using
//! its [`fmt::Debug`] implementation:
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! #[derive(Debug)]
//! struct MyStruct {
//!     field: &'static str,
//! }
//!
//! let my_struct = MyStruct {
//!     field: "Hello world!"
//! };
//!
//! // `my_struct` will be recorded using its `fmt::Debug` implementation.
//! event!(Level::TRACE, greeting = ?my_struct);
//! // is equivalent to:
//! event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));
//! # }
//! ```
//!
//! The `%` sigil operates similarly, but indicates that the value should be
//! recorded using its [`fmt::Display`] implementation:
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! # #[derive(Debug)]
//! # struct MyStruct {
//! #     field: &'static str,
//! # }
//! #
//! # let my_struct = MyStruct {
//! #     field: "Hello world!"
//! # };
//! // `my_struct.field` will be recorded using its `fmt::Display` implementation.
//! event!(Level::TRACE, greeting = %my_struct.field);
//! // is equivalent to:
//! event!(Level::TRACE, greeting = tracing::field::display(&my_struct.field));
//! # }
//! ```
//!
//! The `%` and `?` sigils may also be used with local variable shorthand:
//!
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! # #[derive(Debug)]
//! # struct MyStruct {
//! #     field: &'static str,
//! # }
//! #
//! # let my_struct = MyStruct {
//! #     field: "Hello world!"
//! # };
//! // `my_struct.field` will be recorded using its `fmt::Display` implementation.
//! event!(Level::TRACE, %my_struct.field);
//! # }
//! ```
//!
//! Additionally, a span may declare fields with the special value [`Empty`],
//! which indicates that that the value for that field does not currently exist
//! but may be recorded later. For example:
//!
//! ```
//! use tracing::{trace_span, field};
//!
//! // Create a span with two fields: `greeting`, with the value "hello world", and
//! // `parting`, without a value.
//! let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);
//!
//! // ...
//!
//! // Now, record a value for parting as well.
//! span.record("parting", &"goodbye world!");
//! ```
//!
//! Note that a span may have up to 32 fields. The following will not compile:
//!
//! ```rust,compile_fail
//! # use tracing::Level;
//! # fn main() {
//! let bad_span = span!(
//!     Level::TRACE,
//!     "too many fields!",
//!     a = 1, b = 2, c = 3, d = 4, e = 5, f = 6, g = 7, h = 8, i = 9,
//!     j = 10, k = 11, l = 12, m = 13, n = 14, o = 15, p = 16, q = 17,
//!     r = 18, s = 19, t = 20, u = 21, v = 22, w = 23, x = 24, y = 25,
//!     z = 26, aa = 27, bb = 28, cc = 29, dd = 30, ee = 31, ff = 32, gg = 33
//! );
//! # }
//! ```
//!
//! Finally, events may also include human-readable messages, in the form of a
//! [format string][fmt] and (optional) arguments, **after** the event's
//! key-value fields. If a format string and arguments are provided,
//! they will implicitly create a new field named `message` whose value is the
//! provided set of format arguments.
//!
//! For example:
//!
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! let question = "the answer to the ultimate question of life, the universe, and everything";
//! let answer = 42;
//! // records an event with the following fields:
//! // - `question.answer` with the value 42,
//! // - `question.tricky` with the value `true`,
//! // - "message", with the value "the answer to the ultimate question of life, the
//! //    universe, and everything is 42."
//! event!(
//!     Level::DEBUG,
//!     question.answer = answer,
//!     question.tricky = true,
//!     "the answer to {} is {}.", question, answer
//! );
//! # }
//! ```
//!
//! Specifying a formatted message in this manner does not allocate by default.
//!
//! [struct initializers]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name
//! [target]: struct.Metadata.html#method.target
//! [parent span]: span/struct.Attributes.html#method.parent
//! [determined contextually]: span/struct.Attributes.html#method.is_contextual
//! [`fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
//! [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [fmt]: https://doc.rust-lang.org/std/fmt/#usage
//! [`Empty`]: field/struct.Empty.html
//!
//! ### Shorthand Macros
//!
//! `tracing` also offers a number of macros with preset verbosity levels.
//! The [`trace!`], [`debug!`], [`info!`], [`warn!`], and [`error!`] behave
//! similarly to the [`event!`] macro, but with the [`Level`] argument already
//! specified, while the corresponding [`trace_span!`], [`debug_span!`],
//! [`info_span!`], [`warn_span!`], and [`error_span!`] macros are the same,
//! but for the [`span!`] macro.
//!
//! These are intended both as a shorthand, and for compatibility with the [`log`]
//! crate (see the next section).
//!
//! [`span!`]: macro.span.html
//! [`event!`]: macro.event.html
//! [`trace!`]: macro.trace.html
//! [`debug!`]: macro.debug.html
//! [`info!`]: macro.info.html
//! [`warn!`]: macro.warn.html
//! [`error!`]: macro.error.html
//! [`trace_span!`]: macro.trace_span.html
//! [`debug_span!`]: macro.debug_span.html
//! [`info_span!`]: macro.info_span.html
//! [`warn_span!`]: macro.warn_span.html
//! [`error_span!`]: macro.error_span.html
//! [`Level`]: struct.Level.html
//!
//! ### For `log` Users
//!
//! Users of the [`log`] crate should note that `tracing` exposes a set of
//! macros for creating `Event`s (`trace!`, `debug!`, `info!`, `warn!`, and
//! `error!`) which may be invoked with the same syntax as the similarly-named
//! macros from the `log` crate. Often, the process of converting a project to
//! use `tracing` can begin with a simple drop-in replacement.
//!
//! Let's consider the `log` crate's yak-shaving example:
//!
//! ```rust,ignore
//! use std::{error::Error, io};
//! use tracing::{debug, error, info, span, warn, Level};
//!
//! // the `#[tracing::instrument]` attribute creates and enters a span
//! // every time the instrumented function is called. The span is named after the
//! // the function or method. Parameters passed to the function are recorded as fields.
//! #[tracing::instrument]
//! pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
//!     // this creates an event at the DEBUG level with two fields:
//!     // - `excitement`, with the key "excitement" and the value "yay!"
//!     // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
//!     //
//!     // unlike other fields, `message`'s shorthand initialization is just the string itself.
//!     debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
//!     if yak == 3 {
//!         warn!("could not locate yak!");
//!         // note that this is intended to demonstrate `tracing`'s features, not idiomatic
//!         // error handling! in a library or application, you should consider returning
//!         // a dedicated `YakError`. libraries like snafu or thiserror make this easy.
//!         return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
//!     } else {
//!         debug!("yak shaved successfully");
//!     }
//!     Ok(())
//! }
//!
//! pub fn shave_all(yaks: usize) -> usize {
//!     // Constructs a new span named "shaving_yaks" at the TRACE level,
//!     // and a field whose key is "yaks". This is equivalent to writing:
//!     //
//!     // let span = span!(Level::TRACE, "shaving_yaks", yaks = yaks);
//!     //
//!     // local variables (`yaks`) can be used as field values
//!     // without an assignment, similar to struct initializers.
//!     let span = span!(Level::TRACE, "shaving_yaks", yaks);
//!     let _enter = span.enter();
//!
//!     info!("shaving yaks");
//!
//!     let mut yaks_shaved = 0;
//!     for yak in 1..=yaks {
//!         let res = shave(yak);
//!         debug!(yak, shaved = res.is_ok());
//!
//!         if let Err(ref error) = res {
//!             // Like spans, events can also use the field initialization shorthand.
//!             // In this instance, `yak` is the field being initalized.
//!             error!(yak, error = error.as_ref(), "failed to shave yak!");
//!         } else {
//!             yaks_shaved += 1;
//!         }
//!         debug!(yaks_shaved);
//!     }
//!
//!     yaks_shaved
//! }
//! ```
//!
//! ## In libraries
//!
//! Libraries should link only to the `tracing` crate, and use the provided
//! macros to record whatever information will be useful to downstream
//! consumers.
//!
//! ## In executables
//!
//! In order to record trace events, executables have to use a `Collector`
//! implementation compatible with `tracing`. A `Collector` implements a
//! way of collecting trace data, such as by logging it to standard output.
//!
//! This library does not contain any `Collector` implementations; these are
//! provided by [other crates](#related-crates).
//!
//! The simplest way to use a collector is to call the [`set_global_default`]
//! function:
//!
//! ```
//! # pub struct FooCollector;
//! # use tracing::{span::{Id, Attributes, Record}, Metadata};
//! # impl tracing::Collector for FooCollector {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &tracing::Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! # }
//! # impl FooCollector {
//! #   fn new() -> Self { FooCollector }
//! # }
//! # fn main() {
//!
//! # #[cfg(feature = "alloc")]
//! let my_collector = FooCollector::new();
//! # #[cfg(feature = "alloc")]
//! tracing::collector::set_global_default(my_collector)
//!     .expect("setting tracing default failed");
//! # }
//! ```
//!
//! <div class="information">
//!     <div class="tooltip compile_fail" style="">&#x26a0; &#xfe0f;<span class="tooltiptext">Warning</span></div>
//! </div><div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//! <strong>Warning</strong>: In general, libraries should <em>not</em> call
//! <code>set_global_default()</code>! Doing so will cause conflicts when
//! executables that depend on the library try to set the default later.
//! </pre></div>
//!
//! This collector will be used as the default in all threads for the
//! remainder of the duration of the program, similar to setting the logger
//! in the `log` crate.
//!
//! In addition, the default collector can be set through using the
//! [`with_default`] function. This follows the `tokio` pattern of using
//! closures to represent executing code in a context that is exited at the end
//! of the closure. For example:
//!
//! ```rust
//! # pub struct FooCollector;
//! # use tracing::{span::{Id, Attributes, Record}, Metadata};
//! # impl tracing::Collector for FooCollector {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &tracing::Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! # }
//! # impl FooCollector {
//! #   fn new() -> Self { FooCollector }
//! # }
//! # fn main() {
//!
//! let my_collector = FooCollector::new();
//! # #[cfg(feature = "std")]
//! tracing::collector::with_default(my_collector, || {
//!     // Any trace events generated in this closure or by functions it calls
//!     // will be collected by `my_collector`.
//! })
//! # }
//! ```
//!
//! This approach allows trace data to be collected by multiple collectors
//! within different contexts in the program. Note that the override only applies to the
//! currently executing thread; other threads will not see the change from with_default.
//!
//! Any trace events generated outside the context of a collector will not be collected.
//!
//! Once a collector has been set, instrumentation points may be added to the
//! executable using the `tracing` crate's macros.
//!
//! ## `log` Compatibility
//!
//! The [`log`] crate provides a simple, lightweight logging facade for Rust.
//! While `tracing` builds upon `log`'s foundation with richer structured
//! diagnostic data, `log`'s simplicity and ubiquity make it the "lowest common
//! denominator" for text-based logging in Rust — a vast majority of Rust
//! libraries and applications either emit or consume `log` records. Therefore,
//! `tracing` provides multiple forms of interoperability with `log`: `tracing`
//! instrumentation can emit `log` records, and a compatibility layer enables
//! `tracing` [`Collector`]s to consume `log` records as `tracing` [`Event`]s.
//!
//! ### Emitting `log` Records
//!
//! This crate provides two feature flags, "log" and "log-always", which will
//! cause [spans] and [events] to emit `log` records. When the "log" feature is
//! enabled, if no `tracing` `Collector` is active, invoking an event macro or
//! creating a span with fields will emit a `log` record. This is intended
//! primarily for use in libraries which wish to emit diagnostics that can be
//! consumed by applications using `tracing` *or* `log`, without paying the
//! additional overhead of emitting both forms of diagnostics when `tracing` is
//! in use.
//!
//! Enabling the "log-always" feature will cause `log` records to be emitted
//! even if a `tracing` `Collector` _is_ set. This is intended to be used in
//! applications where a `log` `Logger` is being used to record a textual log,
//! and `tracing` is used only to record other forms of diagnostics (such as
//! metrics, profiling, or distributed tracing data). Unlike the "log" feature,
//! libraries generally should **not** enable the "log-always" feature, as doing
//! so will prevent applications from being able to opt out of the `log` records.
//!
//! See [here][flags] for more details on this crate's feature flags.
//!
//! The generated `log` records' messages will be a string representation of the
//! span or event's fields, and all additional information recorded by `log`
//! (target, verbosity level, module path, file, and line number) will also be
//! populated. Additionally, `log` records are also generated when spans are
//! entered, exited, and closed. Since these additional span lifecycle logs have
//! the potential to be very verbose, and don't include additional fields, they
//! will always be emitted at the `Trace` level, rather than inheriting the
//! level of the span that generated them. Furthermore, they are are categorized
//! under a separate `log` target, "tracing::span" (and its sub-target,
//! "tracing::span::active", for the logs on entering and exiting a span), which
//! may be enabled or disabled separately from other `log` records emitted by
//! `tracing`.
//!
//! ### Consuming `log` Records
//!
//! The [`tracing-log`] crate provides a compatibility layer which
//! allows a `tracing` [`Collector`] to consume `log` records as though they
//! were `tracing` [events]. This allows applications using `tracing` to record
//! the logs emitted by dependencies using `log` as events within the context of
//! the application's trace tree. See [that crate's documentation][log-tracer]
//! for details.
//!
//! [log-tracer]: https://docs.rs/tracing-log/latest/tracing_log/#convert-log-records-to-tracing-events
//!
//! ### `no_std` Support
//!
//! In embedded systems and other bare-metal applications, `tracing` can be
//! used without requiring the Rust standard library, although some features are
//! disabled.
//!
//! The dependency on the standard library is controlled by two crate feature
//! flags, "std", which enables the dependency on [`libstd`], and "alloc", which
//! enables the dependency on [`liballoc`] (and is enabled by the "std"
//! feature). These features are enabled by default, but `no_std` users can
//! disable them using:
//!
//! ```toml
//! # Cargo.toml
//! tracing = { version = "0.2", default-features = false }
//! ```
//!
//! To enable `liballoc` but not `std`, use:
//!
//! ```toml
//! # Cargo.toml
//! tracing = { version = "0.2", default-features = false, features = ["alloc"] }
//! ```
//!
//! When both the "std" and "alloc" feature flags are disabled, `tracing-core`
//! will not make any dynamic memory allocations at runtime, and does not
//! require a global memory allocator.
//!
//! The "alloc" feature is required to enable the [`Dispatch::new`] function,
//! which requires dynamic memory allocation to construct a `Collector` trait
//! object at runtime. When liballoc is disabled, new `Dispatch`s may still be
//! created from `&'static dyn Collector` references, using
//! [`Dispatch::from_static`].
//!
//! The "std" feature is required to enable the following features:
//!
//! * Per-thread scoped trace dispatchers ([`Dispatch::set_default`] and
//!   [`with_default`]. Since setting a thread-local dispatcher inherently
//!   requires a concept of threads to be available, this API is not possible
//!   without the standard library.
//! * Support for [constructing `Value`s from types implementing
//!   `std::error::Error`][err]. Since the `Error` trait is defined in `std`,
//!   it's not possible to provide this feature without `std`.
//!
//! All other features of `tracing` should behave identically with and
//! without `std` and `alloc`.
//!
//! [`libstd`]: https://doc.rust-lang.org/std/index.html
//! [`Dispatch::new`]: crate::dispatcher::Dispatch::new
//! [`Dispatch::from_static`]: crate::dispatcher::Dispatch::from_static
//! [`Dispatch::set_default`]: crate::dispatcher::set_default
//! [`with_default`]: crate::dispatcher::with_default
//! [err]: crate::field::Visit::record_error
//!
//! ## Related Crates
//!
//! In addition to `tracing` and `tracing-core`, the [`tokio-rs/tracing`] repository
//! contains several additional crates designed to be used with the `tracing` ecosystem.
//! This includes a collection of `Collector` implementations, as well as utility
//! and adapter crates to assist in writing `Collector`s and instrumenting
//! applications.
//!
//! In particular, the following crates are likely to be of interest:
//!
//!  - [`tracing-futures`] provides a compatibility layer with the `futures`
//!    crate, allowing spans to be attached to `Future`s, `Stream`s, and `Executor`s.
//!  - [`tracing-subscriber`] provides `Collector` implementations and
//!    utilities for working with `Collector`s. This includes a [`FmtSubscriber`]
//!    `FmtSubscriber` for logging formatted trace data to stdout, with similar
//!    filtering and formatting to the [`env_logger`] crate.
//!  - [`tracing-log`] provides a compatibility layer with the [`log`] crate,
//!    allowing log messages to be recorded as `tracing` `Event`s within the
//!    trace tree. This is useful when a project using `tracing` have
//!    dependencies which use `log`. Note that if you're using
//!    `tracing-subscriber`'s `FmtSubscriber`, you don't need to depend on
//!    `tracing-log` directly.
//!  - [`tracing-appender`] provides utilities for outputting tracing data,
//!     including a file appender and non blocking writer.
//!
//! Additionally, there are also several third-party crates which are not
//! maintained by the `tokio` project. These include:
//!
//!  - [`tracing-timing`] implements inter-event timing metrics on top of `tracing`.
//!    It provides a subscriber that records the time elapsed between pairs of
//!    `tracing` events and generates histograms.
//!  - [`tracing-opentelemetry`] provides a subscriber for emitting traces to
//!    [OpenTelemetry]-compatible distributed tracing systems.
//!  - [`tracing-honeycomb`] Provides a layer that reports traces spanning multiple machines to [honeycomb.io]. Backed by [`tracing-distributed`].
//!  - [`tracing-distributed`] Provides a generic implementation of a layer that reports traces spanning multiple machines to some backend.
//!  - [`tracing-actix`] provides `tracing` integration for the `actix` actor
//!    framework.
//!  - [`tracing-gelf`] implements a subscriber for exporting traces in Greylog
//!    GELF format.
//!  - [`tracing-coz`] provides integration with the [coz] causal profiler
//!    (Linux-only).
//!  - [`tracing-bunyan-formatter`] provides a layer implementation that reports events and spans
//!    in [bunyan] format, enriched with timing information.
//!  - [`tracing-wasm`] provides a `Collector`/`Subscriber` implementation that reports
//!    events and spans via browser `console.log` and [User Timing API (`window.performance`)].
//!  - [`tide-tracing`] provides a [tide] middleware to trace all incoming requests and responses.
//!  - [`test-env-log`] takes care of initializing `tracing` for tests, based on
//!    environment variables with an `env_logger` compatible syntax.
//!  - [`tracing-unwrap`] provides convenience methods to report failed unwraps
//!    on `Result` or `Option` types to a `Collector`.
//!  - [`diesel-tracing`] provides integration with [`diesel`] database connections.
//!  - [`tracing-tracy`] provides a way to collect [Tracy] profiles in instrumented
//!    applications.
//!
//! If you're the maintainer of a `tracing` ecosystem crate not listed above,
//! please let us know! We'd love to add your project to the list!
//!
//! [`tracing-opentelemetry`]: https://crates.io/crates/tracing-opentelemetry
//! [OpenTelemetry]: https://opentelemetry.io/
//! [`tracing-honeycomb`]: https://crates.io/crates/tracing-honeycomb
//! [`tracing-distributed`]: https://crates.io/crates/tracing-distributed
//! [honeycomb.io]: https://www.honeycomb.io/
//! [`tracing-actix`]: https://crates.io/crates/tracing-actix
//! [`tracing-gelf`]: https://crates.io/crates/tracing-gelf
//! [`tracing-coz`]: https://crates.io/crates/tracing-coz
//! [coz]: https://github.com/plasma-umass/coz
//! [`tracing-bunyan-formatter`]: https://crates.io/crates/tracing-bunyan-formatter
//! [bunyan]: https://github.com/trentm/node-bunyan
//! [`tracing-wasm`]: https://docs.rs/tracing-wasm
//! [User Timing API (`window.performance`)]: https://developer.mozilla.org/en-US/docs/Web/API/User_Timing_API
//! [`tide-tracing`]: https://crates.io/crates/tide-tracing
//! [tide]: https://crates.io/crates/tide
//! [`test-env-log`]: https://crates.io/crates/test-env-log
//! [`tracing-unwrap`]: https://docs.rs/tracing-unwrap
//! [`diesel`]: https://crates.io/crates/diesel
//! [`diesel-tracing`]: https://crates.io/crates/diesel-tracing
//! [`tracing-tracy`]: https://crates.io/crates/tracing-tracy
//! [Tracy]: https://github.com/wolfpld/tracy
//!
//! <div class="information">
//!     <div class="tooltip ignore" style="">ⓘ<span class="tooltiptext">Note</span></div>
//! </div>
//! <div class="example-wrap" style="display:inline-block">
//! <pre class="ignore" style="white-space:normal;font:inherit;">
//! <strong>Note</strong>: Some of these ecosystem crates are currently
//! unreleased and/or in earlier stages of development. They may be less stable
//! than <code>tracing</code> and <code>tracing-core</code>.
//! </pre></div>
//!
//! ## Crate Feature Flags
//!
//! The following crate feature flags are available:
//!
//! * A set of features controlling the [static verbosity level].
//! * `log`: causes trace instrumentation points to emit [`log`] records as well
//!   as trace events, if a default `tracing` collector has not been set. This
//!   is intended for use in libraries whose users may be using either `tracing`
//!   or `log`.
//! * `log-always`: Emit `log` records from all `tracing` spans and events, even
//!   if a `tracing` collector has been set. This should be set only by
//!   applications which intend to collect traces and logs separately; if an
//!   adapter is used to convert `log` records into `tracing` events, this will
//!   cause duplicate events to occur.
//! * `attributes`: Includes support for the `#[instrument]` attribute.
//!   This is on by default, but does bring in the `syn` crate as a dependency,
//!   which may add to the compile time of crates that do not already use it.
//! * `std`: Depend on the Rust standard library (enabled by default).
//! * `alloc`: Depend on [`liballoc`] (enabled by "std").
//!
//! [`liballoc`]: https://doc.rust-lang.org/alloc/index.html
//! ## Supported Rust Versions
//!
//! Tracing is built against the latest stable release. The minimum supported
//! version is 1.42. The current Tracing version is not guaranteed to build on
//! Rust versions earlier than the minimum supported version.
//!
//! Tracing follows the same compiler support policies as the rest of the Tokio
//! project. The current stable Rust compiler and the three most recent minor
//! versions before it will always be supported. For example, if the current
//! stable compiler version is 1.45, the minimum supported version will not be
//! increased past 1.42, three minor versions prior. Increasing the minimum
//! supported compiler version is not considered a semver breaking change as
//! long as doing so complies with this policy.
//!
//! [`log`]: https://docs.rs/log/0.4.6/log/
//! [span]: mod@span
//! [spans]: mod@span
//! [`Span`]: span::Span
//! [`in_scope`]: span::Span::in_scope
//! [event]: Event
//! [events]: Event
//! [`Collector`]: collector::Collector
//! [Collector::event]: collector::Collector::event
//! [`enter`]: collector::Collector::enter
//! [`exit`]: collector::Collector::exit
//! [`enabled`]: collector::Collector::enabled
//! [metadata]: Metadata
//! [`field::display`]: field::display
//! [`field::debug`]: field::debug
//! [`set_global_default`]: collector::set_global_default
//! [`with_default`]: collector::with_default
//! [`tokio-rs/tracing`]: https://github.com/tokio-rs/tracing
//! [`tracing-futures`]: https://crates.io/crates/tracing-futures
//! [`tracing-subscriber`]: https://crates.io/crates/tracing-subscriber
//! [`tracing-log`]: https://crates.io/crates/tracing-log
//! [`tracing-timing`]: https://crates.io/crates/tracing-timing
//! [`tracing-appender`]: https://crates.io/crates/tracing-appender
//! [`env_logger`]: https://crates.io/crates/env_logger
//! [`FmtSubscriber`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Subscriber.html
//! [static verbosity level]: level_filters/index.html#compile-time-filters
//! [instrument]: https://docs.rs/tracing-attributes/latest/tracing_attributes/attr.instrument.html
//! [flags]: #crate-feature-flags
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg), deny(broken_intra_doc_links))]
#![doc(html_root_url = "https://docs.rs/tracing/0.1.21")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo-type.png",
    issue_tracker_base_url = "https://github.com/tokio-rs/tracing/issues/"
)]
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

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "log")]
#[doc(hidden)]
pub use log;

// Somehow this `use` statement is necessary for us to re-export the `core`
// macros on Rust 1.26.0. I'm not sure how this makes it work, but it does.
#[allow(unused_imports)]
#[doc(hidden)]
use tracing_core::*;

#[doc(inline)]
pub use self::instrument::Instrument;
pub use self::{collector::Collector, dispatcher::Dispatch, event::Event, field::Value};

#[doc(hidden)]
pub use self::span::Id;

#[doc(hidden)]
pub use tracing_core::{
    callsite::{self, Callsite},
    metadata,
};
pub use tracing_core::{event, Level, Metadata};

#[doc(inline)]
pub use self::span::Span;
#[cfg(feature = "attributes")]
#[cfg_attr(docsrs, doc(cfg(feature = "attributes")))]
#[doc(inline)]
pub use tracing_attributes::instrument;

#[macro_use]
mod macros;

pub mod collector;
pub mod dispatcher;
pub mod field;
/// Attach a span to a `std::future::Future`.
pub mod instrument;
pub mod level_filters;
pub mod span;

#[doc(hidden)]
pub mod __macro_support {
    pub use crate::callsite::{Callsite, Registration};
    use crate::{collector::Interest, Metadata};
    use core::fmt;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use tracing_core::Once;

    /// Callsite implementation used by macro-generated code.
    ///
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This type, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    pub struct MacroCallsite<T = &'static dyn Callsite>
    where
        T: 'static,
    {
        interest: AtomicUsize,
        meta: &'static Metadata<'static>,
        register: Once,
        registration: &'static Registration<T>,
    }

    impl<T: 'static> MacroCallsite<T> {
        /// Returns a new `MacroCallsite` with the specified `Metadata`.
        ///
        /// /!\ WARNING: This is *not* a stable API! /!\
        /// This method, and all code contained in the `__macro_support` module, is
        /// a *private* API of `tracing`. It is exposed publicly because it is used
        /// by the `tracing` macros, but it is not part of the stable versioned API.
        /// Breaking changes to this module may occur in small-numbered versions
        /// without warning.
        pub const fn new(
            meta: &'static Metadata<'static>,
            registration: &'static Registration<T>,
        ) -> Self {
            Self {
                interest: AtomicUsize::new(0xDEADFACED),
                meta,
                register: Once::new(),
                registration,
            }
        }
    }

    impl MacroCallsite<&'static dyn Callsite> {
        /// Registers this callsite with the global callsite registry.
        ///
        /// If the callsite is already registered, this does nothing.
        ///
        /// /!\ WARNING: This is *not* a stable API! /!\
        /// This method, and all code contained in the `__macro_support` module, is
        /// a *private* API of `tracing`. It is exposed publicly because it is used
        /// by the `tracing` macros, but it is not part of the stable versioned API.
        /// Breaking changes to this module may occur in small-numbered versions
        /// without warning.
        #[inline(never)]
        // This only happens once (or if the cached interest value was corrupted).
        #[cold]
        pub fn register(&'static self) -> Interest {
            self.register
                .call_once(|| crate::callsite::register(self.registration));
            match self.interest.load(Ordering::Relaxed) {
                0 => Interest::never(),
                2 => Interest::always(),
                _ => Interest::sometimes(),
            }
        }

        /// Returns the callsite's cached Interest, or registers it for the
        /// first time if it has not yet been registered.
        ///
        /// /!\ WARNING: This is *not* a stable API! /!\
        /// This method, and all code contained in the `__macro_support` module, is
        /// a *private* API of `tracing`. It is exposed publicly because it is used
        /// by the `tracing` macros, but it is not part of the stable versioned API.
        /// Breaking changes to this module may occur in small-numbered versions
        /// without warning.
        #[inline]
        pub fn interest(&'static self) -> Interest {
            match self.interest.load(Ordering::Relaxed) {
                0 => Interest::never(),
                1 => Interest::sometimes(),
                2 => Interest::always(),
                _ => self.register(),
            }
        }

        pub fn is_enabled(&self, interest: Interest) -> bool {
            interest.is_always()
                || crate::dispatcher::get_default(|default| default.enabled(self.meta))
        }

        #[inline]
        #[cfg(feature = "log")]
        pub fn disabled_span(&self) -> crate::Span {
            crate::Span::new_disabled(self.meta)
        }

        #[inline]
        #[cfg(not(feature = "log"))]
        pub fn disabled_span(&self) -> crate::Span {
            crate::Span::none()
        }
    }

    impl Callsite for MacroCallsite {
        fn set_interest(&self, interest: Interest) {
            let interest = match () {
                _ if interest.is_never() => 0,
                _ if interest.is_always() => 2,
                _ => 1,
            };
            self.interest.store(interest, Ordering::SeqCst);
        }

        #[inline(always)]
        fn metadata(&self) -> &Metadata<'static> {
            &self.meta
        }
    }

    impl fmt::Debug for MacroCallsite {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MacroCallsite")
                .field("interest", &self.interest)
                .field("meta", &self.meta)
                .field("register", &self.register)
                .field("registration", &self.registration)
                .finish()
        }
    }
}

mod sealed {
    pub trait Sealed {}
}
