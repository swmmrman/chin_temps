mod serial_parser;
mod sensors;
mod evap_data;
mod temp;
mod rh;

use serialport;
use std::io::{self,Read};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{ExecutableCommand,
    cursor::{MoveUp}
};
extern crate chrono;
use chrono::{Datelike, Local};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut dev = "ACM0";
    if args.len() > 1 {
        dev = &args[1];
    }
    let dev_path = format!("/dev/tty{}", dev);
    let lines: u16 = 10;
    let mut sleep_time = 50; //Sleep time at end of loop.  Short at start.
    let mut port = serialport::new(dev_path, 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let date = Local::now();
    println!("{}", date.format("%m-%d-%Y %H:%M:%S"));
    let mut cur_day: i32 = date.num_days_from_ce();
    let mut serial_buff: Vec<u8> = vec![0; 256];
    let mut data = evap_data::evap_data::new();
    print!("{}","\n".repeat(lines.into()));
    let mut reader = serial_parser::serial_parser::new();
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(t) => {
                if t > 48 {continue}; // Discard Initial buffer.
                match reader.add_and_return(&serial_buff, t) {
                    Some(vals) => { 
                        data.update(vals);
                        sleep_time = 500; //Raise sleep time after first completed.
                    }
                    None => ()
                };
                // let data = parse_raw(text);
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
            },
            //From the examples..  Do nothing if timed out.
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                println!("Pipe is broken, device unplugged or double access.");
                std::process::exit(1);   
            },
            //Print error otherwise
            Err(e) => eprintln!("{:?}", e),
        }
        sleep(Duration::from_millis(sleep_time));
    }
}

