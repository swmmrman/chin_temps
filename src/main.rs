mod serial_parser;
mod sensors;
mod evap_data;
mod logging;
mod temp;
mod rh;

use evap_data::evap_data::EvapData;
use logging::logging::{make_log_file, write_to_log};

use serialport;
use std::{path,fs};
use std::io::{self, Read, Seek, Write};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{ExecutableCommand,
    cursor::{MoveUp}
};
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
        out_file.write(format!("{: >5.2}", data.temp2.get_cur()).as_bytes()).unwrap();
        sleep(Duration::from_millis(sleep_time));
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