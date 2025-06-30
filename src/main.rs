use serialport;
use std::io::Read;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open().expect("failed to open port");
    loop {
        let mut serial_buff: Vec<u8> = vec![0; 256];
        port.read(serial_buff.as_mut_slice()).expect("No data");
        let text = String::from_utf8_lossy(&serial_buff);
        println!("{}",text);
        sleep(Duration::from_millis(2000));
    }
}
