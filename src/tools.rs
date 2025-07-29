pub mod tools {
    use std::{fs, path};
    use chrono::{DateTime, Local};
    use serialport::{self, SerialPort};
    use std::io::Read;
    use crate::EvapData;
    use ron;
    use serde;

    pub fn setup(date: &DateTime<Local>, lines: &usize) -> (std::fs::File, i64) {
        let out_path = path::Path::new("/tmp/page/");
        let out_file_name = "temp_in.txt".to_owned();
        println!("{}", date.format("%m-%d-%Y %H:%M:%S"));
        print!("{}","\n".repeat(*lines));
        let ts = date.timestamp() - (date.timestamp() % 300);
        if ! path::Path::exists(out_path) {
            fs::create_dir(out_path).unwrap();
        }
        let outfile = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(out_path.join(out_file_name)).unwrap();
        (outfile, ts)
    }

    pub fn setup_config_file() -> std::fs::File {
        let conf_path = path::Path::new("/etc/chin_temps/");
        let file = match fs::OpenOptions::new()
            .create(false)
            .write(true)
            .read(true)
            .open(conf_path.join("config.ron")) {
                Ok(f) => f,
                Err(e) => {
                    println!("Config file error: {}", e);
                    std::process::exit(1);
                }
        };
        file
    }

    pub fn read_config(conf_file: &mut std::fs::File) -> self::Config {
        let mut config_string = String::new();
        match conf_file.read_to_string(&mut config_string) {
            Ok(_) => (),
            Err(e) => {
                println!("Failure reading from config file: {:?}", e);
                std::process::exit(1);
            },
        };
        let config = match ron::from_str::<self::Config>(&config_string) {
            Ok(t) => t,
            Err(e) => {
                println!("Failure reading config {:?}", e);
                std::process::exit(1);
            }
        };
        config
    }


    pub fn check_time(time_frame: i64, last_time: i64, aligned: bool) -> i64 {
        let cur_ts =  Local::now().timestamp();
        let time_diff = cur_ts - last_time;
        if time_diff >= time_frame {
            let mut ts = cur_ts - (time_diff - time_frame);
            if aligned {
                ts -= ts % time_frame;
            }
            ts
        }
        else {
            0
        }
    }

    pub fn setup_socket(socket_path: &path::Path) -> std::fs::File {
        match unix_named_pipe::create(&socket_path, None) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to create the socket: {}", e);
                std::process::exit(1);
            },
        }
        let reader = match unix_named_pipe::open_read(socket_path) {
            Ok(r) => r,
            Err(e) => {
                println!("Failed to open socket for reading: {}", e);
                std::process::exit(2);
            }   
        };
        reader
    }

    pub fn read_socket(socket_file: &mut std::fs::File) -> (String, f32) {
        let mut offset = 0.0;
        let mut string_buffer = String::new();
        let mut command = String::new();
        match socket_file.read_to_string(&mut string_buffer) {
            Ok(t) => {
                if t > 0 {
                    (command, offset) = parse_offset(&mut string_buffer);
                }
            },
            Err(_) => (),
        }
        (command, offset)
    }

    /// Parse the buffer, Returns the command and value as a tuple.
    /// Returns ("","") if format is not statisfied.
    /// Returns the command and a value of 69.420 if no valid f32 can be constructed.
    /// Expected format is T[A],V
    /// Where T is H for H limit or L for Low limit.
    /// A is optional and specifies and absolute value.
    /// V is the relative or absolute value.
    pub fn parse_offset(buff: &mut String) -> (String,f32) {
        // Split the string into the value and command from csv.
        // return empty strings if no ,s are present.
        let (command, val) = match buff.find(",") {
            //Comma found, split at t.
            Some(t) => {
                let val = &buff[t+1..].trim().to_owned();
                let com = &buff[..t];
                (com.to_owned(), val.to_owned())
            },
            //Comma not found,  Return as error.
            None => { 
                ("".to_owned(), "".to_owned())
            },
        };
        //Extract the value.
        let value = match val.parse::<f32>() {
            Ok(o) => o,
            Err(_) => 69.420,
        };
        (command, value)
    }

    pub fn update_limits(command: String, offset: f32, sp: &mut Box<dyn SerialPort + 'static>, ed: &EvapData) {
        let c = command.to_uppercase();
        let main_command = &c[0..1];
        //Grab current low limit,  If Command is H, overwrite with high limit.
        let mut cur_set = ed.low_limit;
        let new_offset;
        if main_command == "H" {
            cur_set = ed.high_limit;
        }
        if c.len() > 1 && &c[1..] == "A" {
            new_offset = offset - cur_set;
        }
        else {
            new_offset = offset;
        }
        let out_string = format!("{} {}\n", main_command, new_offset);
        sp.write(out_string.as_bytes()).unwrap();
    }
#[derive(serde::Deserialize)]
    pub struct Config {
        pub device: String,
        pub low_rh: f32,
        pub high_rh: f32,
    }
}