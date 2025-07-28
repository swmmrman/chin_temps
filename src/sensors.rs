pub mod sensor {
    use crate::temp::temp;
    use crate::rh::rh;
    use super::readings::{ReadingKind, ReadingType};
    
    pub struct Sensor {
        id: String,
        temperature: temp::Temp,
        humidity: rh::RH
    }

    impl Sensor {
        /// Returns all values from a sensor.  Values are returned in a readings struct.
        pub fn get_all(&self) -> super::readings::Readings {
            let mut out = super::readings::new();
            out.temp.update_unchecked(
                self.temperature.get_min(),
                self.temperature.get_max(),
                self.temperature.get_cur()
            );
            out.rh.update_unchecked(
                self.humidity.get_min(),
                self.humidity.get_max(),
                self.humidity.get_cur()
            );
            out
        }
        /// Returns a single reading of the requested type, and kind.
        pub fn get_reading(&self,  reading: ReadingType, kind: ReadingKind) -> f32 {
            match reading {
                self::ReadingType::Temp => {
                    match kind {
                        ReadingKind::Min => self.temperature.get_min(),
                        ReadingKind::Max => self.temperature.get_max(),
                        ReadingKind::Cur => self.temperature.get_cur()
                    }
                }
                self::ReadingType::Humidity => {
                    match kind {
                        self::ReadingKind::Min => self.humidity.get_min(),
                        self::ReadingKind::Max => self.humidity.get_max(),
                        self::ReadingKind::Cur => self.humidity.get_cur(),
                    }
                }
            }
        }
        /// Updates the temp and rh structs, updates are checked.
        pub fn update(&mut self, temp:f32, humidity:f32) {
            self.temperature.update(temp);
            self.humidity.update(humidity);
        }
        /// Calls the clear function of the temp and rh structs.
        pub fn clear(&mut self) {
            self.temperature.clear();
            self.humidity.clear();
        }
        pub fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    pub fn new(identifier: String) -> Sensor {
        Sensor {
            id: identifier,
            temperature: temp::new(),
            humidity: rh::new(),
        }
    }
}

pub mod readings {
    use crate::temp::temp;
    use crate::rh::rh;
    
    pub struct Readings {
        pub temp: temp::Temp,
        pub rh: rh::RH,
        
    }

    #[derive(Clone)]
    pub enum ReadingType{
        Temp,
        Humidity,
    }

    pub enum ReadingKind {
        Min,
        Max,
        Cur
    }

    pub fn new() -> Readings {
        Readings {
            temp: temp::new(),
            rh: rh::new(),
        }
    }
}