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
        pub fn update(&mut self, vals: Vec<String>) {
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
        fn get_delta_t(&self) -> f32 {
            self.temp2.get_cur() - self.temp1.get_cur()
        }
        fn get_delta_h(&self) -> f32 {
            self.humid2.get_cur() - self.humid1.get_cur()
        }
        fn valve_status(&self) -> String {
            match self.valve_status{
                0 => "Off  ".to_string(),
                1 => "On   ".to_string(),
                2 => "Wait ".to_string(),
                _ => "What?".to_string(),
            }
        }
        fn clear(&mut self) {
            self.temp1.clear();
            self.temp2.clear();
            self.temp3.clear();
            self.humid1.clear();
            self.humid2.clear();
            self.humid3.clear();
            self.valve_status = -1;
            self.deltas.clear();
        }
    }

    pub fn new() -> EvapData{
        EvapData { temp1: Temp::new(), temp2: Temp::new(), temp3: Temp::new(), humid1: RH::new(), humid2: RH::new(), humid3: RH::new(), ldr: -500, valve_status: -1, deltas: Temp::new() }
    }
}