//! [`Subscriber`]s that control which spans and events are enabled by the wrapped
//! subscriber.
//!
//! [`Subscriber`]: crate::fmt::Subscriber
#[cfg(feature = "env-filter")]
mod env;
mod field;
mod level;

pub use self::field::{
    matcher::{ExactFieldMatcher, FieldMatcher},
    FieldFilter,
};
pub use self::level::{LevelFilter, ParseError as LevelParseError};

#[cfg(feature = "env-filter")]
#[cfg_attr(docsrs, doc(cfg(feature = "env-filter")))]
pub use self::env::*;
