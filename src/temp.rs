pub mod temp {
    pub struct Temp {
        min_temp: f32,
        max_temp: f32,
        cur_temp: f32,
    }

    impl Temp {
        pub fn update(&mut self, new_temp: f32) {
            //filter off errors on the first temp
            //let temp_diff = (self.cur_temp - &new_temp).abs();
            //if temp_diff < 10.0 {self.cur_temp = new_temp;}
            self.cur_temp = new_temp;
            //Clear NaNs first
            if self.cur_temp.is_nan() { self.cur_temp = new_temp }
            if self.min_temp.is_nan() { self.min_temp = new_temp }
            if self.max_temp.is_nan() { self.max_temp = new_temp }
            //Then check min/max
            if self.cur_temp < self.min_temp { self.min_temp = new_temp; }
            if self.cur_temp > self.max_temp { self.max_temp = new_temp; }
        }
        pub fn get_cur(&self) -> f32 {
            self.cur_temp
        }
        pub fn get_min(&self) -> f32 {
            self.min_temp
        }
        pub fn get_max(&self) -> f32 {
            self.max_temp
        }
        pub fn clear(&mut self) {
            self.cur_temp = f32::NAN;
            self.min_temp = f32::NAN;
            self.max_temp = f32::NAN;
        }
    }

    pub fn new() -> Temp {
        Temp {
            min_temp: f32::NAN,
            max_temp: f32::NAN,
            cur_temp: f32::NAN
        }
    }
}