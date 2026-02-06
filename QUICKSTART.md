# Navr 快速开始指南

5 分钟内上手 Navr！

## 安装

### 方式 1：使用安装脚本（推荐）

```bash
curl -sSL https://raw.githubusercontent.com/sidneylyzhang/navr/main/install.sh | bash
```

### 方式 2：从源码构建

```bash
git clone https://github.com/sidneylyzhang/navr
cd navr
cargo build --release
sudo cp target/release/navr /usr/local/bin/
```

### 方式 3：使用 Cargo

```bash
cargo install navr
```

## 初始设置

### 1. 添加 Shell 集成

将以下内容添加到您的 Shell 配置文件中：

**Bash** (`~/.bashrc`):
```bash
eval "$(navr shell init bash)"
```

**Zsh** (`~/.zshrc`):
```bash
eval "$(navr shell init zsh)"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
navr shell init fish | source
```

**PowerShell** (`$PROFILE`):
```powershell
navr shell init powershell | Invoke-Expression
```

### 2. 重新加载 Shell

```bash
source ~/.bashrc  # 或 .zshrc 等
```

## 基本用法

### 添加快捷方式

```bash
# 进入您常用的目录
cd ~/projects/my-awesome-project

# 添加为快捷方式
navr jump --add awesome
# 或使用别名
j --add awesome
```

### 跳转到快捷方式

```bash
# 使用快捷方式跳转
j awesome

# 或使用完整命令
navr jump awesome
```

### 列出所有快捷方式

```bash
j --list
# 或使用别名
jl  # 如果已设置别名
```

### 在文件管理器中打开

```bash
# 使用默认文件管理器打开快捷方式
jo awesome

# 指定文件管理器
navr open awesome --with dolphin
```

## 快速模式

使用 `-k` 或 `--quick` 标志直接打开：

```bash
navr -k awesome
```

适用于脚本和快速访问。

## 配置

### 查看配置

```bash
navr config show
# 或
cfg  # 别名
```

### 设置默认文件管理器

```bash
navr config set-file-manager dolphin
```

### 交互式编辑配置

```bash
navr config edit
```

### 手动配置

直接编辑配置文件：

- **Linux/macOS**: `~/.config/quicknav/config.toml`
- **Windows**: `%APPDATA%\quicknav\config.toml`

示例配置：
```toml
default_file_manager = "dolphin"

[shortcuts]
home = "/home/username"
work = "/home/username/work"
projects = "/home/username/projects"
```

## 命令别名

安装 Shell 集成后，您可以使用以下别名：

| 别名 | 命令 | 描述 |
|------|---------|-------------|
| `j` | `navr jump` | 跳转到快捷方式 |
| `jo` | `navr open` | 在文件管理器中打开 |
| `jl` | `navr jump --list` | 列出快捷方式 |
| `cfg` | `navr config` | 配置管理 |
| `sh` | `navr shell` | Shell 集成 |

## Tab 自动补全

Navr 为快捷方式提供 Tab 自动补全：

```bash
j wo<TAB>    # 如果有 'work' 快捷方式，会自动补全
j pro<TAB>   # 补全为 'projects'
```

## 实用技巧

### 1. 模糊匹配

快捷方式默认支持模糊匹配：

```bash
j wo      # 匹配 'work', 'workspace' 等
```

### 2. 路径扩展

可以将快捷方式与路径结合使用：

```bash
j awesome/src
```

### 3. 自动创建目录

在配置中启用：
```toml
[behavior]
create_missing = true
```

然后：
```bash
j newproject  # 如果目录不存在，会自动创建
```

### 4. 导入/导出配置

备份配置：
```bash
navr export --format toml --output backup.toml
# 或
exp --format json > backup.json
```

在另一台机器上恢复：
```bash
navr import backup.toml
# 或
imp backup.toml
```

### 5. 与其他工具结合使用

与 `ls` 结合：
```bash
j awesome && ls -la
```

在脚本中使用：
```bash
#!/bin/bash
PROJECT_DIR=$(navr jump awesome)
cd "$PROJECT_DIR"
# 执行其他操作...
```

## 默认快捷方式

Navr 预置了常用目录的快捷方式：

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

开发相关（如果目录存在）：
| `dev` | ~/dev |
| `projects`, `proj` | ~/projects |
| `workspace`, `ws` | ~/workspace |
| `repos` | ~/repos |
| `github`, `gh` | ~/github |

## 故障排除

### 命令未找到

确保二进制文件在 PATH 中：
```bash
which navr
```

如果不在，添加到 PATH：
```bash
export PATH="$PATH:/path/to/navr"
```

### Shell 集成不工作

1. 确认已重新加载 Shell 配置
2. 验证初始化命令是否正常工作：
   ```bash
   navr shell init bash
   ```
3. 检查 Shell 配置中的错误

### 快捷方式无法解析

1. 检查快捷方式是否存在：
   ```bash
   j --list
   ```
2. 验证路径是否有效：
   ```bash
   navr config get shortcuts.<name>
   ```

## 获取帮助

- 查看帮助：`navr --help`
- 查看命令帮助：`navr <command> --help`
- 提交 Issue：https://github.com/sidneylyzhang/navr/issues
- 完整文档：[README.md](README.md)
