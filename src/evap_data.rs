pub mod evap_data {
    use crate::sensors::readings::{ReadingKind, ReadingType};
    use crate::sensors::sensor;
    use crate::temp::temp;
    use chrono::Local;
    use serialport::SerialPort;
    use std::fs::File;
    use std::io::{Seek, Write};

    pub struct SensorArray {
        //#[derive(Debug)]
        inside: sensor::Sensor,
        outside: sensor::Sensor,
        interior: sensor::Sensor,
    }

    pub struct EvapData {
        sensors: SensorArray,
        low_limit: f32,
        high_limit: f32,
        _set_point: f32,
        on_point: f32,
        off_point: f32,
        pub water_call: i32,
        pub fan_call: i32,
        delay_start: i64,
        delay_time: i64,
        fan_file: Option<File>,
        ldr: i32,             //Not working, maybe
        pub valve_status: i8, //0-2 normal. oThers are failures
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
                    self.sensors.interior.update(
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
        /// Returns the current temperature change across the media.  Can return NAN if currently unset.
        fn get_delta(&self, reading_type: ReadingType) -> f32 {
            let outside_cur = self
                .sensors
                .outside
                .get_reading(reading_type.clone(), ReadingKind::Cur);
            let inside_cur = self
                .sensors
                .inside
                .get_reading(reading_type, ReadingKind::Cur);
            inside_cur - outside_cur
        }
        /// Get the current valve status.  What indicates either not written to, or an error has occured.
        fn valve_status(&self) -> String {
            match self.valve_status {
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
            self.sensors.interior.clear();
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
            format!(
                "{}: {: >7.2}f {: >7.2}%\r\n\
In:  {: >7.2}f {: >7.2}%\r\n\
Diff:{: >7.2}f {: >7.2}%\r\n\
Dew: {: >7.2}f \r\n\
\n\
Fan Call: {}\n\
Water Call: {}\n\
Valve: {}\n\
\n\
Max Temps:\t\t\t\tMin Temps:\n\
In:  {: >7.2}f  Out:{: >7.2}f\t\tIn:   {: >7.2}f   Out: {: >7.2}f\n\
Max RH:\t\t\t\t\tMin RH:\n\
In:  {: >7.2}%  Out:{: >7.2}%\t\tIn:   {: >7.2}%   Out: {: >7.2}%\n\
Max TDs:\t\t\t\tInterior:\n\
High:{: >7.2}f  Low:{: >7.2}f\t\tTemp: {: >7.2}f   RH:  {: >7.2}%\n\
Min%{: >6.2} Max %{: >6.2} LDR: {}\t\tMin:  {: >7.2}f   Max: {: >7.2}f",
                self.sensors.outside.get_id(),
                outside.temp.get_cur(),
                outside.rh.get_cur(),
                inside.temp.get_cur(),
                inside.rh.get_cur(),
                self.get_delta(ReadingType::Temp),
                self.get_delta(ReadingType::Humidity),
                temp::dew_point(outside.temp.get_cur(), outside.rh.get_cur()),
                self.get_fan_call(),
                self.water_call,
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
                self.sensors
                    .interior
                    .get_reading(ReadingType::Temp, ReadingKind::Cur),
                self.sensors
                    .interior
                    .get_reading(ReadingType::Humidity, ReadingKind::Cur),
                self.low_limit,
                self.high_limit,
                self.ldr,
                self.sensors
                    .interior
                    .get_reading(ReadingType::Temp, ReadingKind::Min),
                self.sensors
                    .interior
                    .get_reading(ReadingType::Temp, ReadingKind::Max),
            )
        }
        pub fn get_inside_temp(&self) -> f32 {
            self.sensors
                .inside
                .get_reading(ReadingType::Temp, ReadingKind::Cur)
        }
        pub fn get_inside_temp_2(&self) -> f32 {
            self.sensors
                .interior
                .get_reading(ReadingType::Temp, ReadingKind::Cur)
        }
        pub fn _get_outside_temp(&self) -> f32 {
            self.sensors
                .interior
                .get_reading(ReadingType::Temp, ReadingKind::Cur)
        }
        pub fn get_low_limit(&self) -> f32 {
            self.low_limit
        }
        pub fn get_high_limit(&self) -> f32 {
            self.high_limit
        }
        /// Sets the fan call to on,  if true sets a delay for the fan and
        /// starts a water call.
        pub fn set_fan_call(&mut self, call: String, sp: &mut Box<dyn SerialPort + 'static>) {
            let ts = Local::now().timestamp();
            if call == "on" {
                if self.fan_call == 0 {
                    self.delay_start = ts;
                    self.set_water_call(sp, 1);
                    self.fan_call = 2;
                    std::process::exit(1);
                }
                //No need to check outside temp,  the Arduino does this on its own.
                else if self.delay_start + self.delay_time > ts {
                    self.fan_call = 1;
                }
            } else {
                self.set_water_call(sp, 0);
                self.fan_call = 0;
            }

            let mut fan_file = match self.fan_file.as_ref() {
                Some(file) => file,
                None => return,
            };
            fan_file.seek(std::io::SeekFrom::Start(0)).unwrap();
            let bw = match fan_file.write(self.get_fan_call().as_bytes()) {
                Ok(n) => n as u64,
                Err(_) => 0u64,
            };
            fan_file.set_len(bw).unwrap();
            fan_file.flush().unwrap();
        }
        /// Enables and disables the water call.
        pub fn set_water_call(&mut self, sp: &mut Box<dyn SerialPort + 'static>, call: i32) {
            if call == self.water_call {
                return;
            }
            match call {
                0 | 2 => {
                    self.water_call = call;
                    sp.write("C,0\n".as_bytes()).unwrap();
                }
                1 | 3 => {
                    self.water_call = call;
                    sp.write("C,1\n".as_bytes()).unwrap();
                }
                _ => (),
            }
        }
        /// Returns the current fan call as in an i32, 0 = off, 1 = on, 2 = wait
        pub fn get_fan_call(&self) -> String {
            match self.fan_call {
                0 => "off".to_string(),
                1 => "on".to_string(),
                2 => "wait".to_string(),
                _ => "off".to_string(),
            }
        }
        /// Returns the water call as an i32,  0 = off, 1 = on, 2 = locked off, 3 = locked on
        pub fn _get_water_call(&self) -> i32 {
            self.water_call
        }
        pub fn update_status(&mut self, call: String, sp: &mut Box<dyn SerialPort + 'static>) {
            if self.get_inside_temp_2() > self.on_point || call == "on".to_string() {
                self.set_fan_call("on".to_owned(), sp);
            } else if self.off_point > self.get_inside_temp_2() {
                self.set_fan_call("off".to_owned(), sp);
            }
        }
        pub fn add_fan_file(&mut self, file: File) {
            match self.fan_file.take() {
                Some(f) => {
                    let oldfile = f;
                    self.fan_file = Some(file);
                    drop(oldfile)
                }
                None => {
                    self.fan_file = Some(file);
                }
            }
        }
    }
    /// Return a new empty EvapData
    pub fn new(set_point: f32, delay_time: i64) -> EvapData {
        EvapData {
            sensors: SensorArray {
                inside: sensor::new("In".to_string()),
                outside: sensor::new("Out".to_string()),
                interior: sensor::new("Inside".to_string()),
            },
            low_limit: 91.0,
            high_limit: 96.0,
            _set_point: set_point,
            on_point: set_point + 1.5,
            off_point: set_point - 1.5,
            fan_call: 0,
            fan_file: None,
            delay_start: 0, // SEconds to delay
            delay_time: delay_time,
            water_call: 1,
            ldr: -500,
            valve_status: -1,
            deltas: temp::new(),
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
                _ => return false,
            }
        }
        match &vals[8].parse::<i32>() {
            Ok(_) => (),
            _ => return false,
        }
        match &vals[9].parse::<i8>() {
            Ok(_) => (),
            _ => return false,
        }
        true
    }
}
