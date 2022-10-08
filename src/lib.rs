pub use tracing::{info, warn, error, debug};


#[macro_export]
macro_rules! span {
    ($name:expr) => {
        let span = tracing::span!(Level::INFO, $name);
        let _enter = span.enter();
    }
    ($name:expr, $($fields:tt)*) => {
        tracing::span!(Level::INFO, $name, $($fields)*);
        let _enter = span.enter();
    }
}

pub fn set_global_default_stdout() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
}