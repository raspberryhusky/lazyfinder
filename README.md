# LazyFinder

**LazyFinder** 是一款使用 Rust 编写的现代化命令行文件扫描与敏感信息探测工具。其设计目标是 **极低的资源占用、极致的搜索性能** 以及 **极佳的终端交互体验**。

## ⚡ 核心特性

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
  - 扫描超大目录时提供原地的**动态刷新 UI**，顶部状态栏固定显示已扫描文件数与当前路径，底部仅输出最近 5 条高亮匹配结果，不再疯狂刷屏。
  - **跨平台兼容**：完美支持 Windows (cmd/PowerShell)、Linux、macOS。
  - 智能拦截 `Ctrl+C` 信号，强制退出时完美恢复光标显示，告别异常遗留。

- **灵活的输出管道机制**
  - **默认持久化**：默认情况下扫描结果不仅实时预览，还会将不带 ANSI 转码的纯净结果自动保存至运行目录下的 `find.log`。
  - **管道兼容 (`--stdout`)**：如果需要与其他 Unix 命令 (如 `head`, `grep`, `awk`) 结合，加入 `--stdout` 即可退化为静默流式输出，并且完美处理了 Broken pipe (`head -n 10` 等截断命令不会引发 panic)。
  - **智能扩写 (`-c <N>`)**：查找到指定关键词时可展示前后 `N` 个字符的上下文信息，如独占一行则智能截断。

## 📦 安装与编译

LazyFinder 对体积进行了极端优化，移除了一切不必要的重量级库，并启用了 `lto = true`、`opt-level = "z"` 和去除符号表等编译策略。最终可执行文件通常在 `800KB` 左右。

```bash
cargo build --release
```

编译产物位于 `target/release/lazyfinder`。

## 🚀 使用方法

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

### 示例

**1. 常规搜索（带上下文展示）**
在根目录下搜索所有的 `.rs` 文件，查找包含 `FileMatcher` 字符串的内容，并且每次匹配前后扩写 15 个字符便于阅读上下文：
```bash
lazyfinder -d . -p ".rs" -k "FileMatcher" -c 15
```

**2. 代码审计：内置敏感信息扫描**
无需指定文件名和查找规则，自动搜索当前目录下的配置文件和代码，找出硬编码密码和 Token：
```bash
lazyfinder -d . --sensitive
```

**3. 配合流水线（Stdout）**
配合管道符，只输出前 30 行匹配信息到终端，不保存日志，不显示动画 UI：
```bash
lazyfinder -d ~/Downloads -p "" -k "password" -c 15 --stdout | head -n 30
```

## 🏗 代码架构

遵循良好的软件工程原则，核心功能被拆分至清晰的模块中：
- `main.rs`: 入口点、参数解析与流程组装。
- `ui.rs`: 基于 `crossterm` 的高性能终端控制与动态 UI 绘制。
- `scanner.rs`: 文件系统遍历（使用 `ignore` 库）、Zero-Copy 内存映射及 SIMD 字符串匹配引擎。
- `sensitive.rs`: 内置敏感文件后缀与高危正则表达式规则库。
- `utils.rs`: 处理上下文扩写、安全截断等通用功能。

## 📃 License
MIT License
