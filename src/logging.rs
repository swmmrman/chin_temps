pub mod logging {
    use crate::EvapData;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path;

    use chrono::{DateTime, Local};

    pub fn make_log_file() -> File {
        let home_dir = std::env::home_dir().unwrap();
        let logs_path = home_dir.join("logs/evap/");
        if !path::Path::exists(&logs_path) {
            fs::create_dir_all(&logs_path).unwrap()
        }
        let log_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path::Path::new(&logs_path).join("evap.log"))
            .unwrap();
        log_file
    }

    pub fn write_to_log(short_term: &mut EvapData, new_date: DateTime<Local>, log_file: &mut File) {
        let log_string = format!(
            "\n[{}]\n{}\n",
            new_date.format("%m-%d-%Y %H:%M:%S"),
            short_term.get_evap_data(),
        );
        match log_file.write(log_string.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writting to log file: {}", e);
            }
        }
        short_term.clear();
    }

    pub struct Logger {
        error_log: File,
        history_log: File,
        adjustments_log: File,
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
    }
}
