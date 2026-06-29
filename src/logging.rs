pub mod logging {
    use std::fs::{self, File};
    use std::io::Write;

    use chrono::Local;

    pub struct Logger {
        error_log: File,
        history_log: File,
        adjustments_log: File,
    }

    pub enum LogType {
        Error,
        History,
        Adjustments,
    }

    impl Logger {
        pub fn new(error_path: &str, history_path: &str, adjustments_path: &str) -> Logger {
            Logger {
                error_log: Logger::initialize_log(error_path),
                history_log: Logger::initialize_log(history_path),
                adjustments_log: Logger::initialize_log(adjustments_path),
            }
        }
        fn initialize_log(file_path: &str) -> File {
            let file = match fs::OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(file_path)
            {
                Ok(f) => f,
                Err(e) => {
                    println!("Error setting up log file {:?}: {}", file_path, e);
                    std::process::exit(1);
                }
            };
            file
        }
        pub fn write_to_log(&mut self, message: &str, log: LogType) {
            let mut file = match log {
                LogType::Error => &self.error_log,
                LogType::History => &self.history_log,
                LogType::Adjustments => &self.adjustments_log,
            };
            let ts = Local::now().format("[%Y-%m-%d %H:%M:%S]");
            let out_message = format!("{} {}\n", ts, message);
            let _ = file.write(out_message.as_bytes());
        }
    }
}
