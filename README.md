# Quote Replacer

一个用于字符替换的桌面工具，支持中英文标点符号互转、键盘Hook实时替换功能。

![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/License-MIT-green)

## 功能特性

- **文本替换工具**：输入文本，一键转换为目标格式
- **字符映射配置**：可自定义字符映射规则，添加/删除/重置
- **键盘Hook**：开启后，在任意应用中按住 Alt + 对应键可自动替换为映射字符
  - `Alt + ,` → `,`
  - `Alt + .` → `.`
  - `Alt + /` → `/`
  - `Alt + ;` → `;`
  - `Alt + -` → `-`
  - `Alt + =` → `=`
  - `Alt + \` → `\`
  - `Alt + \` → `` ` ``
  - `Alt + '` → `'`

## 默认映射规则

| 原字符 | 替换为 |
|--------|--------|
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

## 安装

### 预构建版本

从 [Releases](https://github.com/anomalyco/quote-replacer/releases) 下载最新版本的 `quote_replacer.exe`

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/anomalyco/quote-replacer.git
cd quote-replacer

# 构建
cargo build --release

# 运行
./target/release/quote_replacer.exe
```

## 使用说明

### 替换工具

1. 打开应用，默认显示"替换工具"标签页
2. 在左侧文本框输入要替换的文本
3. 点击"全部替换"按钮
4. 点击"复制结果"复制到剪贴板

### 映射配置

1. 点击"映射配置"标签页
2. 可查看当前所有映射规则
3. 点击"添加"添加新映射，输入原字符和替换值
4. 点击"删除"删除不需要的映射
5. 点击"重置为默认"恢复内置默认规则

### 键盘Hook

1. 在顶部工具栏点击"开启"按钮启用Hook
2. 按住 `Alt` 键并按键盘上的特定按键
3. 会自动输入对应的替换字符

> 注意：键盘Hook功能需要以管理员权限运行程序。

## 配置文件

映射配置保存在：
- Windows: `%APPDATA%\quote_replacer\mappings.json`

## 技术栈

- [eframe](https://github.com/emilk/egui) - Pure Rust GUI
- [enigo](https://github.com/whitelynx/enigo) - 键盘/鼠标模拟
- [windows-rs](https://github.com/microsoft/windows-rs) - Windows API

## 许可证

MIT License