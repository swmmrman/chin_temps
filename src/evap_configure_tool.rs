fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        println!("No args supplied");
        print_help();
        std::process::exit(1);
    }
    for arg in args {
        println!("{}", arg);
    }
}
