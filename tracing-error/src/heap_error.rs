use crate::SpanTrace;
use crate::{ExtractSpanTrace, InstrumentError};
use std::error::Error;
use std::fmt::{self, Debug, Display};

/// A wrapper type for Errors that bundles a `SpanTrace` with an inner `Error` type.
pub struct TracedError {
    inner: ErrorImpl,
}

struct ErrorImpl {
    span_trace: SpanTrace,
    error: Box<dyn Error + Send + Sync + 'static>,
}

impl TracedError {
    fn new<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            inner: ErrorImpl {
                span_trace: SpanTrace::capture(),
                error: Box::new(error),
            },
        }
    }
}

impl Error for TracedError {
    fn source<'a>(&'a self) -> Option<&'a (dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl Debug for TracedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.inner.error, f)
    }
}

impl Display for TracedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner.error, f)
    }
}

impl Error for ErrorImpl {
    fn source<'a>(&'a self) -> Option<&'a (dyn Error + 'static)> {
        self.error.source()
    }
}

impl Debug for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("span backtrace:\n")?;
        Debug::fmt(&self.span_trace, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("span backtrace:\n")?;
        Display::fmt(&self.span_trace, f)
    }
}

impl<E> InstrumentError for E
where
    E: Error + Send + Sync + 'static,
{
    type Instrumented = TracedError;

    fn in_current_span(self) -> Self::Instrumented {
        TracedError::new(self)
    }
}

impl ExtractSpanTrace for &(dyn Error + 'static) {
    fn span_trace(&self) -> Option<&SpanTrace> {
        self.downcast_ref::<ErrorImpl>().map(|e| &e.span_trace)
    }
}
