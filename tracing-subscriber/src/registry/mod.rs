//! Storage for span data shared by multiple [`Subscribe`]s.
//!
//! ## Using the Span Registry
//!
//! This module provides the [`Registry`] type, a [`Collect`] implementation
//! which tracks per-span data and exposes it to subscribers. When a `Registry`
//! is used as the base `Collect` of a `Subscribe` stack, the
//! [`subscribe::Context`][ctx] type will provide methods allowing subscribers to
//! [look up span data][lookup] stored in the registry. While [`Registry`] is a
//! reasonable default for storing spans and events, other stores that implement
//! [`LookupSpan`] and [`Collect`] themselves (with [`SpanData`] implemented
//! by the per-span data they store) can be used as a drop-in replacement.
//!
//! For example, we might create a `Registry` and add multiple `Subscriber`s like so:
//! ```rust
//! use tracing_subscriber::{registry::Registry, Subscribe, prelude::*};
//! # use tracing_core::Collect;
//! # pub struct FooSubscriber {}
//! # pub struct BarSubscriber {}
//! # impl<S: Collect> Subscribe<S> for FooSubscriber {}
//! # impl<S: Collect> Subscribe<S> for BarSubscriber {}
//! # impl FooSubscriber {
//! # fn new() -> Self { Self {} }
//! # }
//! # impl BarSubscriber {
//! # fn new() -> Self { Self {} }
//! # }
//!
//! let subscriber = Registry::default()
//!     .with(FooSubscriber::new())
//!     .with(BarSubscriber::new());
//! ```
//!
//! If a type implementing `Subscribe` depends on the functionality of a `Registry`
//! implementation, it should bound its `Collect` type parameter with the
//! [`LookupSpan`] trait, like so:
//!
//! ```rust
//! use tracing_subscriber::{registry, Subscribe};
//! use tracing_core::Collect;
//!
//! pub struct MySubscriber {
//!     // ...
//! }
//!
//! impl<S> Subscribe<S> for MySubscriber
//! where
//!     S: Collect + for<'a> registry::LookupSpan<'a>,
//! {
//!     // ...
//! }
//! ```
//! When this bound is added, the subscriber implementation will be guaranteed
//! access to the [`Context`][ctx] methods, such as [`Context::span`][lookup], that
//! require the root collector to be a registry.
//!
//! [`Subscribe`]: crate::subscribe::Subscribe
//! [`Collect`]: tracing_core::collect::Collect
//! [ctx]: crate::subscribe::Context
//! [lookup]: crate::subscribe::Context::span()
use tracing_core::{field::FieldSet, span::Id, Metadata};

/// A module containing a type map of span extensions.
mod extensions;

cfg_feature!("registry", {
    mod sharded;
    mod stack;

    pub use sharded::Data;
    pub use sharded::Registry;
});

pub use extensions::{Extensions, ExtensionsMut};

/// Provides access to stored span data.
///
/// Subscribers which store span data and associate it with span IDs should
/// implement this trait; if they do, any [`Subscriber`]s wrapping them can look up
/// metadata via the [`Context`] type's [`span()`] method.
///
/// [`Subscriber`]: crate::Subscribe
/// [`Context`]: crate::subscribe::Context
/// [`span()`]: crate::subscribe::Context::span()
pub trait LookupSpan<'a> {
    /// The type of span data stored in this registry.
    type Data: SpanData<'a>;

    /// Returns the [`SpanData`] for a given [`Id`], if it exists.
    ///
    /// <div class="information">
    ///     <div class="tooltip ignore" style="">ⓘ<span class="tooltiptext">Note</span></div>
    /// </div>
    /// <div class="example-wrap" style="display:inline-block">
    /// <pre class="ignore" style="white-space:normal;font:inherit;">
    ///
    /// **Note**: users of the `LookupSpan` trait should
    /// typically call the [`span`][Self::span] method rather
    /// than this method. The `span` method is implemented by
    /// *calling* `span_data`, but returns a reference which is
    /// capable of performing more sophisticated queries.
    ///
    /// </pre></div>
    ///
    fn span_data(&'a self, id: &Id) -> Option<Self::Data>;

    /// Returns a [`SpanRef`] for the span with the given `Id`, if it exists.
    ///
    /// A `SpanRef` is similar to [`SpanData`], but it allows performing
    /// additional lookups against the registry that stores the wrapped data.
    ///
    /// In general, _users_ of the `LookupSpan` trait should use this method
    /// rather than the [`span_data`] method; while _implementors_ of this trait
    /// should only implement `span_data`.
    ///
    /// [`span_data`]: LookupSpan::span_data()
    fn span(&'a self, id: &Id) -> Option<SpanRef<'_, Self>>
    where
        Self: Sized,
    {
        let data = self.span_data(&id)?;
        Some(SpanRef {
            registry: self,
            data,
        })
    }
}

/// A stored representation of data associated with a span.
pub trait SpanData<'a> {
    /// Returns this span's ID.
    fn id(&self) -> Id;

    /// Returns a reference to the span's `Metadata`.
    fn metadata(&self) -> &'static Metadata<'static>;

    /// Returns a reference to the ID
    fn parent(&self) -> Option<&Id>;

    /// Returns a reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    fn extensions(&self) -> Extensions<'_>;

    /// Returns a mutable reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    fn extensions_mut(&self) -> ExtensionsMut<'_>;
}

