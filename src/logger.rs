use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct Logger {
    log_path: PathBuf,
}

impl Logger {
    pub fn new(log_path: &str) -> Self {
        let path = if log_path.starts_with('~') {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(log_path.replace('~', &home))
        } else {
            PathBuf::from(log_path)
        };

        // ログディレクトリを作成
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        Self { log_path: path }
    }

    pub fn log(&self, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_line = format!("[{}] {}\n", timestamp, message);

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = file.write_all(log_line.as_bytes());
        }
    }

    pub fn log_error(&self, message: &str) {
        self.log(&format!("ERROR: {}", message));
    }

    pub fn log_info(&self, message: &str) {
        self.log(&format!("INFO: {}", message));
    }
}
