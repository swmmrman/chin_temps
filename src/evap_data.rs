pub mod evap_data {
    use crate::temp::temp;
    use crate::rh::rh;

//#[derive(Debug)]
    pub struct EvapData {
        pub temp1: temp::Temp,
        pub temp2: temp::Temp,
        pub temp3: temp::Temp,
        pub humid1: rh::RH,    
        pub humid2: rh::RH,
        pub humid3: rh::RH,
        ldr: i32,
        pub valve_status: i8,
        pub deltas: temp::Temp,
    }

    impl EvapData {
        //Call with the parsed data from the serial parser.  Validates all values.
        //The first 3 are temps, Next 3 Humidity, 7th is LDR(unused), last is valve
        //status. Deltase is calculated from the values of temp1 and 2.  Temp and humidity 1
        //are outside, Temp and humidity 2 are inside. 
        pub fn update(&mut self, vals: Vec<String>) {
            match validate(&vals) {
                true => {
                    self.temp1.update(vals[0].parse::<f32>().unwrap());
                    self.temp2.update(vals[1].parse::<f32>().unwrap());
                    self.temp3.update(vals[2].parse::<f32>().unwrap());
                    self.humid1.update(vals[3].parse::<f32>().unwrap());
                    self.humid2.update(vals[4].parse::<f32>().unwrap());
                    self.humid3.update(vals[5].parse::<f32>().unwrap());
                    self.ldr = vals[6].parse::<i32>().unwrap();
                    self.valve_status = vals[7].parse::<i8>().unwrap();
                    self.deltas.update(self.get_delta_t());
                }
                false => (),
            }
        }
        fn get_delta_t(&self) -> f32 {
            self.temp2.get_cur() - self.temp1.get_cur()
        }
        fn get_delta_h(&self) -> f32 {
            self.humid2.get_cur() - self.humid1.get_cur()
        }
        fn valve_status(&self) -> String {
            match self.valve_status{
                0 => "Closed        ".to_string(),
                1 => "Open          ".to_string(),
                2 => "Sensing Closed".to_string(),
                _ => "What?         ".to_string(),
            }
        }
        //Blanks all values. Currently ingores LDR as it is unused.
        pub fn clear(&mut self) {
            self.temp1.clear();
            self.temp2.clear();
            self.temp3.clear();
            self.humid1.clear();
            self.humid2.clear();
            self.humid3.clear();
            self.valve_status = -1;
            self.deltas.clear();
        }
        //Returns a formated string sutable for logging or displaying to terminal.
        //Possibly return a line count as well.
        //CSV function later. Could be used for html with returns replaced with
        //<br> and tabs or spaces with &nbsp..  probably easier to parse csv with js.
        pub fn get_evap_data(&self) -> String {
            format!(
"Out: {: >7.2}f {: >7.2}%\r\n\
In:  {: >7.2}f {: >7.2}% \r\n\
Diff:{: >7.2}f {: >7.2}%\n\
\n\
Valve: {}\n\
\n\
Max Temps:\t\t\t\tMin Temps:\n\
In:{: >7.2}f  Out:{: >7.2}f\t\tIn:   {: >7.2}f  Out: {: >7.2}f\n\
Max RH:\t\t\t\t\tMin RH:\n\
In:{: >7.2}%  Out:{: >7.2}%\t\tIn:   {: >7.2}%  Out: {: >7.2}%\n\
Max TDs:\t\t\t\tSensor 3\n\
High:{: >7.2}f  Low:{: >7.2}f\t\tTemp:{: >7.2}f   RH:  {: >7.2}%",
                self.temp1.get_cur(),
                self.humid1.get_cur(),
                self.temp2.get_cur(),
                self.humid2.get_cur(),
                self.get_delta_t(),
                self.get_delta_h(),
                self.valve_status(),
                self.temp2.get_max(),
                self.temp1.get_max(),
                self.temp2.get_min(),
                self.temp1.get_min(),
                self.humid2.get_max(),
                self.humid1.get_max(),
                self.humid2.get_min(),
                self.humid1.get_min(),
                self.deltas.get_max(),
                self.deltas.get_min(),
                self.temp3.get_cur(),
                self.humid3.get_cur()
            )
        }
    }
    //Return a new empty EvapData
    pub fn new() -> EvapData{
        EvapData { 
            temp1:temp::new(),
            temp2:temp::new(),
            temp3:temp::new(),
            humid1: rh::new(),
            humid2: rh::new(),
            humid3: rh::new(),
            ldr: -500,
            valve_status: -1,
            deltas:temp::new()
        }
    }
    //Validates the serial parser string vec.  First 6 are temps and RH.
    //LDR readings are i32,  Valid range needs checked and added.
    //Valve status is 0, 1, or 2 for closed open, and sensing.
    fn validate(vals: &Vec<String>) -> bool {
        for val in &vals[0..5] {
            match val.parse::<f32>() {
            Ok(_) => {
                continue;
            }
            _ => return false
            }
        }
        match &vals[6].parse::<i32>() {
            Ok(_) => (),
            _ => return false
        }
        match &vals[7].parse::<i8>() {
            Ok(_) => (),
            _ => return false
        }
        true
    }
}