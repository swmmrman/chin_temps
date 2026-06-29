fn print_help(call: String) {
    println!(
        "Usage: {} command value

Commands:
  settemp\tvalue\tSet the temperature setpoint
  sethumid\tvalue\tSet the humidity target
  help\t \t Print this help message
        ",
        call
    )
}

fn main() {
    let mut args = std::env::args();
    if args.len() != 3 {
        println!("No args supplied");
        print_help(args.nth(0).unwrap());
        std::process::exit(1);
    }
    if args.any(|x| x == "help".to_string()) {
        std::process::exit(0);
    }
    let command = args.nth(1);
    let v = args.nth(2).unwrap();
    let value = match v.parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Value must be of type int");
            std::process::exit(1)
        }
    };
}
