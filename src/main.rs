use colored::Colorize;
use crossterm::{cursor, execute};
use pico_args::Arguments;
use std::io::{self, Write};
use std::process;
use std::time::Instant;

use lazyfinder::i18n::strings::*;
use lazyfinder::scanner;
use lazyfinder::sensitive;
use lazyfinder::ui::{CliReporter, CursorGuard};

fn main() {
    // 启用 Windows 控制台 ANSI 支持
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let start = Instant::now();
    let mut args = Arguments::from_env();

    // 处理帮助信息
    if args.contains(["-h", "--help"]) {
        println!("{}", HELP_TITLE);
        println!("{}", HELP_USAGE);
        println!("{}", HELP_USAGE_LINE);
        println!("{}", HELP_OPTIONS);
        println!("{}", HELP_OPT_DIR);
        println!("{}", HELP_OPT_PRE);
        println!("{}", HELP_OPT_KEYS);
        println!("{}", HELP_OPT_REG);
        println!("{}", HELP_OPT_CTX);
        println!("{}", HELP_OPT_OUT);
        println!("{}", HELP_OPT_STDOUT);
        println!("{}", HELP_OPT_SENS);
        println!("{}", HELP_OPT_IGN);
        process::exit(0);
    }

    let is_sensitive = args.contains(["-s", "--sensitive"]);
    let ignore_dirs = args.contains(["-i", "--ignore"]);

    let dir: String = args.value_from_str(["-d", "--dir"]).unwrap_or_else(|_| {
        eprintln!("{}", ERR_MISSING_DIR);
        process::exit(1);
    });

    let pre_str: String = args.value_from_str(["-p", "--pre"]).unwrap_or_else(|_| {
        if is_sensitive {
            String::new() // 敏感模式下允许为空
        } else {
            eprintln!("{}", ERR_MISSING_PRE);
            process::exit(1);
        }
    });

    let keys_str: String = args.value_from_str(["-k", "--keys"]).unwrap_or_else(|_| {
        if is_sensitive {
            String::new() // 敏感模式下允许为空
        } else {
            eprintln!("{}", ERR_MISSING_KEYS);
            process::exit(1);
        }
    });

    let mut is_reg = args.contains(["-r", "--reg"]);
    
    let context = if args.contains(["-c", "--context"]) {
        let val: Result<Option<usize>, _> = args.opt_value_from_str(["-c", "--context"]);
        Some(val.unwrap_or(Some(10)).unwrap_or(10))
    } else {
        None
    };

    let use_stdout = args.contains("--stdout");
    let output_path_str: Option<String> = args.opt_value_from_str(["-o", "--output"]).unwrap_or(None);

    let saved_path = if use_stdout {
        None
    } else {
        let path = output_path_str.unwrap_or_else(|| "find.log".to_string());
        Some(std::env::current_dir().unwrap().join(&path))
    };

    let mut prefixes: Vec<String> = if pre_str.is_empty() {
        vec!["".to_string()]
    } else {
        pre_str.split(',').map(|s| s.to_string()).collect()
    };
    
    let mut keys: Vec<String> = if is_reg {
        vec![keys_str.clone()]
    } else {
        keys_str.split(',').map(|s| s.to_string()).collect()
    };

    if is_sensitive {
        sensitive::apply_sensitive_rules(&mut is_reg, &mut prefixes, &mut keys, &keys_str, &pre_str);
    }

    // 隐藏光标的守护对象
    let _guard = if !use_stdout {
        let _ = execute!(io::stdout(), cursor::Hide);
        
        // 捕获 Ctrl+C 以便恢复光标
        let _ = ctrlc::set_handler(move || {
            let _ = execute!(io::stdout(), cursor::Show);
            process::exit(130);
        });
        
        Some(CursorGuard)
    } else {
        None
    };

    // 构建引擎并运行
    let engine = scanner::FileMatcher::new(prefixes, keys, is_reg, context, ignore_dirs);
    let mut reporter = CliReporter::new(use_stdout, saved_path.clone());

    if let Err(e) = scanner::run(&dir, &engine, &mut reporter) {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            if io_err.kind() == io::ErrorKind::BrokenPipe {
                process::exit(0);
            }
        }
        eprintln!("{}: {}", ERR_PREFIX, e);
        process::exit(1);
    }

    if !use_stdout {
        // 强制最后一次绘制
        let _ = reporter.draw_ui(true);
        // 往下移动光标避免覆盖
        let _ = execute!(io::stdout(), cursor::MoveDown(1), cursor::MoveToColumn(0));
        println!("\n{} {} {} {}, {}: {} {}, {}: {:.2?}", 
            "✔".green(), 
            RES_DONE,
            reporter.scanned_count.to_string().yellow(), 
            RES_FILES,
            RES_MATCHES,
            reporter.match_count.to_string().red(),
            RES_MATCHES_SUFFIX,
            RES_TIME,
            start.elapsed()
        );
        if reporter.match_count > 0 {
            if let Some(p) = saved_path {
                println!("{} {}: {}", "💾".blue(), RES_SAVED, p.display());
            }
        } else {
            println!("{} {}", "ℹ".cyan(), RES_NO_MATCH);
        }
    } else {
        if let Err(_) = writeln!(io::stdout(), "\n{} {} {} {}, {}: {} {}, {}: {:.2?}", 
            "✔".green(), 
            RES_DONE,
            reporter.scanned_count.to_string().yellow(), 
            RES_FILES,
            RES_MATCHES,
            reporter.match_count.to_string().red(),
            RES_MATCHES_SUFFIX,
            RES_TIME,
            start.elapsed()
        ) {
            process::exit(0);
        }
    }
}
