use colored::Colorize;
use crossterm::{cursor, execute};
use pico_args::Arguments;
use std::io::{self, Write};
use std::process;
use std::time::Instant;

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
        println!("lazyfinder - 快速查询，极致性能");
        println!("用法:");
        println!("  lazyfinder -d <目录> -p <文件名前缀> -k <关键字> [-r] [-c <扩写字符数>] [-o <输出文件>] [--stdout] [--sensitive] [-i]");
        println!("\n选项:");
        println!("  -d, --dir      查询文件目标目录");
        println!("  -p, --pre      指定文件名中包含的关键字 (逗号分隔)");
        println!("  -k, --keys     指定文件内容包含的关键字 (逗号分隔，正则模式下为单个正则)");
        println!("  -r, --reg      是否启用正则模式");
        println!("  -c, --context  扩写选项，显示匹配内容前后的字符，默认 10 个字符");
        println!("  -o, --output   指定结果输出文件 (默认保存至当前目录下的 find.log)");
        println!("      --stdout   禁用文件输出，将所有结果直接打印到当前终端");
        println!("  -s, --sensitive 开启内置的敏感信息扫描模式 (将自动应用内置的敏感正则规则与文件后缀)");
        println!("  -i, --ignore   是否忽略常见的无意义目录 (如 node_modules, target, .git 等)，默认不忽略");
        process::exit(0);
    }

    let is_sensitive = args.contains(["-s", "--sensitive"]);
    let ignore_dirs = args.contains(["-i", "--ignore"]);

    let dir: String = args.value_from_str(["-d", "--dir"]).unwrap_or_else(|_| {
        eprintln!("错误: 缺少 -d / --dir 参数");
        process::exit(1);
    });

    let pre_str: String = args.value_from_str(["-p", "--pre"]).unwrap_or_else(|_| {
        if is_sensitive {
            String::new() // 敏感模式下允许为空
        } else {
            eprintln!("错误: 缺少 -p / --pre 参数");
            process::exit(1);
        }
    });

    let keys_str: String = args.value_from_str(["-k", "--keys"]).unwrap_or_else(|_| {
        if is_sensitive {
            String::new() // 敏感模式下允许为空
        } else {
            eprintln!("错误: 缺少 -k / --keys 参数");
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
        eprintln!("错误: {}", e);
        process::exit(1);
    }

    if !use_stdout {
        // 强制最后一次绘制
        let _ = reporter.draw_ui(true);
        // 往下移动光标避免覆盖
        let _ = execute!(io::stdout(), cursor::MoveDown(1), cursor::MoveToColumn(0));
        println!("\n{} 扫描完成！共扫描了 {} 个文件，发现匹配: {} 处，耗时: {:.2?}", 
            "✔".green(), 
            reporter.scanned_count.to_string().yellow(), 
            reporter.match_count.to_string().red(),
            start.elapsed()
        );
        if reporter.match_count > 0 {
            if let Some(p) = saved_path {
                println!("{} 结果已保存至: {}", "💾".blue(), p.display());
            }
        } else {
            println!("{} 未发现任何匹配项，未生成日志文件。", "ℹ".cyan());
        }
    } else {
        if let Err(_) = writeln!(io::stdout(), "\n{} 扫描完成！共扫描了 {} 个文件，发现匹配: {} 处，耗时: {:.2?}", 
            "✔".green(), 
            reporter.scanned_count.to_string().yellow(), 
            reporter.match_count.to_string().red(),
            start.elapsed()
        ) {
            process::exit(0);
        }
    }
}
