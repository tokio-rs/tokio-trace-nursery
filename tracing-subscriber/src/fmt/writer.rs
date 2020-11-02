//! Abstractions for creating [`io::Write`] instances.
//!
//! [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

use io::Write;
use std::{fmt::Debug, io};

/// A type that can create [`io::Write`] instances.
///
/// `MakeWriter` is used by [`fmt::Collector`] or [`fmt::Subscriber`] to print formatted text representations of
/// [`Event`]s.
///
/// This trait is already implemented for function pointers and immutably-borrowing closures that
/// return an instance of [`io::Write`], such as [`io::stdout`] and [`io::stderr`].
///
/// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`fmt::Collector`]: super::super::fmt::Collector
/// [`fmt::Subscriber`]: super::super::fmt::Subscriber
/// [`Event`]: tracing_core::event::Event
/// [`io::stdout`]: https://doc.rust-lang.org/std/io/fn.stdout.html
/// [`io::stderr`]: https://doc.rust-lang.org/std/io/fn.stderr.html
pub trait MakeWriter {
    /// The concrete [`io::Write`] implementation returned by [`make_writer`].
    ///
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`make_writer`]: MakeWriter::make_writer
    type Writer: io::Write;

    /// Returns an instance of [`Writer`].
    ///
    /// # Implementer notes
    ///
    /// [`fmt::Subscriber`] or [`fmt::Collector`] will call this method each time an event is recorded. Ensure any state
    /// that must be saved across writes is not lost when the [`Writer`] instance is dropped. If
    /// creating a [`io::Write`] instance is expensive, be sure to cache it when implementing
    /// [`MakeWriter`] to improve performance.
    ///
    /// [`Writer`]: MakeWriter::Writer
    /// [`fmt::Subscriber`]: super::super::fmt::Subscriber
    /// [`fmt::Collector`]: super::super::fmt::Collector
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    fn make_writer(&self) -> Self::Writer;
}

impl<F, W> MakeWriter for F
where
    F: Fn() -> W,
    W: io::Write,
{
    type Writer = W;

    fn make_writer(&self) -> Self::Writer {
        (self)()
    }
}

/// A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.
///
/// `TestWriter` is used by [`fmt::Collector`] or [`fmt::Subscriber`] to enable capturing support.
///
/// `cargo test` can only capture output from the standard library's [`print!`] macro. See
/// [`libtest`'s output capturing][capturing] for more details about output capturing.
///
/// Writing to [`io::stdout`] and [`io::stderr`] produces the same results as using
/// [`libtest`'s `--nocapture` option][nocapture] which may make the results look unreadable.
///
/// [`fmt::Collector`]: super::Collector
/// [`fmt::Subscriber`]: super::Subscriber
/// [capturing]: https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output
/// [nocapture]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
/// [`io::stdout`]: https://doc.rust-lang.org/std/io/fn.stdout.html
/// [`io::stderr`]: https://doc.rust-lang.org/std/io/fn.stderr.html
/// [`print!`]: https://doc.rust-lang.org/std/macro.print.html
#[derive(Default, Debug)]
pub struct TestWriter {
    _p: (),
}

impl TestWriter {
    /// Returns a new `TestWriter` with the default configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let out_str = String::from_utf8_lossy(buf);
        print!("{}", out_str);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl MakeWriter for TestWriter {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        Self::default()
    }
}

/// A writer that erases the specific [`io::Write`] and [`MakeWriter`] types being used.
///
/// This is useful in cases where the concrete type of the writer cannot be known
/// until runtime.
///
/// # Examples
///
/// A function that returns a [`Collect`] that will write to either stdout or stderr:
///
/// ```rust
/// # use tracing::Collect;
/// # use tracing_subscriber::fmt::writer::BoxMakeWriter;
///
/// fn dynamic_writer(use_stderr: bool) -> impl Collect {
///     let writer = if use_stderr {
///         BoxMakeWriter::new(std::io::stderr)
///     } else {
///         BoxMakeWriter::new(std::io::stdout)
///     };
///
///     tracing_subscriber::fmt().with_writer(writer).finish()
/// }
/// ```
///
/// [`Collect`]: tracing::Collect
/// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
pub struct BoxMakeWriter {
    inner: Box<dyn MakeWriter<Writer = Box<dyn Write>> + Send + Sync>,
}

impl BoxMakeWriter {
    /// Constructs a `BoxMakeWriter` wrapping a type implementing [`MakeWriter`].
    ///
    pub fn new<M>(make_writer: M) -> Self
    where
        M: MakeWriter + Send + Sync + 'static,
        M::Writer: Write + 'static,
    {
        Self {
            inner: Box::new(Boxed(make_writer)),
        }
    }
}

impl Debug for BoxMakeWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("BoxMakeWriter { ... }")
    }
}

impl MakeWriter for BoxMakeWriter {
    type Writer = Box<dyn Write>;

    fn make_writer(&self) -> Self::Writer {
        self.inner.make_writer()
    }
}

struct Boxed<M>(M);

impl<M> MakeWriter for Boxed<M>
where
    M: MakeWriter,
    M::Writer: Write + 'static,
{
    type Writer = Box<dyn Write>;

    fn make_writer(&self) -> Self::Writer {
        Box::new(self.0.make_writer())
    }
}

#[cfg(test)]
mod test {
    use super::MakeWriter;
    use crate::fmt::format::Format;
    use crate::fmt::test::{MockMakeWriter, MockWriter};
    use crate::fmt::Collector;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use tracing::error;
    use tracing_core::dispatch::{self, Dispatch};

    fn test_writer<T>(make_writer: T, msg: &str, buf: &Mutex<Vec<u8>>)
    where
        T: MakeWriter + Send + Sync + 'static,
    {
        let subscriber = {
            #[cfg(feature = "ansi")]
            {
                let f = Format::default().without_time().with_ansi(false);
                Collector::builder()
                    .event_format(f)
                    .with_writer(make_writer)
                    .finish()
            }
            #[cfg(not(feature = "ansi"))]
            {
                let f = Format::default().without_time();
                Collector::builder()
                    .event_format(f)
                    .with_writer(make_writer)
                    .finish()
            }
        };
        let dispatch = Dispatch::from(subscriber);

        dispatch::with_default(&dispatch, || {
            error!("{}", msg);
        });

        let expected = format!("ERROR {}: {}\n", module_path!(), msg);
        let actual = String::from_utf8(buf.try_lock().unwrap().to_vec()).unwrap();
        assert!(actual.contains(expected.as_str()));
    }

    #[test]
    fn custom_writer_closure() {
        lazy_static! {
            static ref BUF: Mutex<Vec<u8>> = Mutex::new(vec![]);
        }

        let make_writer = || MockWriter::new(&BUF);
        let msg = "my custom writer closure error";
        test_writer(make_writer, msg, &BUF);
    }

    #[test]
    fn custom_writer_struct() {
        lazy_static! {
            static ref BUF: Mutex<Vec<u8>> = Mutex::new(vec![]);
        }

        let make_writer = MockMakeWriter::new(&BUF);
        let msg = "my custom writer struct error";
        test_writer(make_writer, msg, &BUF);
    }
}