/// A reference to [span data] and the associated [registry].
///
/// This type implements all the same methods as [`SpanData`][span data], and
/// provides additional methods for querying the registry based on values from
/// the span.
///
/// [span data]: SpanData
/// [registry]: LookupSpan
#[derive(Debug)]
pub struct SpanRef<'a, R: LookupSpan<'a>> {
    registry: &'a R,
    data: R::Data,
}

/// An iterator over the parents of a span.
///
/// This is returned by the [`SpanRef::parents`] method.
///
#[derive(Debug)]
pub struct Parents<'a, R> {
    registry: &'a R,
    next: Option<Id>,
}

/// An iterator over a span's parents, starting with the root of the trace
/// tree.
///
/// For additional details, see [`SpanRef::from_root`].
///
/// [`Span::from_root`]: SpanRef::from_root()
pub struct FromRoot<'a, R: LookupSpan<'a>> {
    #[cfg(feature = "smallvec")]
    inner: std::iter::Rev<smallvec::IntoIter<SpanRefVecArray<'a, R>>>,
    #[cfg(not(feature = "smallvec"))]
    inner: std::iter::Rev<std::vec::IntoIter<SpanRef<'a, R>>>,
}

#[cfg(feature = "smallvec")]
type SpanRefVecArray<'span, L> = [SpanRef<'span, L>; 16];

impl<'a, R> SpanRef<'a, R>
where
    R: LookupSpan<'a>,
{
    /// Returns this span's ID.
    pub fn id(&self) -> Id {
        self.data.id()
    }

    /// Returns a static reference to the span's metadata.
    pub fn metadata(&self) -> &'static Metadata<'static> {
        self.data.metadata()
    }

    /// Returns the span's name,
    pub fn name(&self) -> &'static str {
        self.data.metadata().name()
    }

    /// Returns a list of [fields] defined by the span.
    ///
    /// [fields]: tracing_core::field
    pub fn fields(&self) -> &FieldSet {
        self.data.metadata().fields()
    }

    /// Returns the ID of this span's parent, or `None` if this span is the root
    /// of its trace tree.
    pub fn parent_id(&self) -> Option<&Id> {
        self.data.parent()
    }

    /// Returns a `SpanRef` describing this span's parent, or `None` if this
    /// span is the root of its trace tree.
    pub fn parent(&self) -> Option<Self> {
        let id = self.data.parent()?;
        let data = self.registry.span_data(id)?;
        Some(Self {
            registry: self.registry,
            data,
        })
    }

    /// Returns an iterator over all parents of this span, starting with the
    /// immediate parent.
    ///
    /// The iterator will first return the span's immediate parent, followed by
    /// that span's parent, followed by _that_ span's parent, and so on, until a
    /// it reaches a root span.
    pub fn parents(&self) -> Parents<'a, R> {
        Parents {
            registry: self.registry,
            next: self.parent().map(|parent| parent.id()),
        }
    }

    /// Returns an iterator over all parents of this span, starting with the
    /// root of the trace tree.
    ///
    /// The iterator will return the root of the trace tree, followed by the
    /// next span, and then the next, until this span's immediate parent is
    /// returned.
    ///
    /// **Note**: if the "smallvec" feature flag is not enabled, this may
    /// allocate.
    pub fn from_root(&self) -> FromRoot<'a, R> {
        #[cfg(feature = "smallvec")]
        type SpanRefVec<'span, L> = smallvec::SmallVec<SpanRefVecArray<'span, L>>;
        #[cfg(not(feature = "smallvec"))]
        type SpanRefVec<'span, L> = Vec<SpanRef<'span, L>>;

        // an alternative way to handle this would be to the recursive approach that
        // `fmt` uses that _does not_ entail any allocation in this fmt'ing
        // spans path.
        let parents = self.parents().collect::<SpanRefVec<'a, _>>();
        let inner = parents.into_iter().rev();
        FromRoot { inner }
    }

    /// Returns a reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    pub fn extensions(&self) -> Extensions<'_> {
        self.data.extensions()
    }

    /// Returns a mutable reference to this span's `Extensions`.
    ///
    /// The extensions may be used by `Subscriber`s to store additional data
    /// describing the span.
    pub fn extensions_mut(&self) -> ExtensionsMut<'_> {
        self.data.extensions_mut()
    }
}

impl<'a, R> Iterator for Parents<'a, R>
where
    R: LookupSpan<'a>,
{
    type Item = SpanRef<'a, R>;
    fn next(&mut self) -> Option<Self::Item> {
        let id = self.next.take()?;
        let span = self.registry.span(&id)?;
        self.next = span.parent().map(|parent| parent.id());
        Some(span)
    }
}

// === impl FromRoot ===

impl<'span, R> Iterator for FromRoot<'span, R>
where
    R: LookupSpan<'span>,
{
    type Item = SpanRef<'span, R>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'span, R> std::fmt::Debug for FromRoot<'span, R>
where
    R: LookupSpan<'span>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("FromRoot { .. }")
    }
}
