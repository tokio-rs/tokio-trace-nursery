/// Constructs a span at the trace level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// [attributes]: macro.span.html#setting-span-attributes
/// [Fields]: macro.span.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate tracing;
/// # use tracing::Level;
/// # fn main() {
/// trace_span!("my_span");
/// // is equivalent to:
/// span!(Level::TRACE, "my_span");
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// let span = trace_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! trace_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::trace_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::trace_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::trace_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    ($name:expr) => { $crate::trace_span!($name,) };
}

/// Constructs a span at the debug level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// [attributes]: macro.span.html#setting-span-attributes
/// [Fields]: macro.span.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate tracing;
/// # use tracing::Level;
/// # fn main() {
/// debug_span!("my_span");
/// // is equivalent to:
/// span!(Level::DEBUG, "my_span");
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// let span = debug_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::debug_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::debug_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::debug_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::debug_span!($name,)};
}

/// Constructs a span at the info level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// [attributes]: macro.span.html#setting-span-attributes
/// [Fields]: macro.span.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate tracing;
/// # use tracing::Level;
/// # fn main() {
/// info_span!("my_span");
/// // is equivalent to:
/// span!(Level::INFO, "my_span");
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// let span = info_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! info_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::info_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::info_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::info_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::info_span!($name,)};
}

/// Constructs a span at the warn level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// [attributes]: macro.span.html#setting-span-attributes
/// [Fields]: macro.span.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate tracing;
/// # use tracing::Level;
/// # fn main() {
/// info_span!("my_span");
/// // is equivalent to:
/// span!(Level::INFO, "my_span");
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// let span = warn_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! warn_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::warn_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::warn_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::warn_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::warn_span!($name,)};
}
/// Constructs a span at the error level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// [attributes]: macro.span.html#setting-span-attributes
/// [Fields]: macro.span.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate tracing;
/// # use tracing::Level;
/// # fn main() {
/// error_span!("my_span");
/// // is equivalent to:
/// span!(Level::ERROR, "my_span");
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// let span = error_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! error_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::error_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::error_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::error_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::error_span!($name,)};
}


/// Constructs an event at the trace level.
///
/// When both a message and fields are included, curly braces (`{` and `}`) are
/// used to delimit the list of fields from the format string for the message.
/// A trailing comma on the final field is valid.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # use std::time::SystemTime;
/// # #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
/// # impl Position {
/// # const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
/// # fn dist(&self, other: Position) -> f32 {
/// #    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
/// #    (x + y).sqrt()
/// # }
/// # }
/// # fn main() {
/// use tracing::field;
///
/// let pos = Position { x: 3.234, y: -1.223 };
/// let origin_dist = pos.dist(Position::ORIGIN);
///
/// trace!(position = field::debug(pos), origin_dist = field::debug(origin_dist));
/// trace!(target: "app_events",
///         { position = field::debug(pos) },
///         "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE,  { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the debug level.
///
/// When both a message and fields are included, curly braces (`{` and `}`) are
/// used to delimit the list of fields from the format string for the message.
/// A trailing comma on the final field is valid.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// # #[derive(Debug)] struct Position { x: f32, y: f32 }
/// use tracing::field;
///
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!(?pos.x, ?pos.y);
/// debug!(target: "app_events", { position = ?pos }, "New position");
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG,  { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the info level.
///
/// When both a message and fields are included, curly braces (`{` and `}`) are
/// used to delimit the list of fields from the format string for the message.
/// A trailing comma on the final field is valid.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # use std::net::Ipv4Addr;
/// # fn main() {
/// # struct Connection { port: u32,  speed: f32 }
/// use tracing::field;
///
/// let addr = Ipv4Addr::new(127, 0, 0, 1);
/// let conn = Connection { port: 40, speed: 3.20 };
///
/// info!({ port = conn.port }, "connected to {}", addr);
/// info!(
///     target: "connection_events",
///     ip = %addr,
///     conn.port,
///     ?conn.speed,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! info {
     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the warn level.
///
/// When both a message and fields are included, curly braces (`{` and `}`) are
/// used to delimit the list of fields from the format string for the message.
/// A trailing comma on the final field is valid.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// use tracing::field;
///
/// let warn_description = "Invalid Input";
/// let input = &[0x27, 0x45];
///
/// warn!(input = field::debug(input), warning = warn_description);
/// warn!(
///     target: "input_events",
///     { warning = warn_description },
///     "Received warning for input: {:?}", input,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! warn {
     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the error level.
///
/// When both a message and fields are included, curly braces (`{` and `}`) are
/// used to delimit the list of fields from the format string for the message.
/// A trailing comma on the final field is valid.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate tracing;
/// # fn main() {
/// use tracing::field;
/// let (err_info, port) = ("No connection", 22);
///
/// error!(port = port, error = field::display(err_info));
/// error!(target: "app_events", "App Error: {}", err_info);
/// error!({ info = err_info }, "error on port: {}", port);
/// # }
/// ```
#[macro_export]
macro_rules! error {
     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            {},
            $($arg)+
        )
    );
}

