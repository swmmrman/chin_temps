pub mod serial_parser{
    pub struct Parser {
        partial: String,
        completed: String,
        last_completed: String
    }

    impl Parser {
        pub fn add_and_return(&mut self, input: &[u8], t:usize) -> Option<Vec<String>> {
            Self::convert_add(self,&input[..t]);
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
        fn convert_add(&mut self, incoming_vec: &[u8]) {
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
            if self.partial.contains("\r\n") {
                let index = self.partial.find("\r\n").unwrap();
                self.completed = self.partial[..index].to_string();
                self.partial = self.partial[index +2..].to_owned();
            }
        }
        fn clear_completed(&mut self) {
            self.last_completed = self.completed.clone();
            self.completed = String::new();
        }
        fn convert_to_vec(input: String) -> Vec<String> {
            let parts = input.split(",");
            let mut out_vec = Vec::new();
            for part in parts {
                out_vec.push(part.to_string());
            }
            out_vec
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