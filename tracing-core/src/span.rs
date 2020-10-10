//! Spans represent periods of time in the execution of a program.
use crate::parent::Parent;
use crate::{field, Metadata};
use core::num::NonZeroU64;

/// Identifies a span within the context of a collector.
///
/// They are generated by [`Collector`]s for each span as it is created, by
/// the [`new_span`] trait method. See the documentation for that method for
/// more information on span ID generation.
///
/// [`Collector`]: super::collector::Collector
/// [`new_span`]: super::collector::Collector::new_span
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id(NonZeroU64);

/// Attributes provided to a `Collector` describing a new span when it is
/// created.
#[derive(Debug)]
pub struct Attributes<'a> {
    metadata: &'static Metadata<'static>,
    values: &'a field::ValueSet<'a>,
    parent: Parent,
}

/// A set of fields recorded by a span.
#[derive(Debug)]
pub struct Record<'a> {
    values: &'a field::ValueSet<'a>,
}

/// Indicates what [the `Collector` considers] the "current" span.
///
/// As collectors may not track a notion of a current span, this has three
/// possible states:
/// - "unknown", indicating that the collector does not track a current span,
/// - "none", indicating that the current context is known to not be in a span,
/// - "some", with the current span's [`Id`] and [`Metadata`].
///
/// [the `Collector` considers]: super::collector::Collector::current_span
/// [`Id`]: Id
/// [`Metadata`]: super::metadata::Metadata
#[derive(Debug)]
pub struct Current {
    inner: CurrentInner,
}

#[derive(Debug)]
enum CurrentInner {
    Current {
        id: Id,
        metadata: &'static Metadata<'static>,
    },
    None,
    Unknown,
}

// ===== impl Span =====

impl Id {
    /// Constructs a new span ID from the given `u64`.
    ///
    /// <div class="information">
    ///     <div class="tooltip ignore" style="">ⓘ<span class="tooltiptext">Note</span></div>
    /// </div>
    /// <div class="example-wrap" style="display:inline-block">
    /// <pre class="ignore" style="white-space:normal;font:inherit;">
    /// <strong>Note</strong>: Span IDs must be greater than zero.</pre></div>
    ///
    /// # Panics
    /// - If the provided `u64` is 0
    pub fn from_u64(u: u64) -> Self {
        Id(NonZeroU64::new(u).expect("span IDs must be > 0"))
    }

    /// Constructs a new span ID from the given `NonZeroU64`.
    ///
    /// Unlike [`Id::from_u64`](Self::from_u64), this will never panic.
    #[inline]
    pub const fn from_non_zero_u64(id: NonZeroU64) -> Self {
        Id(id)
    }

    // Allow `into` by-ref since we don't want to impl Copy for Id
    #[allow(clippy::wrong_self_convention)]
    /// Returns the span's ID as a `u64`.
    pub fn into_u64(&self) -> u64 {
        self.0.get()
    }

    // Allow `into` by-ref since we don't want to impl Copy for Id
    #[allow(clippy::wrong_self_convention)]
    /// Returns the span's ID as a `NonZeroU64`.
    #[inline]
    pub const fn into_non_zero_u64(&self) -> NonZeroU64 {
        self.0
    }
}

impl<'a> Into<Option<Id>> for &'a Id {
    fn into(self) -> Option<Id> {
        Some(self.clone())
    }
}

// ===== impl Attributes =====

impl<'a> Attributes<'a> {
    /// Returns `Attributes` describing a new child span of the current span,
    /// with the provided metadata and values.
    pub fn new(metadata: &'static Metadata<'static>, values: &'a field::ValueSet<'a>) -> Self {
        Attributes {
            metadata,
            values,
            parent: Parent::Current,
        }
    }

    /// Returns `Attributes` describing a new span at the root of its own trace
    /// tree, with the provided metadata and values.
    pub fn new_root(metadata: &'static Metadata<'static>, values: &'a field::ValueSet<'a>) -> Self {
        Attributes {
            metadata,
            values,
            parent: Parent::Root,
        }
    }

    /// Returns `Attributes` describing a new child span of the specified
    /// parent span, with the provided metadata and values.
    pub fn child_of(
        parent: Id,
        metadata: &'static Metadata<'static>,
        values: &'a field::ValueSet<'a>,
    ) -> Self {
        Attributes {
            metadata,
            values,
            parent: Parent::Explicit(parent),
        }
    }

    /// Returns a reference to the new span's metadata.
    pub fn metadata(&self) -> &'static Metadata<'static> {
        self.metadata
    }

