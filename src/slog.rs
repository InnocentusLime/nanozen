use std::fmt;

const MAX_LEVEL: Level = Level::Trace;

static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 1,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warning,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        LOG_LEVEL_NAMES[self as usize]
    }
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.as_str())
    }
}

#[doc(hidden)]
pub fn __log(target: &'static str, lvl: Level, args: fmt::Arguments) {
    if lvl < MAX_LEVEL {
        println!("{lvl:<5} [{target}] {args}");
    }
}

#[clippy::format_args]
macro_rules! error {
    (target: $target:ident, $($arg:tt)+) => ({
        crate::slog::__log(stringify!($target), crate::slog::Level::Error, format_args!($($arg)+))
    });
}

#[clippy::format_args]
macro_rules! warning {
    (target: $target:ident, $($arg:tt)+) => ({
        crate::slog::__log(stringify!($target), crate::slog::Level::Warning, format_args!($($arg)+))
    });
}

#[clippy::format_args]
macro_rules! info {
    (target: $target:ident, $($arg:tt)+) => ({
        crate::slog::__log(stringify!($target), crate::slog::Level::Info, format_args!($($arg)+))
    });
}

#[clippy::format_args]
macro_rules! debug {
    (target: $target:ident, $($arg:tt)+) => ({
        crate::slog::__log(stringify!($target), crate::slog::Level::Debug, format_args!($($arg)+))
    });
}

#[clippy::format_args]
macro_rules! trace {
    (target: $target:ident, $($arg:tt)+) => ({
        crate::slog::__log(stringify!($target), crate::slog::Level::Trace, format_args!($($arg)+))
    });
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use trace;
pub(crate) use warning;
