pub mod config {
    use ron;
    use serde;
    use serialport::SerialPort;
    use std::fs::File;
    use std::io::Read;
    use std::ops::Deref;
    use std::time::Duration;

    #[derive(serde::Deserialize)]
    pub struct Config {
        pub device: String,
        pub low_rh: f32,
        pub high_rh: f32,
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

    pub struct files {
        out_file: File,
        fan_file: File,
        call_file: File,
    }

    pub struct run_time_config {
        pub arduino: Box<dyn SerialPort>,
        files: files,
        ts: i64,
    }
    impl run_time_config {
        pub fn new(
            config: &Config,
            (out_file, fan_file, call_file, ts): (File, File, File, i64),
        ) -> run_time_config {
            run_time_config {
                arduino: serialport::new(config.get_device_path(), 115200)
                    .timeout(Duration::from_millis(10))
                    .open()
                    .expect("failed to open port"),
                files: files {
                    out_file: out_file,
                    fan_file: fan_file,
                    call_file: call_file,
                },
                ts: ts,
            }
        }
        pub fn get_fan_file(&self) -> File {
            //Just gonna crash for now, if it cant clone the file handle
            self.files
                .fan_file
                .try_clone()
                .expect("Could not clone fan_file handle")
        }
        pub fn get_call_file(&mut self) -> &mut File {
            &mut self.files.call_file
        }
        pub fn get_ts(&self) -> i64 {
            self.ts
        }
        pub fn set_ts(&mut self, new_ts: i64) {
            self.ts = new_ts
        }
        pub fn get_out_file(&mut self) -> &File {
            &self.files.out_file
        }
        pub fn reset_arduino(&mut self, config: &mut Config) {
            let _ = self.arduino.deref();
            self.arduino = serialport::new(config.get_device_path(), 115200)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to reopen port");
        }
    }
}
