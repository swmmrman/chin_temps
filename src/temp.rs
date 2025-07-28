pub mod temp {
    pub struct Temp {
        min_temp: f32,
        max_temp: f32,
        cur_temp: f32,
    }

    impl Temp {
        pub fn update(&mut self, new_temp: f32) {
            let temp_diff = (self.cur_temp - &new_temp).abs() < 2.0;
            self.cur_temp = new_temp;
            //Clear NaNs first
            if self.cur_temp.is_nan() && temp_diff { self.cur_temp = new_temp }
            if self.min_temp.is_nan() && temp_diff { self.min_temp = new_temp }
            if self.max_temp.is_nan() && temp_diff { self.max_temp = new_temp }
            //Then check min/max
            if self.cur_temp < self.min_temp { self.min_temp = new_temp; }
            if self.cur_temp > self.max_temp { self.max_temp = new_temp; }
        }
        //Sets the min, max, and current temp.  No checking or validation is performed.
        pub fn update_unchecked(&mut self, min: f32, max: f32, cur: f32) {
            self.min_temp = min;
            self.max_temp = max;
            self.cur_temp = cur;
        }
        //Returns the current temp.  NAN if not set
        pub fn get_cur(&self) -> f32 {
            self.cur_temp
        }
        //Returns the current temp.  NAN if not set
        pub fn get_min(&self) -> f32 {
            self.min_temp
        }
        //Returns the current temp.  NAN if not set
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