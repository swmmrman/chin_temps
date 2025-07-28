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
        //Returns the current RH.  NAN if not set
        pub fn get_cur(&self) -> f32 {
            self.cur_rh
        }
        //Returns the min RH.  NAN if unset.
        pub fn get_min(&self) -> f32 {
            self.min_rh
        }
        //Returns the max RH. NAN if unset.
        pub fn get_max(&self) -> f32 {
            self.max_rh
        }
        //Checked update of the temp struct.
        //Sets all values to the new if NAN in min_rh
        //Updates min and max as needed.
        pub fn update(&mut self, new_val: f32) {
            let rh_diff= (self.cur_rh - new_val).abs();
            if self.min_rh.is_nan() {
                self.min_rh = new_val;
                self.max_rh = new_val;
            }
            if self.min_rh > new_val && rh_diff < 2.0 {
                self.min_rh = new_val;
            }
            if self.max_rh < new_val && rh_diff < 2.0 {
                self.max_rh = new_val;
            }
            self.cur_rh = new_val;
        }
        //Sets the min, max, and current RH.  No checking or validation is performed.
        pub fn update_unchecked(&mut self, min: f32, max: f32, cur: f32) {
            self.min_rh = min;
            self.max_rh = max;
            self.cur_rh = cur;
        }
    }
    //Create a new RH stuct with NAN values.
    pub fn new() -> RH{
        RH {
            min_rh: f32::NAN,
            max_rh: f32::NAN,
            cur_rh: f32::NAN,
        }
    }
}