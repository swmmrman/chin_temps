use serialport;
use std::io::{self,Read};
use std::time::Duration;
use std::thread::sleep;

struct EvapData {
    temp1: f32,
    temp2: f32,
    temp3: f32,
    humid1: f32,    
    humid2: f32,
    humid3: f32,
    valve_status: i8,    
}

fn main() {
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let mut serial_buff: Vec<u8> = vec![0; 256];
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(t) => {
                if(t > 48) {continue};
                if(t < 40) {continue};
                let text = String::from_utf8_lossy(&serial_buff[..t]).to_string();
                parse_raw(text);
                //print!("{}",text);
            },
            //From the examples..  Do nothing if timed out.
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            //Print error otherwise
            Err(e) => eprintln!("{:?}", e),
        }
        sleep(Duration::from_millis(500));
    }
}
