mod serial_parser;
mod sensors;
mod evap_data;
mod logging;
mod temp;
mod rh;

use evap_data::evap_data::EvapData;
use logging::logging::{make_log_file, write_to_log};

use serialport::{self, SerialPort};
use std::{path,fs};
use std::io::{self, Read, Seek, Write};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{ExecutableCommand,
    cursor::{MoveUp}
};
use unix_named_pipe;

extern crate chrono;
use chrono::{Datelike, Local};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut dev = "USB0";
    if args.len() > 1 {
        dev = &args[1];
    }
    let mut log_file = make_log_file();
    let dev_path = format!("/dev/tty{}", dev);
    let out_path = path::Path::new("/tmp/page/");
    let out_file_name = "temp_in.txt".to_owned();
    let socket_path = path::Path::new("/tmp/chin_temp");
    if  socket_path.exists() {
        std::fs::remove_file(socket_path).unwrap();
    }
    let mut socket = setup_socket(socket_path);
    let lines: u16 = 13;
    let mut sleep_time = 50; //Sleep time at end of loop.  Short at start.
    let mut port = serialport::new(dev_path, 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let date = Local::now();
    println!("{}", date.format("%m-%d-%Y %H:%M:%S"));
    let mut cur_day: i32 = date.num_days_from_ce();
    let mut serial_buff: Vec<u8> = vec![0; 256];
    let mut data = evap_data::evap_data::new();
    let mut five_minute = evap_data::evap_data::new();
    print!("{}","\n".repeat(lines.into()));
    if ! path::Path::exists(out_path) {
        fs::create_dir(out_path).unwrap();
    }
    let mut out_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_path.join(out_file_name)).unwrap();
    let mut reader = serial_parser::serial_parser::new();
    let mut ts = date.timestamp() - (date.timestamp() % 300);
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(t) => {
                if t > 60 {continue}; // Discard Initial buffer.
                match reader.add_and_return(&serial_buff, t) {
                    Some(vals) => { 
                        data.update(vals.clone());
                        five_minute.update(vals);
                        sleep_time = 500; //Raise sleep time after first completed.
                    }
                    None => ()
                };
                let _ = io::stdout().execute(MoveUp(lines));
                println!("{}", data.get_evap_data());
                let new_date = Local::now();
                let days = new_date.num_days_from_ce();
                if days != cur_day {
                    cur_day = days;
                    data.clear();
                    println!("\n{}", new_date.format("%m-%d-%Y %H:%M:%S"));
                    print!("{}", "\n".repeat(lines.into()));
                }
                let check = check_time(300, ts, true);
                if check != 0 {
                    ts = check;
                    write_to_log(&mut five_minute, new_date, &mut log_file);
                }
            },
            //Skip timeouts, quit if device is gone.
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                println!("Pipe is broken, device unplugged or double access.");
                std::process::exit(1);   
            },
            //Print error otherwise
            Err(e) => eprintln!("{:?}", e),
        }
        out_file.seek(io::SeekFrom::Start(0)).unwrap();
        out_file.write(format!("{: >5.2}", data.get_inside_temp()).as_bytes()).unwrap();
        sleep(Duration::from_millis(sleep_time));
        let (command, offset) = read_socket(&mut socket);
        if command != "" {
            update_limits(command, offset, &mut port, &data);
        }
    }
}

fn check_time(time_frame: i64, last_time: i64, aligned: bool) -> i64 {
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

fn setup_socket(socket_path: &path::Path) -> std::fs::File {
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
fn read_socket(socket_file: &mut std::fs::File) -> (String, f32) {
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

fn parse_offset(buff: &mut String) -> (String,f32) {
    let (command, val) = match buff.find(",") {
        Some(t) => {
            let val = &buff[t+1..];
            let com = &buff[..t];
            (com.to_owned(), val.to_owned())
        },
        None => { 
            ("".to_owned(), "".to_owned())
        },
    };
    let value = match val.parse::<f32>() {
        Ok(o) => o,
        Err(_) => 69.420,
    };
    (command, value)
}

fn update_limits(command: String, offset: f32, sp: &mut Box<dyn SerialPort + 'static>, ed: &EvapData) {
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