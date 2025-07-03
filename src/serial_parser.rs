pub mod serial_parser{
    pub struct Parser {
        partial: String,
        completed: String,
        last_completed: String
    }

    impl Parser {
        pub fn add_and_return(&mut self, input: &Vec<u8>) -> Option<Vec<String>> {
            Self::convert_add(self,input);
            let out = match Self::get_completed(self) {
                Some(val_string) => {
                    let output = Self::convert_to_vec(val_string);
                    self.clear_completed();
                    output
                }
                None => {
                    return None
                }
            };
            Some(out)
        }
        fn convert_add(&mut self, incoming_vec: &Vec<u8>) {
            let mut outstring = String::new();
            for byte in incoming_vec {
                outstring.push(*byte as char);
            }
            self.partial.push_str(&outstring);
        }
        fn get_completed(&mut self) -> Option<String> {
            if !self.completed.is_empty() {
                let out = Some(self.completed.to_string());
                Self::clear_completed(self);
                out 
            }
            else { 
                None
            }
        }
        fn check_partial(&mut self) {
            let mut outstring = String::new();
            if self.partial.contains("\r\n") {
                let index = outstring.find("\r\n").unwrap();
                outstring = self.partial[..index].to_string();
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