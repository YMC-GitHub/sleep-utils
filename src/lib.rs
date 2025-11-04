//! A smart sleep utilities library with flexible input formats and automatic zero-value handling.
//!
//! This library provides intelligent sleep functionality that supports multiple input formats
//! and automatically handles zero and negative values without performing actual sleep.
//!
//! # Features
//!
//! - **Multiple input formats**: numbers, text, `Duration` objects
//! - **Automatic zero/negative handling**: no sleep for zero or negative values
//! - **Multiple time units**: milliseconds, seconds, minutes, hours
//! - **Combined units**: support for formats like `"1m30s"`, `"1h2m3s"`
//! - **Platform compatibility**: uses `isize` for cross-platform support
//! - **High performance**: optimized regex parsing with lazy static patterns
//!
//! # Examples
//!
//! Basic usage with different input formats:
//!
//! ```
//! use sleep_utils::smart_sleep;
//! use std::time::Duration;
//!
//! // Sleep for 1 millisecond using a number
//! smart_sleep(1).unwrap();
//!
//! // Sleep for 1 millisecond using text with units
//! smart_sleep("1ms").unwrap();
//!
//! // Sleep for 0.001 seconds
//! smart_sleep("0.001s").unwrap();
//!
//! // Sleep for 1 millisecond with full unit name
//! smart_sleep("1 millisecond").unwrap();
//!
//! // Sleep using combined units
//! smart_sleep("1s1ms").unwrap(); // 1 second 1 millisecond
//!
//! // No sleep for zero values
//! smart_sleep(0).unwrap();
//!
//! // No sleep for negative values
//! smart_sleep(-50).unwrap();
//!
//! // Sleep using Duration object
//! smart_sleep(Duration::from_millis(1)).unwrap();
//! ```
//!
//! # Errors
//!
//! Functions return [`Result<T, SleepError>`] and may return the following errors:
//!
//! - [`SleepError::InvalidDuration`] when parsing invalid duration strings
//! - [`SleepError::ParseError`] when encountering parse errors
//! - [`SleepError::NumberOutOfRange`] when numbers are out of valid range

#![warn(missing_docs)]

use std::time::Duration;

mod duration_parser;
mod error;
mod smart_sleep;

pub use duration_parser::parse_sleep_duration;
pub use error::{Result, SleepError};
pub use smart_sleep::{smart_sleep, SleepInput};

/// Standard sleep function for backward compatibility with `std::thread::sleep`.
///
/// This function provides a simple wrapper around `std::thread::sleep` that returns
/// a [`Result`] for consistency with other functions in this crate.
///
/// # Examples
///
/// ```
/// use sleep_utils::sleep;
/// use std::time::Duration;
///
/// // Sleep for 1 millisecond
/// sleep(Duration::from_millis(1)).unwrap();
/// ```
///
/// # Notes
///
/// Unlike [`smart_sleep`], this function does not support multiple input formats
/// and will always sleep for the provided duration, even if it's zero.
pub fn sleep(duration: Duration) -> Result<()> {
    std::thread::sleep(duration);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_basic_sleep() -> Result<()> {
        let start = Instant::now();
        smart_sleep(1)?; // Use minimal sleep time for tests
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(1));
        Ok(())
    }
}
