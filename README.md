# sleep-utils

[![Crates.io](https://img.shields.io/crates/v/sleep-utils.svg)](https://crates.io/crates/sleep-utils)
[![Documentation](https://docs.rs/sleep-utils/badge.svg)](https://docs.rs/sleep-utils)
[![License](https://img.shields.io/crates/l/sleep-utils.svg)](https://github.com/ymc-github/sleep-utils#license)

A smart sleep utilities library for Rust with flexible input formats and automatic zero-value handling.

## Features

- 🕒 **Multiple input formats**: numbers, text, `Duration`
- 🚫 **Automatic zero/negative handling**: no sleep for zero or negative values
- 📏 **Multiple time units**: milliseconds, seconds, minutes
- 💻 **Platform-compatible**: uses `isize` for cross-platform support
- ⚡ **High-performance**: optimized regex parsing
- 🎯 **Smart parsing**: intuitive duration string parsing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sleep-utils = "0.1"
```

## Quick Start

```rust
use sleep_utils::smart_sleep;
use std::time::Duration;

// Various input formats
smart_sleep(100).unwrap();           // 100 milliseconds
smart_sleep("200ms").unwrap();       // 200 milliseconds
smart_sleep("1.5s").unwrap();        // 1.5 seconds
smart_sleep("2 seconds").unwrap();   // 2 seconds
smart_sleep(0).unwrap();             // no sleep
smart_sleep(-50).unwrap();           // no sleep
smart_sleep(Duration::from_secs(1)).unwrap(); // 1 second
```

## Supported Formats

### Numbers (treated as milliseconds)
- `100` → 100ms
- `500` → 500ms
- `0` → no sleep
- `-100` → no sleep

### Text with Units
- `"100ms"`, `"100 millis"`, `"100 milliseconds"`
- `"1s"`, `"1 sec"`, `"1 second"`, `"1 seconds"`
- `"1.5s"`, `"1.5 seconds"` → 1500ms
- `"2m"`, `"2 min"`, `"2 minutes"` → 120,000ms

### Duration Objects
- `Duration::from_millis(100)`
- `Duration::from_secs(1)`

## API Reference

### Main Functions

#### `smart_sleep<S: Into<SleepInput>>(input: S) -> Result<()>`

The primary function that accepts various input formats and performs sleep if needed.

#### `parse_sleep_duration(input: &str) -> Result<Duration>`

Parses a duration string into a `Duration` object.

#### `sleep(duration: Duration) -> Result<()>`

Standard sleep function for backward compatibility.

### Enum `SleepInput`

Represents different types of sleep inputs:

- `Number(isize)`
- `Text(String)`
- `Duration(Duration)`

## Examples

### Basic Usage

```rust
use sleep_utils::smart_sleep;

// Sleep for 100ms
smart_sleep(100).unwrap();

// Sleep for 2 seconds
smart_sleep("2s").unwrap();

// No sleep (zero value)
smart_sleep(0).unwrap();

// No sleep (negative value)
smart_sleep(-100).unwrap();
```

### Advanced Parsing

```rust
use sleep_utils::parse_sleep_duration;

let duration = parse_sleep_duration("1.5 minutes").unwrap();
println!("Duration: {:?}", duration); // 90 seconds
```

### Custom Sleep Logic

```rust
use sleep_utils::{SleepInput, parse_sleep_duration};

let input = SleepInput::from("500ms");
if input.should_sleep() {
    let duration = input.to_duration().unwrap();
    // Custom sleep logic
    std::thread::sleep(duration);
}
```

## Error Handling

The library uses a custom `Result<T>` type and `SleepError` enum for error handling:

```rust
use sleep_utils::{SleepError, Result};

match smart_sleep("invalid") {
    Ok(()) => println!("Sleep completed"),
    Err(SleepError::InvalidDuration(msg)) => println!("Invalid duration: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Features

- `default` (enabled by default): All features enabled
- `minimal`: Minimal feature set without complex parsing

## Performance

The library uses lazy static regex patterns for efficient parsing and avoids unnecessary allocations.

## License

Dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on GitHub.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.
