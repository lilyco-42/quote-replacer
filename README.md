# Quote Replacer

A cross-platform character replacement tool with GUI, supports Chinese/English punctuation conversion and keyboard hook real-time replacement.

![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Platform](https://img.shields.io/badge/platform-macOS-red)
![Platform](https://img.shields.io/badge/platform-Linux-orange)
![Platform](https://img.shields.io/badge/platform-Android-green)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/License-MIT-green)

## Features

- **Text Replacement Tool**: Input text and convert to target format with one click
- **Character Mapping Config**: Customizable character mapping rules, add/delete/reset
- **Keyboard Hook**: After enabling, hold Alt + corresponding key to auto-replace
  - `Alt + ,` → `,`
  - `Alt + .` → `.`
  - `Alt + /` → `/`
  - `Alt + ;` → `;`
  - `Alt + -` → `-`
  - `Alt + =` → `=`
  - `Alt + \` → `\`
  - `Alt + \` → `` ` ``
  - `Alt + '` → `'`

## Default Mappings

| From | To |
|------|-----|
| 「 | " |
| 」 | " |
| 『 | ' |
| 『 | ' |
| （ | ( |
| ） | ) |
| 《 | < |
| 》 | > |
| ， | , |
| 。 | . |
| 、 | , |
| ； | ; |
| ： | : |
| ？ | ? |
| ！ | ! |
| …… | ... |
| — | - |
| ～ | ~ |
| · | ` |

## Installation

### Pre-built

Download from [Releases](https://github.com/lilyco-42/quote-replacer/releases)

### Build from Source

```bash
# Clone
git clone https://github.com/lilyco-42/quote-replacer.git
cd quote-replacer

# Build (Windows/Linux/macOS)
cargo build --release

# Or build for Android (requires Android SDK)
cargo apk build --release

# Run
./target/release/quote_replacer
```

## Usage

### Replace Tool

1. Open app, default shows "替换工具" tab
2. Enter text in input box
3. Click "全部替换" (Replace All)
4. Click "复制结果" (Copy Result)

### Mapping Config

1. Click "映射配置" tab
2. View all mappings
3. Click "添加" to add new mapping
4. Click "删除" to remove mapping
5. Click "重置为默认" to restore defaults

### Keyboard Hook (Windows only)

1. Click "开启" in toolbar
2. Hold Alt + key to auto-replace

> Note: Keyboard hook requires running as Administrator on Windows.

## Config File

Mappings saved to:
- Windows: `%APPDATA%\quote_replacer\mappings.json`
- macOS: `~/Library/Application Support/quote_replacer/mappings.json`
- Linux: `~/.config/quote_replacer/mappings.json`

## Tech Stack

- [eframe](https://github.com/emilk/egui) - Pure Rust GUI
- [enigo](https://github.com/whitelynx/enigo) - Keyboard/Mouse simulation
- [windows-rs](https://github.com/microsoft/windows-rs) - Windows API
- [cargo-apk](https://github.com/nicbarker/cargo-apk) - Android APK build

## License

MIT License