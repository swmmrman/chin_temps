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
        fn convert_add(&mut self) {

        }
        fn get_completed(&self) -> String {

        }
        fn clear_completed(&mut self) {

        }
    }
}