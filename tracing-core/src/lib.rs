#![doc(html_root_url = "https://docs.rs/tracing-core/0.1.0")]
#![deny(missing_debug_implementations, missing_docs, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

//! Core primitives for `tracing`.
//!
//! `tracing` is a framework for instrumenting Rust programs to collect
//! structured, event-based diagnostic information. This crate defines the core
//! primitives of `tracing`.
//!
//! This crate provides:
//!
//! * [`Span`] identifies a span within the execution of a program.
//!
//! * [`Event`] represents a single event within a trace.
//!
//! * [`Subscriber`], the trait implemented to collect trace data.
//!
//! * [`Metadata`] and [`Callsite`] provide information describing `Span`s.
//!
//! * [`Field`], [`FieldSet`], [`Value`], and [`ValueSet`] represent the
//!   structured data attached to a `Span`.
//!
//! * [`Dispatch`] allows span events to be dispatched to `Subscriber`s.
//!
//! In addition, it defines the global callsite registry and per-thread current
//! dispatcher which other components of the tracing system rely on.
//!
//! Application authors will typically not use this crate directly. Instead,
//! they will use the `tracing` crate, which provides a much more
//! fully-featured API. However, this crate's API will change very infrequently,
//! so it may be used when dependencies must be very stable.
//!
//! The [`tracing-nursery`] repository contains less stable crates designed to
//! be used with the `tracing` ecosystem. It includes a collection of
//! `Subscriber` implementations, as well as utility and adapter crates.
//!
//! [`Span`]: span/struct.Span.html
//! [`Event`]: event/struct.Event.html
//! [`Subscriber`]: subscriber/trait.Subscriber.html
//! [`Metadata`]: metadata/struct.Metadata.html
//! [`Callsite`]: callsite/trait.Callsite.html
//! [`Field`]: field/struct.Field.html
//! [`FieldSet`]: field/struct.FieldSet.html
//! [`Value`]: field/trait.Value.html
//! [`ValueSet`]: field/struct.ValueSet.html
//! [`Dispatch`]: dispatcher/struct.Dispatch.html
//! [`tracing-nursery`]: https://github.com/tokio-rs/tracing-nursery
#[macro_use]
extern crate lazy_static;

/// Statically constructs an [`Identifier`] for the provided [`Callsite`].
///
/// This may be used in contexts, such as static initializers, where the
/// [`Callsite::id`] function is not currently usable.
///
/// For example:
/// ```rust
/// # #[macro_use]
/// # extern crate tracing_core;
/// use tracing_core::callsite;
/// # use tracing_core::{Metadata, subscriber::Interest};
/// # fn main() {
/// pub struct MyCallsite {
///    // ...
/// }
/// impl callsite::Callsite for MyCallsite {
/// # fn set_interest(&self, _: Interest) { unimplemented!() }
/// # fn metadata(&self) -> &Metadata { unimplemented!() }
///     // ...
/// }
///
/// static CALLSITE: MyCallsite = MyCallsite {
///     // ...
/// };
///
/// static CALLSITE_ID: callsite::Identifier = identify_callsite!(&CALLSITE);
/// # }
/// ```
///
/// [`Identifier`]: callsite/struct.Identifier.html
/// [`Callsite`]: callsite/trait.Callsite.html
/// [`Callsite`]: callsite/trait.Callsite.html#method.id
#[macro_export]
macro_rules! identify_callsite {
    ($callsite:expr) => {
        $crate::callsite::Identifier($callsite)
    };
}

/// Statically constructs new span [metadata].
///
/// /// For example:
/// ```rust
/// # #[macro_use]
/// # extern crate tracing_core;
/// # use tracing_core::{callsite::Callsite, subscriber::Interest};
/// use tracing_core::metadata::{Kind, Level, Metadata};
/// # fn main() {
/// # pub struct MyCallsite { }
/// # impl Callsite for MyCallsite {
/// # fn set_interest(&self, _: Interest) { unimplemented!() }
/// # fn metadata(&self) -> &Metadata { unimplemented!() }
/// # }
/// #
/// static FOO_CALLSITE: MyCallsite = MyCallsite {
///     // ...
/// };
///
/// static FOO_METADATA: Metadata = metadata!{
///     name: "foo",
///     target: module_path!(),
///     level: Level::DEBUG,
///     fields: &["bar", "baz"],
///     callsite: &FOO_CALLSITE,
///     kind: Kind::SPAN,
/// };
/// # }
/// ```
///
/// [metadata]: metadata/struct.Metadata.html
/// [`Metadata::new`]: metadata/struct.Metadata.html#method.new
#[macro_export(local_inner_macros)]
macro_rules! metadata {
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr
    ) => {
        metadata! {
            name: $name,
            target: $target,
            level: $level,
            fields: $fields,
            callsite: $callsite,
            kind: $kind,
        }
    };
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr,
    ) => {
        $crate::metadata::Metadata::new(
            $name,
            $target,
            $level,
            Some(__tracing_core_file!()),
            Some(__tracing_core_line!()),
            Some(__tracing_core_module_path!()),
            $crate::field::FieldSet::new($fields, identify_callsite!($callsite)),
            $kind,
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_core_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_core_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_core_line {
    () => {
        line!()
    };
}

pub mod callsite;
pub mod dispatcher;
pub mod event;
pub mod field;
pub mod metadata;
mod parent;
pub mod span;
pub mod subscriber;

pub use self::{
    callsite::Callsite,
    dispatcher::Dispatch,
    event::Event,
    field::Field,
    metadata::{Kind, Level, Metadata},
    subscriber::{Interest, Subscriber},
};

mod sealed {
    pub trait Sealed {}
}
