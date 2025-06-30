use serialport;
use std::io::{self,Read};
use std::str::FromStr;
use std::time::Duration;
use std::thread::sleep;
use crossterm::{ExecutableCommand,
    cursor::{MoveUp,MoveDown}
};

#[derive(Debug)]
struct EvapData {
    temp1: f32,
    temp2: f32,
    temp3: f32,
    humid1: f32,    
    humid2: f32,
    humid3: f32,
    ldr: i32,
    valve_status: i8,
}

impl EvapData {
    fn update(&mut self, vals: Vec<&str>) {
        self.temp1 = vals[0].parse::<f32>().unwrap();
        self.temp2 = vals[1].parse::<f32>().unwrap();
        self.temp3 = vals[2].parse::<f32>().unwrap();
        self.humid1 = vals[3].parse::<f32>().unwrap();
        self.humid2 = vals[4].parse::<f32>().unwrap();
        self.humid3 = vals[5].parse::<f32>().unwrap();
        self.ldr = vals[6].parse::<i32>().unwrap();
        self.valve_status = vals[7].parse::<i8>().unwrap();
    }
    fn new() -> EvapData{
        EvapData { temp1: -70.0f32, temp2: -70.0f32, temp3: -70.0f32, humid1: -50.0f32, humid2: -50.0f32, humid3: -50.0f32, ldr: -500, valve_status: -1 }
    }
    fn get_delta_t(&self) -> f32 {
        self.temp2 - self.temp1
    }
    fn get_delta_h(&self) -> f32 {
        self.humid2 - self.humid1
    }
    fn valve_status(&self) -> String {
        match self.valve_status{
            0 => "Off  ".to_string(),
            1 => "On   ".to_string(),
            2 => "Wait ".to_string(),
            _ => "What?".to_string(),
        }
    }
}

fn main() {
    let lines: u16 = 4;
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let mut serial_buff: Vec<u8> = vec![0; 256];
    let mut data = EvapData::new();
    print!("{}","\n".repeat(lines.into()));
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(t) => {
                if t > 48 {continue};
                if t < 40 {continue};
                let text = String::from_utf8_lossy(&serial_buff[..t]).to_string();
                let vals = text[0..text.len()-2].split(",").collect::<Vec<_>>();
                data.update(vals);
                // let data = parse_raw(text);
                let _ = io::stdout().execute(MoveUp(lines));
                println!("Out: {:.2}f {:.2}%\r\nIn:  {:.2}f {:.2}% \r\nTD:  {:.2}f HD: {:.2}%\nValve: {}",
                    data.temp1,
                    data.humid1,
                    data.temp2,
                    data.humid2,
                    data.get_delta_t(),
                    data.get_delta_h(),
                    data.valve_status()
                );
                
            },
            //From the examples..  Do nothing if timed out.
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            //Print error otherwise
            Err(e) => eprintln!("{:?}", e),
        }
        sleep(Duration::from_millis(500));
    }
}

