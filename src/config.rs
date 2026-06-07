pub mod config {
    use chrono::Local;
    use ron;
    use serde;
    use serialport::SerialPort;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::time::Duration;

    use crate::tools::tools::{setup_error_file, setup_watch_dog_file};

    #[derive(serde::Deserialize)]
    pub struct Config {
        pub device: String,
        pub low_rh: f32,
        pub high_rh: f32,
        pub error_file: String,
        pub history_file: String,
        pub adjustments_file: String,
    }
    impl Config {
        pub fn read_config(conf_file: &mut std::fs::File) -> self::Config {
            let mut config_string = String::new();
            match conf_file.read_to_string(&mut config_string) {
                Ok(_) => (),
                Err(e) => {
                    println!("Failure reading from config file: {:?}", e);
                    std::process::exit(1);
                }
            };
            let config: Config = match ron::from_str(&config_string) {
                Ok(t) => t,
                Err(e) => {
                    println!("Failure reading config {:?}", e);
                    std::process::exit(1);
                }
            };
            config
        }
        pub fn get_low_limit(&self) -> f32 {
            self.low_rh
        }
        pub fn get_high_limit(&self) -> f32 {
            self.high_rh
        }
        pub fn set_low_limit(&mut self, new_limit: f32) {
            self.low_rh = new_limit;
        }
        pub fn set_high_limit(&mut self, new_limit: f32) {
            self.high_rh = new_limit;
        }
        pub fn update(&mut self, limit: String, value: f32) {
            let lim = &limit[0..1].to_ascii_lowercase();
            if lim == "h" {
                self.set_high_limit(value);
            }
            if lim == "l" {
                self.set_low_limit(value);
            }
        }
        pub fn get_device_path(&self) -> String {
            self.device.clone()
        }
    }

    pub struct RunTimeFiles {
        out_file: File,
        fan_file: File,
        call_file: File,
    }

    pub struct RunTimeConfig {
        pub arduino: Box<dyn SerialPort>,
        rtf: RunTimeFiles,
        ts: i64,
        watch_dog_time: i64,
        watch_dog_timeout: i64,
    }
    impl RunTimeConfig {
        pub fn new(
            config: &Config,
            (out_file, fan_file, call_file, ts): (File, File, File, i64),
        ) -> RunTimeConfig {
            RunTimeConfig {
                arduino: serialport::new(config.get_device_path(), 115200)
                    .timeout(Duration::from_millis(10))
                    .open()
                    .expect("failed to open port"),
                rtf: RunTimeFiles {
                    out_file: out_file,
                    fan_file: fan_file,
                    call_file: call_file,
                },
                ts: ts,
                watch_dog_time: 20,
                watch_dog_timeout: Local::now().timestamp() + 20,
            }
        }
        pub fn get_fan_file(&self) -> File {
            //Just gonna crash for now, if it cant clone the file handle
            self.rtf
                .fan_file
                .try_clone()
                .expect("Could not clone fan_file handle")
        }
        pub fn get_call_file(&mut self) -> &mut File {
            &mut self.rtf.call_file
        }
        pub fn get_ts(&self) -> i64 {
            self.ts
        }
        pub fn set_ts(&mut self, new_ts: i64) {
            self.ts = new_ts
        }
        pub fn get_out_file(&mut self) -> &File {
            &self.rtf.out_file
        }
        pub fn set_watch_dog_timeout(&mut self) {
            self.watch_dog_timeout = Local::now().timestamp() + self.watch_dog_time;
        }
        fn _get_watch_dog_timeout(&self) -> i64 {
            self.watch_dog_timeout
        }
        pub fn check_watch_dog(&mut self) {
            let cur_ts = Local::now().timestamp();
            if self.watch_dog_timeout < cur_ts {
                self.set_watch_dog_timeout();
                self.reset_arduino();
            }
        }
        fn reset_arduino(&self) {
            let mut watch_dog_file = setup_watch_dog_file();
            watch_dog_file.write("1".as_bytes()).unwrap();
            drop(watch_dog_file);
            let mut error_file = setup_error_file();
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            let error_message = format!("[{}] Arduino crash detected", ts);
            error_file.write(error_message.as_bytes()).unwrap();
        }
    }
}
