use crate::temp::temp;
use crate::rh::rh;

struct sensor {
    id: String,
    temperature: temp::Temp,
    humidity: rh::RH
}
struct readings {

}

enum r_type{
    temp,
    humidity,
}

enum r_kind {
    min,
    max,
    cur
}
impl sensor {
    fn get_all(&self) {

    }
    fn get_reading(&self,  reading: r_type, kind: r_kind) -> f32 {
        match reading {
            self::r_type::temp => {
                match kind {
                    self::r_kind::min => self.temperature.get_min(),
                    self::r_kind::max => self.temperature.get_max(),
                    self::r_kind::cur => self.temperature.get_cur()
                }
            }
            self::r_type::humidity => {
                match kind {
                    self::r_kind::min => self.humidity.get_min(),
                    self::r_kind::max => self.humidity.get_max(),
                    self::r_kind::cur => self.humidity.get_cur(),
                }
            }
        }
    }
}