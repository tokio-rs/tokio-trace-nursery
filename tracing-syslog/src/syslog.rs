use std::{borrow::Cow, ffi::CStr};
use tracing_core::{
    field::{Field, Visit},
    Collect, Event, Level,
};
use tracing_subscriber::{
    registry::LookupSpan,
    subscribe::{Context, Subscribe},
};

/// `syslog` options.
///
/// # Examples
/// ```
/// use tracing_syslog::Options;
/// // Log PID with messages and log to stderr as well as `syslog`.
/// let opts = Options::LOG_PID | Options::LOG_PERROR;
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct Options(libc::c_int);

impl Options {
    /// Log the pid with each message.
    pub const LOG_PID: Self = Self(libc::LOG_PID);
    /// Log on the console if errors in sending.
    pub const LOG_CONS: Self = Self(libc::LOG_CONS);
    /// Delay open until first syslog() (default).
    pub const LOG_ODELAY: Self = Self(libc::LOG_ODELAY);
    /// Don't delay open.
    pub const LOG_NDELAY: Self = Self(libc::LOG_NDELAY);
    /// Don't wait for console forks: DEPRECATED.
    pub const LOG_NOWAIT: Self = Self(libc::LOG_NOWAIT);
    /// Log to stderr as well.
    pub const LOG_PERROR: Self = Self(libc::LOG_PERROR);
}

impl std::ops::BitOr for Options {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// `syslog` facility.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum Facility {
    /// Kernel messages (these can't be generated from user processes).
    LOG_KERN = libc::LOG_KERN,
    /// Generic user-level messages.
    LOG_USER = libc::LOG_USER,
    /// Mail subsystem.
    LOG_MAIL = libc::LOG_MAIL,
    /// System daemons without separate facility value.
    LOG_DAEMON = libc::LOG_DAEMON,
    /// Security/authorization messages.
    LOG_AUTH = libc::LOG_AUTH,
    /// Messages generated internally by `syslogd(8)`.
    LOG_SYSLOG = libc::LOG_SYSLOG,
    /// Line printer subsystem.
    LOG_LPR = libc::LOG_LPR,
    /// USENET news subsystem.
    LOG_NEWS = libc::LOG_NEWS,
    /// UUCP subsystem.
    LOG_UUCP = libc::LOG_UUCP,
    /// Clock daemon (`cron` and `at`).
    LOG_CRON = libc::LOG_CRON,
    /// Security/authorization messages (private).
    LOG_AUTHPRIV = libc::LOG_AUTHPRIV,
    /// FTP daemon.
    LOG_FTP = libc::LOG_FTP,
    /// Reserved for local use.
    LOG_LOCAL0 = libc::LOG_LOCAL0,
    /// Reserved for local use.
    LOG_LOCAL1 = libc::LOG_LOCAL1,
    /// Reserved for local use.
    LOG_LOCAL2 = libc::LOG_LOCAL2,
    /// Reserved for local use.
    LOG_LOCAL3 = libc::LOG_LOCAL3,
    /// Reserved for local use.
    LOG_LOCAL4 = libc::LOG_LOCAL4,
    /// Reserved for local use.
    LOG_LOCAL5 = libc::LOG_LOCAL5,
    /// Reserved for local use.
    LOG_LOCAL6 = libc::LOG_LOCAL6,
    /// Reserved for local use.
    LOG_LOCAL7 = libc::LOG_LOCAL7,
}

impl Default for Facility {
    fn default() -> Self {
        Self::LOG_USER
    }
}

/// `syslog` severity.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(i32)]
pub enum Severity {
    /// System is unusable.
    LOG_EMERG = libc::LOG_EMERG,
    /// Action must be taken immediately.
    LOG_ALERT = libc::LOG_ALERT,
    /// Critical conditions.
    LOG_CRIT = libc::LOG_CRIT,
    /// Error conditions.
    LOG_ERR = libc::LOG_ERR,
    /// Warning conditions.
    LOG_WARNING = libc::LOG_WARNING,
    /// Normal, but significant, condition.
    LOG_NOTICE = libc::LOG_NOTICE,
    /// Informational message.
    LOG_INFO = libc::LOG_INFO,
    /// Debug-level message.
    LOG_DEBUG = libc::LOG_DEBUG,
}

impl From<Level> for Severity {
    fn from(level: Level) -> Self {
        match level {
            Level::ERROR => Self::LOG_ERR,
            Level::WARN => Self::LOG_WARNING,
            Level::INFO => Self::LOG_NOTICE,
            Level::DEBUG => Self::LOG_INFO,
            Level::TRACE => Self::LOG_DEBUG,
        }
    }
}

/// `syslog` priority.
#[derive(Copy, Clone, Debug)]
struct Priority(libc::c_int);

