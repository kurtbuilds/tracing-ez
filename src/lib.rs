pub use tracing::span as tracing_span;
pub use tracing::{info, warn, error, debug, Level};
use tracing_subscriber::fmt;
use std::io;

#[macro_export]
macro_rules! span {
    ($name:expr) => {
        let span = $crate::tracing_span!($crate::Level::INFO, $name);
        let _enter = span.enter();
    };
    ($name:expr, $($fields:tt)*) => {
        let span = $crate::tracing_span!($crate::Level::INFO, $name, $($fields)*);
        let _enter = span.enter();
    }
}

/// According to tracing-subscriber docs, init() is supposed to panic if
/// another subscriber is already set. However, it does not. Instead, right
/// now, the *first* subscriber is used (*not* the last).
pub fn set_global_default_stdout() {
    fmt()
        .with_writer(io::stdout)
        .init();
}

/// According to tracing-subscriber docs, init() is supposed to panic if
/// another subscriber is already set. However, it does not. Instead, right
/// now, the *first* subscriber is used (*not* the last).
pub fn set_global_default_stderr() {
    fmt()
        .with_writer(io::stderr)
        .init();
}