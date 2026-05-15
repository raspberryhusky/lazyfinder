// src/i18n.rs

#[cfg(feature = "i18n-zh")]
pub mod strings {
    pub const HELP_TITLE: &str = "lazyfinder - 快速查询，极致性能";
    pub const HELP_USAGE: &str = "用法:";
    pub const HELP_USAGE_LINE: &str = "  lazyfinder -d <目录> -p <文件名前缀> -k <关键字> [-r] [-c <扩写字符数>] [-o <输出文件>] [--stdout] [--sensitive] [-i]";
    pub const HELP_OPTIONS: &str = "\n选项:";
    pub const HELP_OPT_DIR: &str = "  -d, --dir      查询文件目标目录";
    pub const HELP_OPT_PRE: &str = "  -p, --pre      指定文件名中包含的关键字 (逗号分隔)";
    pub const HELP_OPT_KEYS: &str = "  -k, --keys     指定文件内容包含的关键字 (逗号分隔，正则模式下为单个正则)";
    pub const HELP_OPT_REG: &str = "  -r, --reg      是否启用正则模式";
    pub const HELP_OPT_CTX: &str = "  -c, --context  扩写选项，显示匹配内容前后的字符，默认 10 个字符";
    pub const HELP_OPT_OUT: &str = "  -o, --output   指定结果输出文件 (默认保存至当前目录下的 find.log)";
    pub const HELP_OPT_STDOUT: &str = "      --stdout   禁用文件输出，将所有结果直接打印到当前终端";
    pub const HELP_OPT_SENS: &str = "  -s, --sensitive 开启内置的敏感信息扫描模式 (将自动应用内置的敏感正则规则与文件后缀)";
    pub const HELP_OPT_IGN: &str = "  -i, --ignore   是否忽略常见的无意义目录 (如 node_modules, target, .git 等)，默认不忽略";

    pub const ERR_MISSING_DIR: &str = "错误: 缺少 -d / --dir 参数";
    pub const ERR_MISSING_PRE: &str = "错误: 缺少 -p / --pre 参数";
    pub const ERR_MISSING_KEYS: &str = "错误: 缺少 -k / --keys 参数";
    pub const ERR_CREATE_FILE: &str = "无法创建输出文件";
    pub const ERR_PREFIX: &str = "错误";

    pub const UI_SCANNING: &str = "正在扫描...";
    pub const UI_SCANNED_FILES: &str = "已扫描文件数";
    pub const UI_FOUND_MATCHES: &str = "发现匹配数";
    pub const UI_CURRENT_FILE: &str = "当前文件";
    
    pub const RES_DONE: &str = "扫描完成！共扫描了";
    pub const RES_FILES: &str = "个文件";
    pub const RES_MATCHES: &str = "发现匹配";
    pub const RES_MATCHES_SUFFIX: &str = "处";
    pub const RES_TIME: &str = "耗时";
    pub const RES_SAVED: &str = "结果已保存至";
    pub const RES_NO_MATCH: &str = "未发现任何匹配项，未生成日志文件。";
}

#[cfg(not(feature = "i18n-zh"))]
pub mod strings {
    // ... 前面略 ...
    pub const HELP_TITLE: &str = "lazyfinder - Fast search, ultimate performance";
    pub const HELP_USAGE: &str = "Usage:";
    pub const HELP_USAGE_LINE: &str = "  lazyfinder -d <dir> -p <prefix> -k <keyword> [-r] [-c <context_chars>] [-o <output>] [--stdout] [--sensitive] [-i]";
    pub const HELP_OPTIONS: &str = "\nOptions:";
    pub const HELP_OPT_DIR: &str = "  -d, --dir      Target directory to scan";
    pub const HELP_OPT_PRE: &str = "  -p, --pre      Keywords included in filenames (comma separated)";
    pub const HELP_OPT_KEYS: &str = "  -k, --keys     Keywords to search in file content (comma separated, or regex pattern if -r is set)";
    pub const HELP_OPT_REG: &str = "  -r, --reg      Enable regex matching mode";
    pub const HELP_OPT_CTX: &str = "  -c, --context  Show context characters around the match (default: 10)";
    pub const HELP_OPT_OUT: &str = "  -o, --output   Specify output log file (default: find.log in current dir)";
    pub const HELP_OPT_STDOUT: &str = "      --stdout   Disable file logging and stream all matches directly to terminal";
    pub const HELP_OPT_SENS: &str = "  -s, --sensitive Enable built-in sensitive info audit mode (applies built-in rules and extensions)";
    pub const HELP_OPT_IGN: &str = "  -i, --ignore   Ignore common meaningless directories (node_modules, target, .git, etc.)";

    pub const ERR_MISSING_DIR: &str = "Error: Missing -d / --dir parameter";
    pub const ERR_MISSING_PRE: &str = "Error: Missing -p / --pre parameter";
    pub const ERR_MISSING_KEYS: &str = "Error: Missing -k / --keys parameter";
    pub const ERR_CREATE_FILE: &str = "Cannot create output file";
    pub const ERR_PREFIX: &str = "Error";

    pub const UI_SCANNING: &str = "Scanning...";
    pub const UI_SCANNED_FILES: &str = "Scanned files";
    pub const UI_FOUND_MATCHES: &str = "Matches found";
    pub const UI_CURRENT_FILE: &str = "Current file";
    
    pub const RES_DONE: &str = "Scan complete! Scanned";
    pub const RES_FILES: &str = "files";
    pub const RES_MATCHES: &str = "Matches";
    pub const RES_MATCHES_SUFFIX: &str = "found";
    pub const RES_TIME: &str = "Time";
    pub const RES_SAVED: &str = "Results saved to";
    pub const RES_NO_MATCH: &str = "No matches found. No log file generated.";
}
