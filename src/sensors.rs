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
    fn get_reading(&self,  reading: r_type, kind: r_kind) {
        
    }
}