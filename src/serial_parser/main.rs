pub mod serail_parser{
    struct parser {
        partial: String,
        completed: String,
        last_completed: String
    }
    pub fn new() -> parser {
        parser {
            partial: String::new(),
            completed: String::new(),
            last_completed: String::new(),
        }
    }
    impl parser {
        pub fn add_and_return(input: Vec<u8>) -> Vec<String> {

        }
        fn convert_add(&mut self, incoming_vec: &Vec<u8>) {
            let outstring = String::new();
            for byte in incoming_vec {
                outstring.push(*byte as char);
            }
            &self.partial.partial.push_str(outstring);
        }
        fn get_completed(&self) -> String {

        }
        fn clear_completed(&mut self) {

        }
    }
}