    /// Returns a reference to a `ValueSet` containing any values the new span
    /// was created with.
    pub fn values(&self) -> &field::ValueSet<'a> {
        self.values
    }

    /// Returns true if the new span should be a root.
    pub fn is_root(&self) -> bool {
        matches!(self.parent, Parent::Root)
    }

    /// Returns true if the new span's parent should be determined based on the
    /// current context.
    ///
    /// If this is true and the current thread is currently inside a span, then
    /// that span should be the new span's parent. Otherwise, if the current
    /// thread is _not_ inside a span, then the new span will be the root of its
    /// own trace tree.
    pub fn is_contextual(&self) -> bool {
        matches!(self.parent, Parent::Current)
    }

    /// Returns the new span's explicitly-specified parent, if there is one.
    ///
    /// Otherwise (if the new span is a root or is a child of the current span),
    /// returns false.
    pub fn parent(&self) -> Option<&Id> {
        match self.parent {
            Parent::Explicit(ref p) => Some(p),
            _ => None,
        }
    }

    /// Records all the fields in this set of `Attributes` with the provided
    /// [Visitor].
    ///
    /// [visitor]: super::field::Visit
    pub fn record(&self, visitor: &mut dyn field::Visit) {
        self.values.record(visitor)
    }

    /// Returns `true` if this set of `Attributes` contains a value for the
    /// given `Field`.
    pub fn contains(&self, field: &field::Field) -> bool {
        self.values.contains(field)
    }

    /// Returns true if this set of `Attributes` contains _no_ values.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// ===== impl Record =====

impl<'a> Record<'a> {
    /// Constructs a new `Record` from a `ValueSet`.
    pub fn new(values: &'a field::ValueSet<'a>) -> Self {
        Self { values }
    }

    /// Records all the fields in this `Record` with the provided [Visitor].
    ///
    /// [visitor]: super::field::Visit
    pub fn record(&self, visitor: &mut dyn field::Visit) {
        self.values.record(visitor)
    }

    /// Returns `true` if this `Record` contains a value for the given `Field`.
    pub fn contains(&self, field: &field::Field) -> bool {
        self.values.contains(field)
    }

    /// Returns true if this `Record` contains _no_ values.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// ===== impl Current =====

impl Current {
    /// Constructs a new `Current` that indicates the current context is a span
    /// with the given `metadata` and `metadata`.
    pub fn new(id: Id, metadata: &'static Metadata<'static>) -> Self {
        Self {
            inner: CurrentInner::Current { id, metadata },
        }
    }

    /// Constructs a new `Current` that indicates the current context is *not*
    /// in a span.
    pub fn none() -> Self {
        Self {
            inner: CurrentInner::None,
        }
    }

    /// Constructs a new `Current` that indicates the `Collector` does not
    /// track a current span.
    pub(crate) fn unknown() -> Self {
        Self {
            inner: CurrentInner::Unknown,
        }
    }

    /// Returns `true` if the `Collector` that constructed this `Current` tracks a
    /// current span.
    ///
    /// If this returns `true` and [`id`], [`metadata`], or [`into_inner`]
    /// return `None`, that indicates that we are currently known to *not* be
    /// inside a span. If this returns `false`, those methods will also return
    /// `None`, but in this case, that is because the collector does not keep
    /// track of the currently-entered span.
    ///
    /// [`id`]: Self::id
    /// [`metadata`]: Self::metadata
    /// [`into_inner`]: Self::into_inner
    pub fn is_known(&self) -> bool {
        !matches!(self.inner, CurrentInner::Unknown)
    }

    /// Consumes `self` and returns the span `Id` and `Metadata` of the current
    /// span, if one exists and is known.
    pub fn into_inner(self) -> Option<(Id, &'static Metadata<'static>)> {
        match self.inner {
            CurrentInner::Current { id, metadata } => Some((id, metadata)),
            _ => None,
        }
    }

    /// Borrows the `Id` of the current span, if one exists and is known.
    pub fn id(&self) -> Option<&Id> {
        match self.inner {
            CurrentInner::Current { ref id, .. } => Some(id),
            _ => None,
        }
    }

    /// Borrows the `Metadata` of the current span, if one exists and is known.
    pub fn metadata(&self) -> Option<&'static Metadata<'static>> {
        match self.inner {
            CurrentInner::Current { ref metadata, .. } => Some(*metadata),
            _ => None,
        }
    }
}

impl<'a> Into<Option<&'a Id>> for &'a Current {
    fn into(self) -> Option<&'a Id> {
        self.id()
    }
}

impl<'a> Into<Option<Id>> for &'a Current {
    fn into(self) -> Option<Id> {
        self.id().cloned()
    }
}

impl Into<Option<Id>> for Current {
    fn into(self) -> Option<Id> {
        self.id().cloned()
    }
}

impl<'a> Into<Option<&'static Metadata<'static>>> for &'a Current {
    fn into(self) -> Option<&'static Metadata<'static>> {
        self.metadata()
    }
}
