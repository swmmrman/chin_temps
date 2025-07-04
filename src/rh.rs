pub mod rh {
    pub struct RH {
        min_rh: f32,
        max_rh: f32,
        cur_rh: f32
    }

    impl RH {
        pub fn clear(&mut self) {
            self.min_rh = f32::NAN;
            self.max_rh = f32::NAN;
            self.cur_rh = f32::NAN;
        }
        pub fn get_cur(&self) -> f32 {
            self.cur_rh
        }
        pub fn get_min(&self) -> f32 {
            self.min_rh
        }
        pub fn get_max(&self) -> f32 {
            self.max_rh
        }
        pub fn update(&mut self, new_val: f32) {
            if self.min_rh.is_nan() {
                self.min_rh = new_val;
                self.max_rh = new_val;
            }
            else if self.min_rh > new_val {
                self.min_rh = new_val;
            }
            else if self.max_rh < new_val {
                self.max_rh = new_val;
            }
            self.cur_rh = new_val;
        }
    }

    pub fn new() -> RH{
        RH {
            min_rh: f32::NAN,
            max_rh: f32::NAN,
            cur_rh: f32::NAN,
        }
    }
}