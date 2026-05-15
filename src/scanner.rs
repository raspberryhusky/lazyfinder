use colored::Colorize;
use ignore::WalkBuilder;
use memchr::memmem;
use memmap2::MmapOptions;
use regex::bytes::Regex;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::utils::{count_lines, extract_context};

pub trait Reporter {
    fn on_file_scanned(&mut self, path: &str);
    fn on_file_match_start(&mut self, path: &str);
    fn on_match(&mut self, line_num: usize, raw_line: &str, colored_line: &str);
}

pub struct FileMatcher {
    pub prefixes: Vec<String>,
    pub keys: Vec<String>,
    pub is_reg_word: bool,
    pub context: Option<usize>,
    pub regex_keys: Option<Vec<Regex>>,
    pub ignore_dirs: bool,
}

impl FileMatcher {
    pub fn new(
        prefixes: Vec<String>,
        keys: Vec<String>,
        is_reg_word: bool,
        context: Option<usize>,
        ignore_dirs: bool,
    ) -> Self {
        let regex_keys = if is_reg_word {
            let mut r_keys = Vec::with_capacity(keys.len());
            for k in &keys {
                match Regex::new(k) {
                    Ok(re) => r_keys.push(re),
                    Err(e) => {
                        eprintln!("警告: 正则表达式解析失败: {} \n正则: {}", e, k);
                    }
                }
            }
            Some(r_keys)
        } else {
            None
        };

        FileMatcher {
            prefixes,
            keys,
            is_reg_word,
            context,
            regex_keys,
            ignore_dirs,
        }
    }
}

pub fn process_file<R: Reporter>(
    path: &Path,
    engine: &FileMatcher,
    reporter: &mut R,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    // 跳过空文件
    if file.metadata()?.len() == 0 {
        return Ok(());
    }

    // Zero-copy: 尝试使用 memmap 映射文件到内存
    let mmap = unsafe { MmapOptions::new().map(&file) };
    let content = match &mmap {
        Ok(m) => &m[..],
        Err(_) => return Ok(()), // 映射失败（如设备文件等）忽略
    };

    let mut printed_path = false;
    let path_str = path.to_string_lossy();

    if engine.is_reg_word {
        if let Some(regexes) = &engine.regex_keys {
            for re in regexes {
                for m in re.find_iter(content) {
                    if !printed_path {
                        reporter.on_file_match_start(&path_str);
                        printed_path = true;
                    }
                    let line_num = count_lines(content, m.start());

                    let display_bytes = if let Some(ctx_len) = engine.context {
                        extract_context(content, m.start(), m.end(), ctx_len)
                    } else {
                        m.as_bytes()
                    };

                    let display_str = String::from_utf8_lossy(display_bytes);
                    let match_str = String::from_utf8_lossy(m.as_bytes());

                    // 高亮匹配部分
                    let highlighted =
                        display_str.replace(&*match_str, &match_str.green().to_string());
                    reporter.on_match(line_num, &display_str, &highlighted);
                }
            }
        }
    } else {
        for key in &engine.keys {
            let key_bytes = key.as_bytes();
            for m_start in memmem::find_iter(content, key_bytes) {
                if !printed_path {
                    reporter.on_file_match_start(&path_str);
                    printed_path = true;
                }
                let m_end = m_start + key_bytes.len();
                let line_num = count_lines(content, m_start);

                let display_bytes = if let Some(ctx_len) = engine.context {
                    extract_context(content, m_start, m_end, ctx_len)
                } else {
                    key_bytes
                };

                let display_str = String::from_utf8_lossy(display_bytes);
                let match_str = String::from_utf8_lossy(key_bytes);

                let highlighted = display_str.replace(&*match_str, &match_str.green().to_string());
                reporter.on_match(line_num, &display_str, &highlighted);
            }
        }
    }

    Ok(())
}

pub fn run<R: Reporter>(
    dir: &str,
    engine: &FileMatcher,
    reporter: &mut R,
) -> Result<(), Box<dyn Error>> {
    let mut builder = WalkBuilder::new(dir);
    
    if engine.ignore_dirs {
        builder.hidden(true)
               .ignore(true)
               .git_ignore(true)
               .filter_entry(|entry| {
                   if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                       let name = entry.file_name().to_string_lossy();
                       !crate::sensitive::COMMON_IGNORE_DIRS.contains(&name.as_ref())
                   } else {
                       true
                   }
               });
    } else {
        builder.hidden(false)
               .ignore(false)
               .git_ignore(false);
    }
    
    let walker = builder.build();

    for result in walker {
        if let Ok(entry) = result {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let path = entry.path();
                let path_str = path.to_string_lossy();
                reporter.on_file_scanned(&path_str);

                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                // 检查前缀
                let mut matched_prefix = false;
                for pre in &engine.prefixes {
                    if file_name.contains(pre.as_str()) {
                        matched_prefix = true;
                        break;
                    }
                }

                if matched_prefix {
                    if let Err(e) = process_file(path, engine, reporter) {
                        // 抛出 IO 错误以供外层检测
                        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                            if io_err.kind() == std::io::ErrorKind::BrokenPipe {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