/// Constructs a new static callsite for a span or event.
#[doc(hidden)]
#[macro_export]
macro_rules! callsite {
    (name: $name:expr, kind: $kind:expr, fields: $($fields:tt)*) => {{
        $crate::callsite! {
            name: $name,
            kind: $kind,
            target: module_path!(),
            level: $crate::Level::TRACE,
            fields: $($fields)*
        }
    }};
    (
        name: $name:expr,
        kind: $kind:expr,
        level: $lvl:expr,
        fields: $($fields:tt)*
    ) => {{
        $crate::callsite! {
            name: $name,
            kind: $kind,
            target: module_path!(),
            level: $lvl,
            fields: $($fields)*
        }
    }};
    (
        name: $name:expr,
        kind: $kind:expr,
        target: $target:expr,
        level: $lvl:expr,
        fields: $($fields:tt)*
    ) => {{
        use std::sync::{
            atomic::{self, AtomicUsize, Ordering},
            Once,
        };
        use $crate::{callsite, subscriber::Interest, Metadata};
        struct MyCallsite;
        static META: Metadata<'static> = {
            $crate::metadata! {
                name: $name,
                target: $target,
                level: $lvl,
                fields: $crate::fieldset!( $($fields)* ),
                callsite: &MyCallsite,
                kind: $kind,
            }
        };
        // FIXME: Rust 1.34 deprecated ATOMIC_USIZE_INIT. When Tokio's minimum
        // supported version is 1.34, replace this with the const fn `::new`.
        #[allow(deprecated)]
        static INTEREST: AtomicUsize = atomic::ATOMIC_USIZE_INIT;
        static REGISTRATION: Once = Once::new();
        impl MyCallsite {
            #[inline]
            fn interest(&self) -> Interest {
                match INTEREST.load(Ordering::Relaxed) {
                    0 => Interest::never(),
                    2 => Interest::always(),
                    _ => Interest::sometimes(),
                }
            }
        }
        impl callsite::Callsite for MyCallsite {
            fn set_interest(&self, interest: Interest) {
                let interest = match () {
                    _ if interest.is_never() => 0,
                    _ if interest.is_always() => 2,
                    _ => 1,
                };
                INTEREST.store(interest, Ordering::SeqCst);
            }

            fn metadata(&self) -> &Metadata {
                &META
            }
        }
        REGISTRATION.call_once(|| {
            callsite::register(&MyCallsite);
        });
        &MyCallsite
    }};
}

