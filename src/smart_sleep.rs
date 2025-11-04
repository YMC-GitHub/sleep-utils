use crate::{parse_sleep_duration, Result};
use std::time::Duration;

/// Smart sleep function that supports multiple input formats.
///
/// This function accepts various input types and automatically converts them
/// to appropriate sleep durations. Zero and negative values result in no sleep.
///
/// # Supported Formats
///
/// - **Numbers**: `100`, `500` (interpreted as milliseconds)
/// - **Text with units**: `"100ms"`, `"2s"`, `"1.5s"`, `"2 minutes"`
/// - **Plain text**: `"100"` (interpreted as milliseconds)
/// - **Duration objects**: `Duration::from_millis(100)`
/// - **Zero/negative**: `0`, `-100` (no sleep performed)
///
/// # Examples
///
/// ```
/// use sleep_utils::smart_sleep;
///
/// // Sleep for 100 milliseconds using a number
/// smart_sleep(100).unwrap();
///
/// // Sleep for 200 milliseconds using text with units
/// smart_sleep("200ms").unwrap();
///
/// // Sleep for 1.5 seconds
/// smart_sleep("1.5s").unwrap();
///
/// // Sleep for 2 seconds with full unit name
/// smart_sleep("2 seconds").unwrap();
///
/// // No sleep for zero values
/// smart_sleep(0).unwrap();
///
/// // No sleep for negative values
/// smart_sleep(-100).unwrap();
/// ```
///
/// # Errors
///
/// Returns [`SleepError::InvalidDuration`] if the input string cannot be parsed
/// as a valid duration.
///
/// # Panics
///
/// This function does not panic. All errors are returned as [`Result`].
pub fn smart_sleep<S>(input: S) -> Result<()>
where
    S: Into<SleepInput>,
{
    let sleep_input = input.into();

    if sleep_input.should_sleep() {
        let duration = sleep_input.to_duration()?;
        std::thread::sleep(duration);
    }

    Ok(())
}

/// Represents different types of sleep inputs.
///
/// This enum allows the [`smart_sleep`] function to accept multiple input types
/// through the [`Into<SleepInput>`] trait.
///
/// # Variants
///
/// - `Number(isize)`: Numeric input interpreted as milliseconds
/// - `Text(String)`: String input that will be parsed for duration
/// - `Duration(Duration)`: Standard duration object
#[derive(Debug, Clone)]
pub enum SleepInput {
    /// Numeric input interpreted as milliseconds
    Number(isize),
    /// Text input that will be parsed for duration information
    Text(String),
    /// Standard duration object
    Duration(Duration),
}

// Implement various From traits for seamless conversion
impl From<i32> for SleepInput {
    fn from(value: i32) -> Self {
        SleepInput::Number(value as isize)
    }
}

impl From<isize> for SleepInput {
    fn from(value: isize) -> Self {
        SleepInput::Number(value)
    }
}

impl From<&str> for SleepInput {
    fn from(value: &str) -> Self {
        SleepInput::Text(value.to_string())
    }
}

impl From<String> for SleepInput {
    fn from(value: String) -> Self {
        SleepInput::Text(value)
    }
}

impl From<Duration> for SleepInput {
    fn from(value: Duration) -> Self {
        SleepInput::Duration(value)
    }
}

impl SleepInput {
    /// Determines whether sleep should be performed for this input.
    ///
    /// Returns `false` for zero or negative numeric values, allowing
    /// the caller to skip sleep operations when appropriate.
    ///
    /// # Examples
    ///
    /// ```
    /// use sleep_utils::SleepInput;
    ///
    /// let positive = SleepInput::from(100);
    /// assert!(positive.should_sleep());
    ///
    /// let zero = SleepInput::from(0);
    /// assert!(!zero.should_sleep());
    ///
    /// let negative = SleepInput::from(-50);
    /// assert!(!negative.should_sleep());
    /// ```
    pub fn should_sleep(&self) -> bool {
        match self {
            SleepInput::Number(n) => *n > 0,
            SleepInput::Text(text) => {
                if let Ok(n) = text.parse::<isize>() {
                    n > 0
                } else {
                    true
                }
            }
            SleepInput::Duration(duration) => !duration.is_zero(),
        }
    }

    /// Converts the input to a [`Duration`] object.
    ///
    /// For text inputs, this will parse the string using [`parse_sleep_duration`].
    /// For numeric inputs, values are interpreted as milliseconds.
    ///
    /// # Errors
    ///
    /// Returns [`SleepError::InvalidDuration`] if text input cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use sleep_utils::SleepInput;
    /// use std::time::Duration;
    ///
    /// let input = SleepInput::from("100ms");
    /// let duration = input.to_duration().unwrap();
    /// assert_eq!(duration, Duration::from_millis(100));
    /// ```
    pub fn to_duration(&self) -> Result<Duration> {
        match self {
            SleepInput::Number(n) => {
                if *n <= 0 {
                    Ok(Duration::ZERO)
                } else {
                    Ok(Duration::from_millis(*n as u64))
                }
            }
            SleepInput::Text(text) => parse_sleep_duration(text),
            SleepInput::Duration(duration) => Ok(*duration),
        }
    }
}
