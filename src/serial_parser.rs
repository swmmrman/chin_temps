pub mod serial_parser{
    pub struct Parser {
        partial: String,
        completed: String,
        last_completed: String
    }

    impl Parser {
        pub fn add_and_return(&mut self, input: &Vec<u8>) -> Vec<String> {
            Self::convert_add(self,input);
            let output: String = Self::get_completed();
            Self::clear_completed(self);
            //more here
            output
        }
        fn convert_add(&mut self, incoming_vec: &Vec<u8>) {
            let mut outstring = String::new();
            for byte in incoming_vec {
                outstring.push(*byte as char);
            }
            self.partial.push_str(&outstring);
        }
        fn get_completed() -> String {
            String::new()
        }
        fn clear_completed(&mut self) {
            self.completed = String::new();
        }
    }
    pub fn new() -> Parser {
        Parser {
            partial: String::new(),
            completed: String::new(),
            last_completed: String::new(),
        }
    }
}