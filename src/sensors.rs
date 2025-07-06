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
        pub fn get_all(&self) -> super::readings::Readings {
            let mut out = super::readings::new();
            out.temp.update(self.temperature.get_min());
            out.temp.update(self.temperature.get_max());
            out.temp.update(self.temperature.get_cur());
            out.rh.update(self.humidity.get_max());
            out.rh.update(self.humidity.get_min());
            out.rh.update(self.humidity.get_cur());
            out
        }

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
    }

    fn new(identifier: String) -> Sensor {
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