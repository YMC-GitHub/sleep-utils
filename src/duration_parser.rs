use crate::{Result, SleepError};
use std::time::Duration;

/// Parse sleep duration with support for multiple formats
///
/// Supports single units (e.g., "1s", "2m") and multiple units (e.g., "1m30s", "1h2m3s")
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

    // Parse time with units (single or multiple)
    if let Some(duration) = parse_duration_with_unit(&input)? {
        Ok(duration)
    } else {
        Err(SleepError::InvalidDuration(format!(
            "Invalid sleep duration format: '{}'",
            input
        )))
    }
}

/// Parse duration with single or multiple time units
fn parse_duration_with_unit(input: &str) -> Result<Option<Duration>> {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        // Patterns for single units (existing functionality)
        static ref SINGLE_PATTERNS: Vec<(&'static str, f64)> = vec![
            // Milliseconds
            (r"^(\d+)\s*(ms|millis?|milliseconds?)$", 1.0),
            // Seconds
            (r"^(\d+)\s*(s|sec|seconds?)$", 1000.0),
            // Minutes
            (r"^(\d+)\s*(m|min|minutes?)$", 60_000.0),
            // Hours
            (r"^(\d+)\s*(h|hr|hours?)$", 3_600_000.0),
            // Short format (no spaces)
            (r"^(\d+)(ms)$", 1.0),
            (r"^(\d+)(s)$", 1000.0),
            (r"^(\d+)(m)$", 60_000.0),
            (r"^(\d+)(h)$", 3_600_000.0),
        ];

        static ref FLOAT_PATTERNS: Vec<(&'static str, f64)> = vec![
            (r"^(\d*\.?\d+)\s*(s|sec|seconds?)$", 1000.0),
            (r"^(\d*\.?\d+)\s*(m|min|minutes?)$", 60_000.0),
            (r"^(\d*\.?\d+)(s)$", 1000.0),
            (r"^(\d*\.?\d+)(m)$", 60_000.0),
        ];
    }

    // First, try single unit patterns (including float patterns)
    for (pattern, multiplier) in SINGLE_PATTERNS.iter() {
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

    // Then try float patterns (this handles "1.5s" correctly)
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

    // Finally, try multiple units pattern (e.g., "1h2m3s")
    // Only if no single unit pattern matched
    if let Some(duration) = parse_multiple_units(input)? {
        return Ok(Some(duration));
    }

    Ok(None)
}

/// Parse multiple time units in a single string
fn parse_multiple_units(input: &str) -> Result<Option<Duration>> {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        // Improved pattern to avoid matching float numbers
        static ref MULTI_UNIT_PATTERN: Regex = Regex::new(r"(?i)(\d+)\s*([a-z]+)").unwrap();
    }

    let mut total_millis: u64 = 0;
    let mut found_any = false;
    let mut has_positive_value = false;

    for caps in MULTI_UNIT_PATTERN.captures_iter(input) {
        let value: u64 = match caps[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let unit = &caps[2].to_lowercase();
        let multiplier = match unit.as_str() {
            "ms" | "milli" | "millis" | "millisecond" | "milliseconds" => 1,
            "s" | "sec" | "second" | "seconds" => 1000,
            "m" | "min" | "minute" | "minutes" => 60_000,
            "h" | "hr" | "hour" | "hours" => 3_600_000,
            _ => continue, // Skip unknown units
        };

        total_millis += value * multiplier;
        found_any = true;
        if value > 0 {
            has_positive_value = true;
        }
    }

    if found_any {
        // Return Duration::ZERO for all-zero values like "0h0m0s"
        if has_positive_value {
            Ok(Some(Duration::from_millis(total_millis)))
        } else {
            Ok(Some(Duration::ZERO))
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sleep_duration() -> Result<()> {
        // Single unit tests (existing functionality)
        assert_eq!(parse_sleep_duration("100")?, Duration::from_millis(100));
        assert_eq!(parse_sleep_duration("100ms")?, Duration::from_millis(100));
        assert_eq!(parse_sleep_duration("1s")?, Duration::from_secs(1));
        assert_eq!(parse_sleep_duration("1.5s")?, Duration::from_millis(1500));
        assert_eq!(parse_sleep_duration("0")?, Duration::ZERO);

        // Multiple unit tests (new functionality)
        assert_eq!(parse_sleep_duration("1m30s")?, Duration::from_millis(90000));
        assert_eq!(
            parse_sleep_duration("1h2m3s")?,
            Duration::from_millis(3723000)
        );
        assert_eq!(
            parse_sleep_duration("1h 2m 3s")?,
            Duration::from_millis(3723000)
        );
        assert_eq!(
            parse_sleep_duration("2s500ms")?,
            Duration::from_millis(2500)
        );
        assert_eq!(
            parse_sleep_duration("1m30s500ms")?,
            Duration::from_millis(90500)
        );

        Ok(())
    }

    #[test]
    fn test_multiple_units_edge_cases() -> Result<()> {
        // Mixed formats
        assert_eq!(parse_sleep_duration("1m30s")?, Duration::from_secs(90));
        assert_eq!(parse_sleep_duration("1h30m")?, Duration::from_secs(5400));
        assert_eq!(parse_sleep_duration("1h1s")?, Duration::from_secs(3601));

        // With spaces
        assert_eq!(parse_sleep_duration("1h 30m")?, Duration::from_secs(5400));
        assert_eq!(parse_sleep_duration("2m 30s")?, Duration::from_secs(150));

        // Zero values in multi-unit - these should return Duration::ZERO
        assert_eq!(parse_sleep_duration("0h0m0s")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0s")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0ms")?, Duration::ZERO);

        Ok(())
    }

    #[test]
    fn test_float_numbers_not_matched_as_multi_units() -> Result<()> {
        // These should be parsed as single units with float values, not multiple units
        assert_eq!(parse_sleep_duration("1.5s")?, Duration::from_millis(1500));
        assert_eq!(parse_sleep_duration("0.5m")?, Duration::from_millis(30000));
        assert_eq!(parse_sleep_duration("2.5s")?, Duration::from_millis(2500));

        // These should be parsed as multiple units
        assert_eq!(
            parse_sleep_duration("1s500ms")?,
            Duration::from_millis(1500)
        );
        assert_eq!(parse_sleep_duration("1m30s")?, Duration::from_millis(90000));

        Ok(())
    }

    #[test]
    fn test_zero_values() -> Result<()> {
        // All zero values should return Duration::ZERO
        assert_eq!(parse_sleep_duration("0")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0ms")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0s")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0m")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0h")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0h0m0s")?, Duration::ZERO);
        assert_eq!(parse_sleep_duration("0s0ms")?, Duration::ZERO);

        Ok(())
    }
}
