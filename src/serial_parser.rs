pub mod serial_parser{
    pub struct Parser {
        partial: String,
        completed: String,
        last_completed: String
    }

    impl Parser {
        pub fn add_and_return(&mut self, input: &Vec<u8>) -> Option<Vec<String>> {
            Self::convert_add(self,input);
            let output= Self::get_completed();
            Self::convert_to_vec(self);
            output.
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
        fn get_completed(&mut self) -> Option<String> {
            let mut outstring = String::new();
            if self.partial.contains("\r\n") {
                let index = outstring.find("\r\n").unwrap();
                outstring = self.partial[..index].to_string();
                Some(outstring)
            }
            else { 
                None
            }
        }
        fn clear_completed(&mut self) {
            self.last_completed = self.completed.clone();
            self.completed = String::new();
        }
        fn convert_to_vec(input: String) -> Vec<String> {
            
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