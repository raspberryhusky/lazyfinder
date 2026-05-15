# LazyFinder

[English](#english) | [中文](#中文)

---

<a id="english"></a>
## 🇬🇧 English

**LazyFinder** is a modern, cross-platform command-line file scanning and sensitive information auditing tool written in Rust. It is designed to achieve **extremely low resource footprint, ultimate search performance**, and **an excellent terminal interactive experience**.

### ⚡ Core Features

- **Ultimate Performance (Zero-Copy)**
  - Utilizes OS-level virtual memory mapping (Memory-Mapped Files) via `memmap2`, meaning **no heap memory allocation/copying** when reading files.
  - Implements SIMD-based byte searching algorithms via `memchr::memmem`, achieving microsecond-level response times for massive codebases, completely eliminating OOM issues.
  
- **Built-in Sensitive Information Audit (`--sensitive`)**
  - Automatically targets common configurations (`.yml`, `.env`, `.properties`, etc.) and source code files (`.py`, `.java`, `.go`, etc.).
  - Contains powerful built-in RegEx rules to accurately detect hardcoded:
    - Passwords / Access Keys / Secret Keys
    - API Tokens / Bearer strings
    - JDBC URLs, phone numbers, App IDs, and RSA/DSA private keys.

- **Modern Terminal UI (`crossterm`)**
  - Features an **in-place dynamic refreshing UI** when scanning huge directories. A fixed status bar at the top displays progress, while the bottom limits output to the 5 most recent matches, preventing terminal spam.
  - **Cross-Platform**: Perfect native ANSI support for Windows (cmd/PowerShell), Linux, and macOS.
  - Gracefully handles `Ctrl+C` interruption to restore cursor visibility.

- **Flexible Output & Pipeline Support**
  - **Lazy Logging**: Matches are previewed in the terminal and also silently saved to `find.log` in the current directory. However, if no matches are found, **no empty log file is created**.
  - **Pipeline Mode (`--stdout`)**: Combine with Unix tools like `head`, `grep`, or `awk`. Appending `--stdout` downgrades the UI to a silent, continuous stream and handles `BrokenPipe` gracefully.
  - **Smart Context (`-c <N>`)**: Displays `N` surrounding characters of a match to provide context, smartly truncating at line breaks.
  - **Ignore Dirs (`-i`)**: Optionally ignore common meaningless directories like `node_modules`, `target`, and `.git` to boost speed.

### 📦 Installation & I18n

LazyFinder is extremely optimized for binary size. With `lto=true`, `opt-level="z"`, and `UPX` compression, the final binary is typically around **300KB - 800KB**.

Download the pre-compiled binaries from the [Releases](https://github.com/raspberryhusky/lazyfinder/releases) page, or build from source.

**Internationalization (I18n):**
LazyFinder natively supports bilingual UI using Rust's zero-cost compile-time features.
- Build **English** version (Default):
  ```bash
  cargo build --release
  ```
- Build **Chinese** version:
  ```bash
  cargo build --release --no-default-features --features i18n-zh
  ```

### 🚀 Usage

```text
Usage:
  lazyfinder -d <dir> -p <prefix> -k <keyword> [-r] [-c <context_chars>] [-o <output>] [--stdout] [--sensitive] [-i]

Options:
  -d, --dir      Target directory to scan
  -p, --pre      Keywords included in filenames (comma separated)
  -k, --keys     Keywords to search in file content (comma separated, or regex pattern if -r is set)
  -r, --reg      Enable regex matching mode
  -c, --context  Show context characters around the match (default: 10)
  -o, --output   Specify output log file (default: find.log in current dir)
      --stdout   Disable file logging and stream all matches directly to terminal
  -s, --sensitive Enable built-in sensitive info audit mode (applies built-in rules and extensions)
  -i, --ignore   Ignore common meaningless directories (node_modules, target, .git, etc.)
```

#### Examples

**1. Normal Search (with context)**
Search for `FileMatcher` inside all `.rs` files, showing 15 characters of context:
```bash
lazyfinder -d . -p ".rs" -k "FileMatcher" -c 15
```

**2. Code Audit: Sensitive Info Scan**
No need to specify prefixes or keywords. Just run this to audit configurations and code files for hardcoded secrets, ignoring `.git` and `target`:
```bash
lazyfinder -d . --sensitive -i
```

**3. Pipeline Integration (Stdout)**
Stream only the first 30 matches to the terminal without showing the dynamic UI or saving a log:
```bash
lazyfinder -d ~/Downloads -p "" -k "password" -c 15 --stdout -i | head -n 30
```

---

<br><br>

<a id="中文"></a>
## 🇨🇳 中文

**LazyFinder** 是一款使用 Rust 编写的现代化命令行文件扫描与敏感信息探测工具。其设计目标是 **极低的资源占用、极致的搜索性能** 以及 **极佳的终端交互体验**。

### ⚡ 核心特性

- **极致性能 (Zero-Copy)**
  - 基于 `memmap2` 库实现了操作系统的虚拟内存映射（Memory-Mapped File），获取文件内容时**无需分配堆内存进行拷贝**。
  - 使用 `memchr::memmem` 的底层 SIMD 字节查找算法，在大规模文件检索中达到微秒甚至纳秒级响应，告别内存 OOM 问题。
  
- **内置敏感信息审计模式 (`--sensitive`)**
  - 自动定位项目内的各类配置（`.yml`, `.env`, `.properties` 等）与源码文件（`.py`, `.java`, `.go` 等）。
  - 内置了强大的高危正则规则库，能够精准抓取代码中硬编码的：
    - 各类 Password / Access Key / Secret Key
    - Token、Bearer 认证串
    - JDBC URL、大陆手机号、微信小程序 ID 及 RSA/DSA 等私钥证书。

- **现代化终端 UI 交互 (`crossterm`)**
  - 扫描超大目录时提供原地的**动态刷新 UI**，顶部状态栏固定显示已扫描文件数、发现匹配数与当前路径，底部仅输出最近 5 条高亮匹配结果，不再疯狂刷屏。
  - **跨平台兼容**：完美支持 Windows (cmd/PowerShell)、Linux、macOS。
  - 智能拦截 `Ctrl+C` 信号，强制退出时完美恢复光标显示，告别异常遗留。

- **灵活的输出与管道机制**
  - **懒加载日志 (`find.log`)**：匹配结果不仅实时预览，还会将纯净结果保存至运行目录下的 `find.log`。**若未发现任何匹配，则不会创建空文件**。
  - **管道兼容 (`--stdout`)**：如果需要与其他 Unix 命令 (如 `head`, `grep`) 结合，加入 `--stdout` 即可退化为静默流式输出，并且完美处理了 Broken pipe (`head -n 10` 等截断命令不会引发 panic)。
  - **智能扩写 (`-c <N>`)**：查找到指定关键词时可展示前后 `N` 个字符的上下文信息，如独占一行则智能截断。
  - **忽略目录 (`-i`)**：智能忽略 `node_modules`、`target`、`.git` 等无意义目录，大幅提升扫描速度。

### 📦 安装与编译 (含多语言)

LazyFinder 对体积进行了极端优化。启用了 `lto = true`、`opt-level = "z"` 编译策略，并结合 GitHub Actions 中的 `UPX` 终极压缩。最终可执行文件通常在 **300KB - 800KB** 之间。

可以直接从 [Releases](https://github.com/raspberryhusky/lazyfinder/releases) 页面下载跨平台二进制文件，或者源码编译。

**国际化 (I18n) 零开销编译：**
基于 Rust 的条件编译 (Features) 机制，本工具实现了完全没有运行时开销的双语支持。
- 编译 **英文** 版 (默认):
  ```bash
  cargo build --release
  ```
- 编译 **中文** 版:
  ```bash
  cargo build --release --no-default-features --features i18n-zh
  ```

### 🚀 使用方法

```text
用法:
  lazyfinder -d <目录> -p <文件名前缀> -k <关键字> [-r] [-c <扩写字符数>] [-o <输出文件>] [--stdout] [--sensitive] [-i]

选项:
  -d, --dir      查询文件目标目录
  -p, --pre      指定文件名中包含的关键字 (逗号分隔)
  -k, --keys     指定文件内容包含的关键字 (逗号分隔，正则模式下为单个正则)
  -r, --reg      是否启用正则模式
  -c, --context  扩写选项，显示匹配内容前后的字符，默认 10 个字符
  -o, --output   指定结果输出文件 (默认保存至当前目录下的 find.log)
      --stdout   禁用文件输出，将所有结果直接打印到当前终端
  -s, --sensitive 开启内置的敏感信息扫描模式 (将自动应用内置的敏感正则规则与文件后缀)
  -i, --ignore   是否忽略常见的无意义目录 (如 node_modules, target, .git 等)，默认不忽略
```

#### 示例

**1. 常规搜索（带上下文展示）**
在根目录下搜索所有的 `.rs` 文件，查找包含 `FileMatcher` 字符串的内容，并且每次匹配前后扩写 15 个字符便于阅读上下文：
```bash
lazyfinder -d . -p ".rs" -k "FileMatcher" -c 15
```

**2. 代码审计：内置敏感信息扫描**
无需指定文件名和查找规则，自动搜索当前目录下的配置文件和代码，找出硬编码密码和 Token，并忽略垃圾目录：
```bash
lazyfinder -d . --sensitive -i
```

**3. 配合流水线（Stdout）**
配合管道符，只输出前 30 行匹配信息到终端，不保存日志，不显示动画 UI：
```bash
lazyfinder -d ~/Downloads -p "" -k "password" -c 15 --stdout -i | head -n 30
```

## 📃 License
MIT License
