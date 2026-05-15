use colored::Colorize;
use crossterm::{
    cursor, execute, queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::utils::truncate_to_width;
use crate::i18n::strings::*;

pub struct CursorGuard;
impl Drop for CursorGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show);
    }
}

pub struct CliReporter {
    pub use_stdout: bool,
    pub output_file: Option<File>,
    pub output_path: Option<std::path::PathBuf>,
    pub scanned_count: usize,
    pub match_count: usize,
    pub recent_matches: VecDeque<String>,
    pub last_update: Instant,
    pub stdout: io::Stdout,
    pub current_file: String,
    pub ui_initialized: bool,
}

impl CliReporter {
    pub fn new(use_stdout: bool, output_path: Option<std::path::PathBuf>) -> Self {
        Self {
            use_stdout,
            output_file: None,
            output_path,
            scanned_count: 0,
            match_count: 0,
            recent_matches: VecDeque::with_capacity(5),
            last_update: Instant::now(),
            stdout: io::stdout(),
            current_file: String::new(),
            ui_initialized: false,
        }
    }

    pub fn draw_ui(&mut self, force: bool) -> io::Result<()> {
        if self.use_stdout {
            return Ok(());
        }

        let now = Instant::now();
        if !force && now.duration_since(self.last_update) < Duration::from_millis(50) {
            return Ok(());
        }
        self.last_update = now;

        let (term_width, _) = terminal::size().unwrap_or((80, 24));
        let term_width = term_width as usize;

        if !self.ui_initialized {
            // 预留 9 行空间
            for _ in 0..9 {
                writeln!(self.stdout)?;
            }
            self.ui_initialized = true;
        }

        queue!(
            self.stdout,
            cursor::MoveUp(9),
            cursor::MoveToColumn(0),
        )?;

        queue!(self.stdout, Clear(ClearType::UntilNewLine), Print(format!("{} {}", "[LazyFinder]".green().bold(), UI_SCANNING.cyan())), cursor::MoveDown(1), cursor::MoveToColumn(0))?;
        queue!(self.stdout, Clear(ClearType::UntilNewLine), Print(format!("{}: {} | {}: {}", UI_SCANNED_FILES, self.scanned_count.to_string().yellow(), UI_FOUND_MATCHES, self.match_count.to_string().red())), cursor::MoveDown(1), cursor::MoveToColumn(0))?;
        
        let max_file_len = term_width.saturating_sub(15).max(10);
        let display_file = if self.current_file.len() > max_file_len {
            format!("...{}", &self.current_file[self.current_file.len() - max_file_len + 3..])
        } else {
            self.current_file.clone()
        };
        queue!(self.stdout, Clear(ClearType::UntilNewLine), Print(format!("{}: {}", UI_CURRENT_FILE, display_file)), cursor::MoveDown(1), cursor::MoveToColumn(0))?;
        
        let sep = "-".repeat(term_width.min(80));
        queue!(self.stdout, Clear(ClearType::UntilNewLine), Print(sep.dimmed()), cursor::MoveDown(1), cursor::MoveToColumn(0))?;
        
        let max_line_len = term_width.saturating_sub(2);
        for i in 0..5 {
            queue!(self.stdout, Clear(ClearType::UntilNewLine))?;
            if let Some(line) = self.recent_matches.get(i) {
                let truncated = truncate_to_width(line, max_line_len);
                queue!(self.stdout, Print(truncated))?;
            }
            if i < 4 {
                queue!(self.stdout, cursor::MoveDown(1), cursor::MoveToColumn(0))?;
            }
        }
        
        self.stdout.flush()?;
        Ok(())
    }
}

impl crate::scanner::Reporter for CliReporter {
    fn on_file_scanned(&mut self, path: &str) {
        self.scanned_count += 1;
        self.current_file = path.to_string();
        let _ = self.draw_ui(false);
    }

    fn on_file_match_start(&mut self, path: &str) {
        if self.use_stdout {
            let _ = writeln!(self.stdout, "{} : {} ", "path".blue(), path);
        } else {
            if self.output_file.is_none() {
                if let Some(p) = &self.output_path {
                    self.output_file = std::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(p)
                        .ok();
                }
            }
            if let Some(f) = &mut self.output_file {
                let _ = writeln!(f, "path : {}", path);
            }
        }
    }

    fn on_match(&mut self, line_num: usize, raw_line: &str, colored_line: &str) {
        self.match_count += 1;
        let display_raw = format!("\tline {}: {}", line_num, raw_line);
        let display_colored = format!("\t{} {}: {}", "line".red(), line_num.to_string().cyan(), colored_line);

        if self.use_stdout {
            let _ = writeln!(self.stdout, "{}", display_colored);
        } else {
            if let Some(f) = &mut self.output_file {
                let _ = writeln!(f, "{}", display_raw);
            }
            if self.recent_matches.len() == 5 {
                self.recent_matches.pop_front();
            }
            self.recent_matches.push_back(display_colored);
            let _ = self.draw_ui(true);
        }
    }
}
