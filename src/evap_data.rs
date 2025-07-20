pub mod evap_data {
    use crate::sensors::readings::{ReadingKind, ReadingType};
    use crate::sensors::sensor;
    use crate::temp::temp;

//#[derive(Debug)]
    pub struct SensorArray{ 
        inside: sensor::Sensor,
        outside: sensor::Sensor,
        spare: sensor::Sensor,
    }

    pub struct EvapData {
        sensors: SensorArray,
        pub low_limit: f32,
        pub high_limit: f32, 
        ldr: i32,               //Not working
        pub valve_status: i8,   //0-2 normal. oThers are failures
        pub deltas: temp::Temp, //Diff of sensor 1 and 2 temps.
        //pub delta_hs: rh::RH, //Add later might be neat to see.
    }

    impl EvapData {
        /// Call with the parsed data from the serial parser.  Validates all values.
        /// The first 3 are temps, Next 3 Humidity, 7th is LDR(unused), last is valve
        /// status. Deltas is calculated from the values of temp1 and 2.  Temp and humidity 1
        /// are outside, Temp and humidity 2 are inside. 
        pub fn update(&mut self, vals: Vec<String>) {
            match validate(&vals) {
                true => {
                    self.sensors.inside.update(
                        vals[1].parse::<f32>().unwrap(),
                        vals[4].parse::<f32>().unwrap(),
                    );
                    self.sensors.outside.update(
                        vals[0].parse::<f32>().unwrap(),
                        vals[3].parse::<f32>().unwrap(),
                    );
                    self.sensors.spare.update(
                        vals[2].parse::<f32>().unwrap(),
                        vals[5].parse::<f32>().unwrap(),
                    );
                    self.low_limit = vals[6].parse::<f32>().unwrap();
                    self.high_limit = vals[7].parse::<f32>().unwrap();
                    self.ldr = vals[8].parse::<i32>().unwrap();
                    self.valve_status = vals[9].parse::<i8>().unwrap();
                    self.deltas.update(self.get_delta(ReadingType::Temp));
                }
                false => (),
            }
        }
        fn get_delta(&self, reading_type: ReadingType) -> f32 {
            let outside_cur =self.sensors.outside.get_reading(reading_type.clone(), ReadingKind::Cur);
            let inside_cur = self.sensors.inside.get_reading(reading_type, ReadingKind::Cur);
            outside_cur - inside_cur
        }
        fn valve_status(&self) -> String {
            match self.valve_status{
                0 => "Closed        ".to_string(),
                1 => "Open          ".to_string(),
                2 => "Sensing Closed".to_string(),
                _ => "What?         ".to_string(),
            }
        }
        /// Blanks all values. Currently ingores LDR as it is unused.
        pub fn clear(&mut self) {
            self.sensors.inside.clear();
            self.sensors.outside.clear();
            self.sensors.spare.clear();
            self.valve_status = -1;
            self.deltas.clear();
        }
        /// Returns a formated string sutable for logging or displaying to terminal.
        /// Not currently HTML friendly.  Formatting will break.
        //Possibly return a line count as well.
        //CSV function later. Could be used for html with returns replaced with
        //<br> and tabs or spaces with &nbsp..  probably easier to parse csv with js.
        pub fn get_evap_data(&self) -> String {
            let inside = self.sensors.inside.get_all();
            let outside = self.sensors.outside.get_all();
            let spare = self.sensors.spare.get_all();
            format!(
"Out: {: >7.2}f {: >7.2}%\r\n\
In:  {: >7.2}f {: >7.2}% \r\n\
Diff:{: >7.2}f {: >7.2}%\n\
\n\
Valve: {}\n\
\n\
Max Temps:\t\t\t\tMin Temps:\n\
In:  {: >7.2}f  Out:{: >7.2}f\t\tIn:   {: >7.2}f  Out: {: >7.2}f\n\
Max RH:\t\t\t\t\tMin RH:\n\
In:  {: >7.2}%  Out:{: >7.2}%\t\tIn:   {: >7.2}%  Out: {: >7.2}%\n\
Max TDs:\t\t\t\tSensor 3\n\
High:{: >7.2}f  Low:{: >7.2}f\t\tTemp:{: >7.2}f   RH:  {: >7.2}%\n\
Min%{: >6.2} Max %{: >6.2} LDR: {}",
                inside.temp.get_cur(),
                inside.rh.get_cur(),
                outside.temp.get_cur(),
                outside.rh.get_cur(),
                self.get_delta(ReadingType::Temp),
                self.get_delta(ReadingType::Humidity),
                self.valve_status(),
                inside.temp.get_max(),
                outside.temp.get_max(),
                inside.temp.get_min(),
                outside.temp.get_min(),
                inside.rh.get_max(),
                outside.rh.get_max(),
                inside.rh.get_min(),
                outside.rh.get_min(),
                self.deltas.get_max(),
                self.deltas.get_min(),
                spare.temp.get_cur(),
                spare.rh.get_cur(),
                self.low_limit,
                self.high_limit,
                self.ldr
            )
        }
        pub fn get_inside_temp(&self) -> f32 {
            self.sensors.inside.get_reading(ReadingType::Temp, ReadingKind::Cur)
        }
    }
    /// Return a new empty EvapData
    pub fn new() -> EvapData{
        EvapData { 
            sensors: SensorArray { 
                inside: sensor::new("inside".to_string()),
                outside: sensor::new("outside".to_string()),
                spare: sensor::new("spare".to_string()),
            },
            low_limit: 0.0,
            high_limit: 0.0,
            ldr: -500,
            valve_status: -1,
            deltas:temp::new()
        }
    }
    /// Validates the serial parser string vec.  First 6 are temps and RH.
    /// LDR readings are i32,  Valid range needs checked and added.
    /// Valve status is 0, 1, or 2 for closed open, and sensing.
    fn validate(vals: &Vec<String>) -> bool {
        for val in &vals[0..7] {
            match val.parse::<f32>() {
            Ok(_) => {
                continue;
            }
            _ => return false
            }
        }
        match &vals[8].parse::<i32>() {
            Ok(_) => (),
            _ => return false
        }
        match &vals[9].parse::<i8>() {
            Ok(_) => (),
            _ => return false
        }
        true
    }
}