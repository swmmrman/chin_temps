use crate::temp::temp;
use crate::rh::rh;

pub struct Sensor {
    id: String,
    temperature: temp::Temp,
    humidity: rh::RH
}
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
impl Sensor {
    pub fn get_all(&self) -> Readings {
        let mut out = new_readings();
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
                    self::ReadingKind::Min => self.temperature.get_min(),
                    self::ReadingKind::Max => self.temperature.get_max(),
                    self::ReadingKind::Cur => self.temperature.get_cur()
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
fn new(identifier: String) -> self::Sensor {
    Sensor {
        id: identifier,
        temperature: temp::new(),
        humidity: rh::new(),
    }
}
fn new_readings() -> Readings {
    Readings {
        temp: temp::new(),
        rh: rh::new(),
    }
}