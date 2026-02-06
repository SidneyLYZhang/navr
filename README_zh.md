![navr 图标](./navr.svg)

```text

▸▸ navr
```
────────────────────────

Fast directory navigation for your shell

![RUST](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![LICENSE](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)

[English](README.md) | 中文

**navr** 是一个用 Rust 编写的快速、跨平台的目录导航工具。使用快捷方式快速跳转到常用目录，打开文件管理器，并管理导航偏好设置。

## 特性

- 🚀 **快速目录跳转** - 使用简短别名快速导航到常用目录
- 📂 **文件管理器集成** - 使用偏好的文件管理器打开目录
- 🔧 **高度可配置** - 自定义快捷方式、文件管理器和工作方式
- 🖥️ **跨平台支持** - 支持 Windows、macOS 和 Linux
- 🐚 **Shell 集成** - 与 Bash、Zsh、Fish 和 PowerShell 无缝集成
- 📋 **Tab 自动补全** - 在 shell 中自动补全快捷方式
- 📤 **导入/导出** - 备份配置，简化电脑迁移
- 🎯 **模糊匹配** - 智能快捷方式匹配
- 🆕 **自动创建目录** - 按需自动创建缺失目录

## 安装

### 从源码安装

```bash
# 克隆仓库
git clone https://github.com/sidneylyzhang/navr
cd navr

# 构建并安装
cargo build --release
cargo install --path .
```

### 使用 Cargo 安装

```bash
cargo install navr
```

### 前置要求

- Rust 1.70 或更高版本
- 如需 Shell 集成：需要 Bash、Zsh、Fish 或 PowerShell

## 快速开始

```bash
# 将当前目录添加为快捷方式
navr jump --add work
# 或使用别名
j --add work

# 跳转到快捷方式
navr jump work
# 或简写
j work

# 在文件管理器中打开
navr open work
# 或使用别名
jo work

# 列出所有快捷方式
navr jump --list
```

## 命令说明

### Jump 命令 (`j`)

使用快捷方式或路径导航到目录。

```bash
navr jump [目标] [选项]

选项：
  -l, --list          列出所有快捷方式
  -a, --add <名称>    将当前目录添加为快捷方式
  -r, --remove <名称> 移除快捷方式
```

示例：
```bash
navr jump work          # 跳转到 'work' 快捷方式
j work                  # 使用别名
navr jump ~/projects    # 跳转到路径
j --add dev             # 将当前目录添加为 'dev'
j --remove old          # 移除 'old' 快捷方式
j --list                # 列出所有快捷方式
```

### Open 命令 (`o`)

在文件管理器中打开目录。

```bash
navr open [目标] [选项]

选项：
  -w, --with <管理器>  使用特定文件管理器打开
```

示例：
```bash
navr open work          # 使用默认文件管理器打开
jo work                 # 使用别名
navr open docs --with dolphin  # 使用 Dolphin 打开
```

### 快速模式

使用 `-k` 或 `--quick` 进行直接打开：

```bash
navr -k work            # 快速打开 'work' 快捷方式
```

### Config 命令 (`cfg`)

管理配置。

```bash
navr config <操作>

操作：
  show                    显示当前配置
  edit                    交互式编辑配置
  set <键> <值>          设置配置值
  get <键>               获取配置值
  reset                   重置为默认值
  set-file-manager <管理器> 设置默认文件管理器
```

示例：
```bash
navr config show
navr config set behavior.create_missing true
navr config set-file-manager dolphin
```

### Shell 命令 (`sh`)

Shell 集成和自动补全。

```bash
navr shell <操作>

操作：
  complete <shell>        生成自动补全脚本
  install <shell>         安装 shell 集成
  init <shell>            打印初始化脚本
```

示例：
```bash
# 生成自动补全
navr shell complete bash > /etc/bash_completion.d/navr

# 安装 shell 集成
navr shell install bash
navr shell install zsh
navr shell install fish

# 打印初始化脚本用于手动安装
navr shell init bash
```

### 导出/导入 (`exp`/`imp`)

备份和恢复配置。

```bash
# 导出配置
navr export --format toml --output backup.toml
navr export --format json > backup.json

# 导入配置
navr import backup.toml
navr import backup.json --merge  # 与现有配置合并
```

## 配置

配置文件存储在：

- **Windows**: `%APPDATA%\quicknav\config.toml`
- **macOS**: `~/Library/Application Support/quicknav/config.toml`
- **Linux**: `~/.config/quicknav/config.toml`

### 配置示例

```toml
version = "1.0"
default_file_manager = "dolphin"

[shortcuts]
home = "/home/user"
dev = "/home/user/development"
work = "/home/user/work"

[shell]
enabled = true
hook_cd = true
track_history = true
max_history = 1000
completion_style = "fuzzy"

[behavior]
confirm_overwrite = true
create_missing = false
follow_symlinks = true
case_sensitive = false
default_to_home = true

[platform.linux]
desktop_env = "kde"
file_manager = "dolphin"
terminal = "kitty"

[platform.windows]
use_windows_terminal = true
use_powershell_aliases = true

[platform.macos]
use_finder = true
prefer_iterm2 = false
```

## Shell 集成

Navr 提供深度 shell 集成以增强工作流程。

### Bash

```bash
# 添加到 ~/.bashrc
eval "$(navr shell init bash)"
```

### Zsh

```bash
# 添加到 ~/.zshrc
eval "$(navr shell init zsh)"
```

### Fish

```fish
# 添加到 ~/.config/fish/config.fish
navr shell init fish | source
```

### PowerShell

```powershell
# 添加到 $PROFILE
navr shell init powershell | Invoke-Expression
```

### 可用别名

安装 Shell 集成后，可以使用以下便捷别名：

| 别名 | 命令 | 描述 |
|------|---------|-------------|
| `j` | `navr jump` | 跳转到快捷方式 |
| `jo` | `navr open` | 在文件管理器中打开 |
| `jl` | `navr jump --list` | 列出快捷方式 |
| `cfg` | `navr config` | 配置管理 |
| `sh` | `navr shell` | Shell 集成 |
| `exp` | `navr export` | 导出配置 |
| `imp` | `navr import` | 导入配置 |

## 默认快捷方式

Navr 为常用目录提供了合理的默认设置：

| 快捷方式 | 目录 |
|----------|-----------|
| `home`, `~`, `h` | 主目录 |
| `desktop`, `desk` | 桌面 |
| `docs`, `documents` | 文档 |
| `downloads`, `dl` | 下载 |
| `pictures`, `pics` | 图片 |
| `music` | 音乐 |
| `videos` | 视频 |
| `config`, `cfg` | 配置目录 |
| `dev` | ~/dev（如果存在） |
| `projects`, `proj` | ~/projects（如果存在） |
| `workspace`, `ws` | ~/workspace（如果存在） |
| `repos` | ~/repos（如果存在） |
| `github`, `gh` | ~/github（如果存在） |

## 支持的文件管理器

### Windows
- Explorer（默认）
- Total Commander
- Double Commander
- Files
- OneCommander

### macOS
- Finder（默认）
- Path Finder
- ForkLift
- Commander One

### Linux
- xdg-open（默认）
- Nautilus（GNOME）
- Dolphin（KDE）
- Thunar（XFCE）
- PCManFM（LXDE/LXQt）
- Nemo（Cinnamon）
- Caja（MATE）
- Ranger（终端）
- Vifm（终端）
- Midnight Commander

## 构建

```bash
# 调试构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

## 开发

### 项目结构

```
navr/
├── Cargo.toml
├── README.md
├── QUICKSTART.md
├── ARCHITECTURE.md
└── src/
    ├── main.rs              # CLI 入口
    ├── config/              # 配置管理
    │   ├── mod.rs
    │   ├── defaults.rs
    │   └── tests.rs
    ├── commands/            # 命令实现
    │   ├── mod.rs
    │   ├── jump.rs
    │   ├── open.rs
    │   ├── config.rs
    │   ├── export.rs
    │   └── import.rs
    ├── platform/            # 平台相关代码
    │   ├── mod.rs
    │   └── file_manager.rs
    └── shell/               # Shell 集成
        ├── mod.rs
        ├── completions.rs
        ├── integration.rs
        └── shell_integration.rs
```

## 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 许可证

本项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- 使用 [clap](https://github.com/clap-rs/clap) 构建 CLI
- 使用 [serde](https://github.com/serde-rs/serde) 进行配置管理
- 使用 [anyhow](https://github.com/dtolnay/anyhow) 进行错误处理
- 使用 [owo-colors](https://github.com/jam1garner/owo-colors) 进行终端着色
