use crate::{Result, SleepError};
use std::time::Duration;

/// Parse sleep duration with support for multiple formats
pub fn parse_sleep_duration(input: &str) -> Result<Duration> {
    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Ok(Duration::ZERO);
    }

    // Try to parse as plain number (default to milliseconds)
    if let Ok(millis) = input.parse::<isize>() {
        if millis <= 0 {
            return Ok(Duration::ZERO);
        }
        return Ok(Duration::from_millis(millis as u64));
    }

    // Parse time with units
    if let Some(duration) = parse_duration_with_unit(&input)? {
        Ok(duration)
    } else {
        Err(SleepError::InvalidDuration(format!(
            "Invalid sleep duration format: '{}'",
            input
        )))
    }
}

/// Parse duration with time units
fn parse_duration_with_unit(input: &str) -> Result<Option<Duration>> {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref PATTERNS: Vec<(&'static str, f64)> = vec![
            // Milliseconds
            (r"^(\d+)\s*(ms|millis?|milliseconds?)$", 1.0),
            // Seconds
            (r"^(\d+)\s*(s|sec|seconds?)$", 1000.0),
            // Minutes
            (r"^(\d+)\s*(m|min|minutes?)$", 60_000.0),
            // Short format (no spaces)
            (r"^(\d+)(ms)$", 1.0),
            (r"^(\d+)(s)$", 1000.0),
            (r"^(\d+)(m)$", 60_000.0),
        ];

        static ref FLOAT_PATTERNS: Vec<(&'static str, f64)> = vec![
            (r"^(\d*\.?\d+)\s*(s|sec|seconds?)$", 1000.0),
            (r"^(\d*\.?\d+)\s*(m|min|minutes?)$", 60_000.0),
            (r"^(\d*\.?\d+)(s)$", 1000.0),
            (r"^(\d*\.?\d+)(m)$", 60_000.0),
        ];
    }

    for (pattern, multiplier) in PATTERNS.iter() {
        let re = Regex::new(pattern).unwrap();
        if let Some(caps) = re.captures(input) {
            if let Ok(value) = caps[1].parse::<isize>() {
                if value <= 0 {
                    return Ok(Some(Duration::ZERO));
                }
                let millis = (value as f64 * multiplier) as u64;
                return Ok(Some(Duration::from_millis(millis)));
            }
        }
    }

    for (pattern, multiplier) in FLOAT_PATTERNS.iter() {
        let re = Regex::new(pattern).unwrap();
        if let Some(caps) = re.captures(input) {
            if let Ok(value) = caps[1].parse::<f64>() {
                if value <= 0.0 {
                    return Ok(Some(Duration::ZERO));
                }
                let millis = (value * multiplier) as u64;
                return Ok(Some(Duration::from_millis(millis)));
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sleep_duration() -> Result<()> {
        assert_eq!(parse_sleep_duration("100")?, Duration::from_millis(100));
        assert_eq!(parse_sleep_duration("100ms")?, Duration::from_millis(100));
        assert_eq!(parse_sleep_duration("1s")?, Duration::from_secs(1));
        assert_eq!(parse_sleep_duration("1.5s")?, Duration::from_millis(1500));
        assert_eq!(parse_sleep_duration("0")?, Duration::ZERO);
        Ok(())
    }
}
