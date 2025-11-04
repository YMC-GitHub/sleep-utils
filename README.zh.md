# sleep-utils

[![Crates.io](https://img.shields.io/crates/v/sleep-utils.svg)](https://crates.io/crates/sleep-utils)
[![文档](https://docs.rs/sleep-utils/badge.svg)](https://docs.rs/sleep-utils)
[![许可证](https://img.shields.io/crates/l/sleep-utils.svg)](https://github.com/ymc-github/sleep-utils#license)

一个智能的 Rust 睡眠工具库，支持灵活的输入格式和自动零值处理。

## 特性

- 🕒 **多种输入格式**: 数字、文本、`Duration` 对象
- 🚫 **自动零值/负值处理**: 零值或负值不进行睡眠
- 📏 **多种时间单位**: 毫秒、秒、分钟
- 💻 **平台兼容**: 使用 `isize` 支持跨平台
- ⚡ **高性能**: 优化的正则表达式解析
- 🎯 **智能解析**: 直观的持续时间字符串解析

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
sleep-utils = "0.1"
```

## 快速开始

```rust
use sleep_utils::smart_sleep;
use std::time::Duration;

// 多种输入格式
smart_sleep(100).unwrap();           // 100 毫秒
smart_sleep("200ms").unwrap();       // 200 毫秒
smart_sleep("1.5s").unwrap();        // 1.5 秒
smart_sleep("2 seconds").unwrap();   // 2 秒
smart_sleep(0).unwrap();             // 不睡眠
smart_sleep(-50).unwrap();           // 不睡眠
smart_sleep(Duration::from_secs(1)).unwrap(); // 1 秒
```

## 支持的格式

### 数字（默认为毫秒）
- `100` → 100ms
- `500` → 500ms
- `0` → 不睡眠
- `-100` → 不睡眠

### 带单位的文本
- `"100ms"`, `"100 millis"`, `"100 milliseconds"`
- `"1s"`, `"1 sec"`, `"1 second"`, `"1 seconds"`
- `"1.5s"`, `"1.5 seconds"` → 1500ms
- `"2m"`, `"2 min"`, `"2 minutes"` → 120,000ms

### Duration 对象
- `Duration::from_millis(100)`
- `Duration::from_secs(1)`

## API 参考

### 主要函数

#### `smart_sleep<S: Into<SleepInput>>(input: S) -> Result<()>`

主函数，接受多种输入格式并在需要时执行睡眠。

#### `parse_sleep_duration(input: &str) -> Result<Duration>`

将持续时间字符串解析为 `Duration` 对象。

#### `sleep(duration: Duration) -> Result<()>`

标准睡眠函数，用于向后兼容。

### 枚举 `SleepInput`

表示不同类型的睡眠输入：

- `Number(isize)` - 数字
- `Text(String)` - 文本
- `Duration(Duration)` - 持续时间对象

## 示例

### 基础用法

```rust
use sleep_utils::smart_sleep;

// 睡眠 100 毫秒
smart_sleep(100).unwrap();

// 睡眠 2 秒
smart_sleep("2s").unwrap();

// 不睡眠（零值）
smart_sleep(0).unwrap();

// 不睡眠（负值）
smart_sleep(-100).unwrap();
```

### 高级解析

```rust
use sleep_utils::parse_sleep_duration;

let duration = parse_sleep_duration("1.5 分钟").unwrap();
println!("持续时间: {:?}", duration); // 90 秒
```

### 自定义睡眠逻辑

```rust
use sleep_utils::{SleepInput, parse_sleep_duration};

let input = SleepInput::from("500ms");
if input.should_sleep() {
    let duration = input.to_duration().unwrap();
    // 自定义睡眠逻辑
    std::thread::sleep(duration);
}
```

## 错误处理

库使用自定义的 `Result<T>` 类型和 `SleepError` 枚举进行错误处理：

```rust
use sleep_utils::{SleepError, Result};

match smart_sleep("无效输入") {
    Ok(()) => println!("睡眠完成"),
    Err(SleepError::InvalidDuration(msg)) => println!("无效的持续时间: {}", msg),
    Err(e) => println!("其他错误: {}", e),
}
```

## 特性

- `default` (默认启用): 所有功能启用
- `minimal`: 最小功能集，不包含复杂解析

## 性能

库使用惰性静态正则表达式模式进行高效解析，避免不必要的内存分配。

## 许可证

双重许可：

- MIT 许可证 ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)
- Apache 许可证 2.0 版 ([LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0)

## 贡献

欢迎贡献！请在 GitHub 上提交 Pull Request 或开启 Issue。

## 更新日志

查看 [CHANGELOG.md](CHANGELOG.md) 了解版本历史和变更。
