pub const COMMON_IGNORE_DIRS: &[&str] = &[
    // 版本控制与 IDE
    ".git", ".svn", ".idea", ".vscode", ".settings",
    // 依赖与包管理
    "node_modules", "vendor", "bower_components", "packages",
    // 编译输出与缓存
    "target", "build", "dist", "out", "bin", "obj", "__pycache__", ".cache", ".next", ".nuxt",
    // 虚拟环境
    "venv", ".venv", "env", ".env_dir",
    // 日志与临时文件
    "logs", "tmp", "temp", "coverage"
];

pub fn apply_sensitive_rules(
    is_reg: &mut bool,
    prefixes: &mut Vec<String>,
    keys: &mut Vec<String>,
    keys_str: &str,
    pre_str: &str,
) {
    // 开启敏感信息模式时强制覆盖为正则模式
    *is_reg = true;
    
    // 添加常见配置文件与代码后缀
    let sens_prefixes = vec![
        ".yml", ".yaml", ".properties", ".json", ".conf", ".xml", ".ini", ".env", ".sh", ".bash",
        ".py", ".java", ".go", ".js", ".ts", ".php", ".rb", ".txt", ".sql"
    ];
    if pre_str.is_empty() {
        *prefixes = sens_prefixes.into_iter().map(|s| s.to_string()).collect();
    } else {
        prefixes.extend(sens_prefixes.into_iter().map(|s| s.to_string()));
    }

    // 内置高危敏感信息正则
    let sens_keys = vec![
        r"(?i)(password|passwd|pwd)[\s]*[:=][\s]*['\x22]?[a-zA-Z0-9!@#$%^&*()_+]{6,}['\x22]?", // 密码
        r"(?i)(ak|access_key|accesskey|aws_access_key_id)[\s]*[:=][\s]*['\x22]?[A-Za-z0-9]{16,40}['\x22]?", // Access Key
        r"(?i)(sk|secret_key|secretkey|aws_secret_access_key)[\s]*[:=][\s]*['\x22]?[A-Za-z0-9/+=]{30,60}['\x22]?", // Secret Key
        r"(?i)(token|api_token|access_token|bearer_token)[\s]*[:=][\s]*['\x22]?[A-Za-z0-9\-\._]{20,}['\x22]?", // Tokens
        r"(?i)jdbc:mysql://[a-zA-Z0-9\-\.]+:[0-9]{1,5}/[a-zA-Z0-9_]+", // JDBC URL
        r"1[3-9]\d{9}", // 大陆手机号
        r"(?i)wx[a-f0-9]{16}", // 微信小程序 ID
        r"-----BEGIN (RSA |DSA |EC |OPENSSH |PRIVATE )?PRIVATE KEY-----", // 私钥
    ];
    
    let combined_sens_regex = sens_keys.join("|");
    
    if keys_str.is_empty() {
        *keys = vec![combined_sens_regex];
    } else {
        // 如果用户自带了 keys，将其并入大正则
        let user_reg = if *is_reg { keys_str.to_string() } else { regex::escape(&keys_str) };
        *keys = vec![format!("{}|{}", user_reg, combined_sens_regex)];
    }
}
