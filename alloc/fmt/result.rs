use super::Error;

/// Custom result type for String formatting.
pub type Result<T> = core::result::Result<T, Error>;