#[macro_export]
// TODO: determine if this ought to be public API?
#[doc(hidden)]
macro_rules! is_enabled {
    ($callsite:expr) => {{
        let interest = $callsite.interest();
        if interest.is_never() {
            false
        } else if interest.is_always() {
            true
        } else {
            let meta = $callsite.metadata();
            $crate::dispatcher::get_default(|current| current.enabled(meta))
        }
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! fieldset {
    // == base case ==
    (@ { $($out:expr),* $(,)* } $(,)*) => {
        &[ $($out),* ]
    };

    // == empty out set, remaining tts ==
    (@ { } $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { } $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { } $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@ { } $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    // };
    (@ { } ?$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { } %$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { } $($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };


    // == non-empty out set, remaining tts ==
    (@ { $($out:expr),+ } $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),+,$crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $($out:expr),+ } $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),+, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $($out:expr),+ } $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),+, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@ { $($out:expr),+ } $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::fieldset!(@ { $($out),+, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    // };
    (@ { $($out:expr),+ } ?$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),+, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $($out:expr),+ } %$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),+, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $($out:expr),+ } $($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),+, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };

    // == entry ==
    ($($args:tt)*) => {
        $crate::fieldset!(@ { } $($args)*,)
    };

}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! level_to_log {
    ($level:expr) => {
        match $level {
            &$crate::Level::ERROR => $crate::log::Level::Error,
            &$crate::Level::WARN => $crate::log::Level::Warn,
            &$crate::Level::INFO => $crate::log::Level::Info,
            &$crate::Level::DEBUG => $crate::log::Level::Debug,
            _ => $crate::log::Level::Trace,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_stringify {
    ($s:expr) => {
        stringify!($s)
    };
}

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_log {
    (target: $target:expr, $level:expr, $($field:tt)+ ) => {};
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_disabled_span {
    ($meta:expr, $valueset:expr) => {{
        let span = $crate::Span::new_disabled($meta);
        span.record_all(&$valueset);
        span
    }};
}

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_disabled_span {
    ($meta:expr, $valueset:expr) => {
        $crate::Span::new_disabled($meta)
    };
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __mk_format_string {
    // === base case ===
    (@ { $($out:expr),+ } $(,)*) => {
        concat!( $($out),+)
    };

    // === recursive case (more tts), non-empty out set ===

    // ====== shorthand field syntax ===
    (@ { $($out:expr),+ }, ?$($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { $($out:expr),+ }, %$($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, tracing::__tracing_stringify!($($k).+), "={} " }, $($rest)*)
    };
    (@ { $($out:expr),+ }, $($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };

    (@ { $($out:expr),+ }, message = $val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, "{} " }, $($rest)*)
    };
    (@ { $($out:expr),+ }, $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { $($out:expr),+ }, $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, tracing::__tracing_stringify!($($k).+), "={} " }, $($rest)*)
    };
    (@ { $($out:expr),+ }, $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),+, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };


    // === recursive case (more tts), empty out set ===
    // ====== shorthand field syntax ===
    (@ { }, ?$($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { }, %$($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { tracing::__tracing_stringify!($($k).+), "={} " }, $($rest)*)
    };
    (@ { }, $($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { }, message = $val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { "{} " }, $($rest)*)
    };
    (@ { }, $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { }, $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ {  tracing::__tracing_stringify!($($k).+), "={} " }, $($rest)*)
    };
    (@ { }, $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };

    // === entry ===
    ($($kvs:tt)+) => {
        tracing::__mk_format_string!(@ { }, $($kvs)+,)
    };
    () => {
        ""
    }
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __mk_format_args {
    // == base case ==
    (@ { $($out:expr),* }, $fmt:expr, fields: $(,)*) => {
        format_args!($fmt, $($out),*)
    };

    // === recursive case (more tts), non-empty out set ===
    (@ { $($out:expr),+ }, $fmt:expr, fields: $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $($out),+, $val }, $fmt, fields: $($rest)*)
    };
    (@ { $($out:expr),+ }, $fmt:expr, fields: $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $($out),+, $val }, $fmt, fields: $($rest)*)
    };
    (@ { $($out:expr),+ }, $fmt:expr, fields: $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $($out),+, $val }, $fmt, fields: $($rest)*)
    };
    // ====== shorthand field syntax ===
    (@ { $($out:expr),+ }, $fmt:expr, fields: ?$($k:ident).+, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $($out),+, &$($k).+ }, $fmt, fields: $($rest)*)
    };
    (@ { $($out:expr),+ }, $fmt:expr, fields: %$($k:ident).+, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $($out),+, &$($k).+ }, $fmt, fields: $($rest)*)
    };
    (@ { $($out:expr),+ }, $fmt:expr, fields: $($k:ident).+, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $($out),+, &$($k).+ }, $fmt, fields: $($rest)*)
    };


    // == recursive case (more tts), empty out set ===
    (@ { }, $fmt:expr, fields: message = $val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $val }, $fmt, fields: $($rest)*)
    };
    (@ { }, $fmt:expr, fields: $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $val }, $fmt, fields: $($rest)*)
    };
    (@ { }, $fmt:expr, fields: $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $val }, $fmt, fields: $($rest)*)
    };
    (@ { }, $fmt:expr, fields: $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { $val }, $fmt, fields: $($rest)*)
    };
    // ====== shorthand field syntax ===
    (@ { }, $fmt:expr, fields: ?$($k:ident).+, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { &$($k).+ }, $fmt, fields: $($rest)*)
    };
    (@ { }, $fmt:expr, fields: %$($k:ident).+, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { &$($k).+ }, $fmt, fields: $($rest)*)
    };
    (@ { }, $fmt:expr, fields: $($k:ident).+, $($rest:tt)*) => {
        $crate::__mk_format_args!(@ { &$($k).+ }, $fmt, fields: $($rest)*)
    };

    // === entry ===
    ($($kv:tt)*) => {
        {
            // use $crate::__mk_format_string;
            $crate::__mk_format_args!(@ { }, tracing::__mk_format_string!($($kv)*), fields: $($kv)*,)
        }
    };
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_log {
    (target: $target:expr, $level:expr, $($field:tt)+ ) => {
        use $crate::log;
        let level = $crate::level_to_log!(&$level);
        if level <= log::STATIC_MAX_LEVEL {
            let log_meta = log::Metadata::builder()
                .level(level)
                .target($target)
                .build();
            let logger = log::logger();
            if logger.enabled(&log_meta) {
                logger.log(&log::Record::builder()
                    .file(Some(file!()))
                    .module_path(Some(module_path!()))
                    .line(Some(line!()))
                    .metadata(log_meta)
                    .args($crate::__mk_format_args!($($field)+))
                    .build());
            }
        }
    };
}
