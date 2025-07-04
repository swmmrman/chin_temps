mod serial_parser;
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
    let mut port = serialport::new(dev_path, 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let mut date = Local::now();
    println!("{} {}", date.format("%m-%d-%Y %H:%M:%S"), date.num_days_from_ce());
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
                    Some(vals) => { data.update(vals); }
                    None => ()
                };
                // let data = parse_raw(text);
                let _ = io::stdout().execute(MoveUp(lines));
                println!("Out: {: >7.2}f {: >7.2}%\r\nIn:  {: >7.2}f {: >7.2}% \r\nDiff:{: >7.2}f {: >7.2}%\nValve: {}\nMax Temps:\t\t\t\tMin Temps:\nIn:{: >7.2}f  Out:{: >7.2}f\t\tIn:   {: >7.2}f  Out: {: >7.2}f\nMax RH:\t\t\t\t\tMax TDs:\nIn:{: >7.2}%  Out:{: >7.2}%\t\tHigh: {: >7.2}f  Low: {: >7.2}f\nMin RH:\nIn:{: >7.2}%  Out:{: >7.2}%",
                    data.temp1.get_cur(),
                    data.humid1.get_cur(),
                    data.temp2.get_cur(),
                    data.humid2.get_cur(),
                    data.get_delta_t(),
                    data.get_delta_h(),
                    data.valve_status(),
                    data.temp2.max_temp,
                    data.temp1.max_temp,
                    data.temp2.min_temp,
                    data.temp1.min_temp,
                    data.humid2.max_rh,
                    data.humid1.max_rh,
                    data.deltas.max_temp,
                    data.deltas.min_temp,
                    data.humid2.min_rh,
                    data.humid1.min_rh,
                );
                let new_date = Local::now();
                let days = new_date.num_days_from_ce();
                print!("{}\r", days);
                if days != cur_day {
                    cur_day = days;
                    data.clear();
                    println!("\n{} {}", new_date.format("%m-%d-%Y %H:%M:%S"), days);
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
        sleep(Duration::from_millis(500));
    }
}

