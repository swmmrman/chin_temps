use serialport::{self, new};
use std::io::{self,Read};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{ExecutableCommand,
    cursor::{MoveUp}
};
extern crate chrono;
use chrono::{Datelike, Local};

struct Temp {
    min_temp: f32,
    max_temp: f32,
    cur_temp: f32,
}

impl Temp {
    fn update(&mut self, new_temp: f32) {
        self.cur_temp = new_temp;
        //Clear NaNs first
        if self.min_temp.is_nan() { self.min_temp = new_temp };
        if self.max_temp.is_nan() { self.max_temp = new_temp }
        //Then check min/max
        if self.cur_temp < self.min_temp { self.min_temp = new_temp; }
        if self.cur_temp > self.max_temp { self.max_temp = new_temp; }
    }
    fn new() -> Temp {
        Temp {
            min_temp: f32::NAN,
            max_temp: f32::NAN,
            cur_temp: f32::NAN
        }
    }
    fn get_cur(&self) -> f32 {
        self.cur_temp
    }
    fn clear(&mut self) {
        self.cur_temp = f32::NAN;
        self.min_temp = f32::NAN;
        self.max_temp = f32::NAN;
    }
}

struct RH {
    min_rh: f32,
    max_rh: f32,
    cur_rh: f32
}

impl RH {
    fn new() -> RH{
        RH {
            min_rh: f32::NAN,
            max_rh: f32::NAN,
            cur_rh: f32::NAN,
        }
    }
    fn clear(&mut self) {
        self.min_rh = f32::NAN;
        self.max_rh = f32::NAN;
        self.cur_rh = f32::NAN;
    }
    fn get_cur(&self) -> f32 {
        self.cur_rh
    }
    fn update(&mut self, new_val: f32) {
        if self.min_rh.is_nan() {
            self.min_rh = new_val;
            self.max_rh = new_val;
        }
        else if self.min_rh < new_val {
            self.min_rh = new_val;
        }
        else if self.max_rh > new_val {
            self.max_rh = new_val;
        }
        self.cur_rh = new_val;
    }
}

//#[derive(Debug)]
struct EvapData {
    temp1: Temp,
    temp2: Temp,
    temp3: Temp,
    humid1: RH,    
    humid2: RH,
    humid3: RH,
    ldr: i32,
    valve_status: i8,
}

impl EvapData {
    fn update(&mut self, vals: Vec<&str>) {
        self.temp1.update(vals[0].parse::<f32>().unwrap());
        self.temp2.update(vals[1].parse::<f32>().unwrap());
        self.temp3.update(vals[2].parse::<f32>().unwrap());
        self.humid1.update(vals[3].parse::<f32>().unwrap());
        self.humid2.update(vals[4].parse::<f32>().unwrap());
        self.humid3.update(vals[5].parse::<f32>().unwrap());
        self.ldr = vals[6].parse::<i32>().unwrap();
        self.valve_status = vals[7].parse::<i8>().unwrap();
    }
    fn new() -> EvapData{
        EvapData { temp1: Temp::new(), temp2: Temp::new(), temp3: Temp::new(), humid1: RH::new(), humid2: RH::new(), humid3: RH::new(), ldr: -500, valve_status: -1 }
    }
    fn get_delta_t(&self) -> f32 {
        self.temp2.get_cur() - self.temp1.get_cur()
    }
    fn get_delta_h(&self) -> f32 {
        self.humid2.get_cur() - self.humid1.get_cur()
    }
    fn valve_status(&self) -> String {
        match self.valve_status{
            0 => "Off  ".to_string(),
            1 => "On   ".to_string(),
            2 => "Wait ".to_string(),
            _ => "What?".to_string(),
        }
    }
    fn clear(&mut self) {
        self.temp1.clear();
        self.temp2.clear();
        self.temp3.clear();
        self.humid1.clear();
        self.humid2.clear();
        self.humid3.clear();
        self.valve_status = -1;
    }
}

fn main() {
    let lines: u16 = 6;
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let mut date = Local::now();
    println!("{}", date.format("%m-%d-%Y %H:%M:%S"));
    let mut cur_day: i32 = date.num_days_from_ce();
    let mut serial_buff: Vec<u8> = vec![0; 256];
    let mut data = EvapData::new();
    print!("{}","\n".repeat(lines.into()));
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(t) => {
                if t > 48 {continue};
                if t < 32 {continue};
                let text = String::from_utf8_lossy(&serial_buff[..t]).to_string();
                let vals = text[0..text.len()-2].split(",").collect::<Vec<_>>();
                //Check for empty values in the vec.
                if vals.contains(&"") {continue;}
                //Check for 7 to be there.
                if vals.len() < 8 {continue;}
                data.update(vals);
                // let data = parse_raw(text);
                let _ = io::stdout().execute(MoveUp(lines));
                println!("Out: {:.2}f {:.2}%\r\nIn:  {:.2}f {:.2}% \r\nTD:  {:.2}f HD: {:.2}%\nValve: {}\nMax Temps:\nIn:{:.2}f Out:{:.2}f",
                    data.temp1.get_cur(),
                    data.humid1.get_cur(),
                    data.temp2.get_cur(),
                    data.humid2.get_cur(),
                    data.get_delta_t(),
                    data.get_delta_h(),
                    data.valve_status(),
                    data.temp2.max_temp,
                    data.temp1.max_temp,
                );
                date = Local::now();
                let days = date.num_days_from_ce();
                if days != cur_day {
                    cur_day = days;
                    data.clear();
                    println!("{}", date.format("%m-%d-%Y %H:%M:%S"));
                    print!("{}", "\n".repeat(lines.into()));
                }
                
            },
            //From the examples..  Do nothing if timed out.
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            //Print error otherwise
            Err(e) => eprintln!("{:?}", e),
        }
        sleep(Duration::from_millis(500));
    }
}

