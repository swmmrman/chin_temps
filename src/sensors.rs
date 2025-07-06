use crate::temp::temp;
use crate::rh::rh;

struct Sensor {
    id: String,
    temperature: temp::Temp,
    humidity: rh::RH
}
struct Readings {
    temp: temp::Temp,
    rh: rh::RH,
}

enum ReadingType{
    Temp,
    Humidity,
}

enum ReadingKind {
    Min,
    Max,
    Cur
}
impl Sensor {
    fn get_all(&self) -> Readings {
        Readings {
            temp: temp::Temp{min_temp: get_reading(ReadingType::temp,)}
            rh: self.humidity.clone(),
        }
    }
    fn get_reading(&self,  reading: ReadingType, kind: ReadingKind) -> f32 {
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