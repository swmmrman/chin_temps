fn print_help(call: String) {
    println!(
        "
Usage: {} command value

Commands:
  tsettemp\tvalue\tSet the temperature setpoint
  sethumid\tvalue\tSet the humidity target
        ",
        call
    )
}

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("No args supplied");
        print_help(args.nth(0).unwrap());
        std::process::exit(1);
    }
    for arg in args {
        println!("{}", arg);
    }
}
