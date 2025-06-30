use serialport;
use std::io::{self,Read};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    let mut serial_buff: Vec<u8> = vec![0; 256];
    loop {
        match port.read(serial_buff.as_mut_slice()) {
            Ok(_) => {
                let text = String::from_utf8_lossy(&serial_buff);
                println!("{}",text);
            },
            //From the examples..  Do nothing if timed out.
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            //Print error otherwise
            Err(e) => eprintln!("{:?}", e),
        }
        sleep(Duration::from_millis(100));
    }
}
