pub mod logging {
    use std::fs::{self,File};
    use std::path;
    use std::io::Write;
    use crate::EvapData;

    use chrono::{DateTime, Local};

    pub fn make_log_file() -> File {
        let home_dir = std::env::home_dir().unwrap();
        let logs_path = home_dir.join("logs/evap/");
        if ! path::Path::exists(&logs_path) {
            fs::create_dir_all(&logs_path).unwrap()
        }
        let log_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path::Path::new(&logs_path).join("evap.log")).unwrap();
        log_file
    }

    pub fn write_to_log(
            short_term: &mut EvapData,
            new_date: DateTime<Local>,
            log_file: &mut File,
        ) {
        let log_string = format!("\n[{}]\n{}\n", 
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
}