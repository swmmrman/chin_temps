use serialport;
use std::io::{self,Read};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{ExecutableCommand,
    cursor::{MoveUp}
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

fn main() {
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let mut serial_buff: Vec<u8> = vec![0; 256];
    print!("\n\n\n");
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(t) => {
                if t > 48 {continue};
                if t < 40 {continue};
                let text = String::from_utf8_lossy(&serial_buff[..t]).to_string();
                let data = parse_raw(text);
                let _ = io::stdout().execute(MoveUp(3));
                println!("Out: {:.2}f {:.2}%\r\nIn: {:.2}f {:.2}% \r\nTD: {:.2}f HD: {:.2}%",
                    data.temp1,
                    data.humid1,
                    data.temp2,
                    data.humid2,
                    data.temp2 - data.temp1,
                    data.humid2 - data.humid1
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
fn parse_raw(raw_string: String) -> EvapData {
    let vals = raw_string[0..raw_string.len()-2].split(",").collect::<Vec<_>>();
    let raw_data = EvapData {
        temp1:          vals[0].parse::<f32>().unwrap(),
        temp2:          vals[1].parse::<f32>().unwrap(),
        temp3:          vals[2].parse::<f32>().unwrap(),
        humid1:         vals[3].parse::<f32>().unwrap(),
        humid2:         vals[4].parse::<f32>().unwrap(),
        humid3:         vals[5].parse::<f32>().unwrap(),
        ldr:            vals[6].parse::<i32>().unwrap(),
        valve_status:   vals[7].parse::<i8>().unwrap(),
    };
    raw_data
}
