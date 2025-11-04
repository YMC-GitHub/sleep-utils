use thiserror::Error;

/// Sleep utilities error types
#[derive(Error, Debug)]
pub enum SleepError {
    /// Returned when a duration string cannot be parsed
    #[error("Invalid duration format: {0}")]
    InvalidDuration(String),

    /// Returned when general parsing fails
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Returned when numeric values are outside valid range
    #[error("Number out of range: {0}")]
    NumberOutOfRange(String),
}

/// Result type alias for sleep-utils operations
///
/// This is a convenience type that uses [`SleepError`] as the error type
/// for all functions in this crate.
pub type Result<T> = std::result::Result<T, SleepError>;