impl Priority {
    fn new(facility: Facility, level: Level) -> Self {
        let severity = Severity::from(level);
        Self((facility as libc::c_int) | (severity as libc::c_int))
    }
}

fn syslog(priority: Priority, msg: &CStr) {
    // SAFETY: the second argument must be a valid pointer to a nul-terminated
    // format string and formatting placeholders e.g. %s must correspond to
    // one of the variable-length arguments. By construction, the format string
    // is nul-terminated, and the only string formatting placeholder corresponds
    // to `msg.as_ptr()`, which is a valid, nul-terminated string in C world
    // because `msg` is a `CStr`.
    unsafe { libc::syslog(priority.0, "%s\0".as_ptr().cast(), msg.as_ptr()) }
}

/// [`Subscriber`](tracing_subscriber::Subscribe) that logs to `syslog` via
/// `libc`'s [`syslog()`](libc::syslog) function.
///
/// # Level Mapping
///
/// `tracing` [`Level`](tracing_core::Level)s are mapped to `syslog` [`Severity`]s as follows:
///
/// ```raw
/// Level::ERROR => Severity::LOG_ERR,
/// Level::WARN  => Severity::LOG_WARNING,
/// Level::INFO  => Severity::LOG_NOTICE,
/// Level::DEBUG => Severity::LOG_INFO,
/// Level::TRACE => Severity::LOG_DEBUG,
/// ```
///
/// **Note:** the mapping is lossless, but the corresponding `syslog` severity
/// names differ from `tracing`'s level names towards the bottom. `syslog`
/// does not have a level lower than `LOG_DEBUG`, so this is unavoidable.
///
/// # Examples
/// Initializing a global [`Collector`](tracing_core::Collect) that logs to `syslog` with
/// an identity of `example-program` and the default `syslog` options and facility:
/// ```
/// use tracing_syslog::Syslog;
/// use tracing_subscriber::{Registry, subscribe::CollectExt};
/// let identity = std::ffi::CStr::from_bytes_with_nul(b"example-program\0").unwrap();
/// let (options, facility) = Default::default();
/// let collector = Registry::default().with(Syslog::new(identity, options, facility));
/// tracing::collect::set_global_default(collector).unwrap();
/// ```
pub struct Syslog {
    /// Identity e.g. program name. Referenced by syslog, so we store it here to
    /// ensure it lives until we are done logging.
    #[allow(dead_code)]
    identity: Cow<'static, CStr>,
    facility: Facility,
}

impl Syslog {
    /// Creates a [`Subscriber`](tracing_subscriber::Subscribe) that logs to `syslog`.
    ///
    /// This calls [`libc::openlog()`] to initialize the logger. The corresponding
    /// [`libc::closelog()`] call happens when the returned logger is dropped.
    ///
    /// # Examples
    /// Creating a `syslog` subscriber with an identity of `example-program` and
    /// the default `syslog` options and facility:
    /// ```
    /// use tracing_syslog::Syslog;
    /// let identity = std::ffi::CStr::from_bytes_with_nul(b"example-program\0").unwrap();
    /// let (options, facility) = Default::default();
    /// let subscriber = Syslog::new(identity, options, facility);
    /// ```
    pub fn new(
        identity: impl Into<Cow<'static, CStr>>,
        options: Options,
        facility: Facility,
    ) -> Self {
        let identity = identity.into();
        // SAFETY: identity will remain alive until the returned struct's fields
        // are dropped, by which point `closelog` will have been called by the
        // `Drop` implementation.
        unsafe { libc::openlog(identity.as_ptr(), options.0, facility as libc::c_int) };
        Syslog { identity, facility }
    }
}

impl Drop for Syslog {
    /// Calls [`libc::closelog()`].
    fn drop(&mut self) {
        unsafe { libc::closelog() };
    }
}

impl<C> Subscribe<C> for Syslog
where
    C: Collect + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event, _ctx: Context<C>) {
        use std::cell::RefCell;
        thread_local! { static BUF: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(256)) }

        BUF.with(|buf| {
            let mut buf = buf.borrow_mut();

            // Record event fields
            event.record(&mut EventVisitor(&mut buf));
            // Append nul-terminator
            buf.push(0);

            // Log message
            let priority = Priority::new(self.facility, *event.metadata().level());
            let msg =
                CStr::from_bytes_with_nul(&buf).expect("logs free of interior nul-terminators");
            syslog(priority, &msg);

            // Clear buffer
            buf.clear();
        })
    }
}

struct EventVisitor<'a>(&'a mut Vec<u8>);

impl Visit for EventVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        use std::io::Write;
        if field.name() != "message" {
            self.0.push(b' ');
            self.0.extend_from_slice(field.name().as_bytes());
            self.0.push(b'=');
        }
        write!(&mut self.0, "{:?}", value).expect("io::Write impl on Vec never fails");
    }
}
