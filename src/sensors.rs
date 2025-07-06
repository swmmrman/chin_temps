use crate::temp::temp;
use crate::rh::rh;

struct sensor {
    id: String,
    temperature: temp::Temp,
    humidity: rh::RH
}