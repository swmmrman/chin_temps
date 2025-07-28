pub mod temp {
    pub struct Temp {
        min_temp: f32,
        max_temp: f32,
        cur_temp: f32,
    }

    impl Temp {
        /// Updates the temp struct,  Checks for large jumps due to sensor glitch.
        /// Sets all values if currently NANs.
        /// Sets min/max as needed.
        /// Current temp is always updated incase of temporary sensor failure.
        pub fn update(&mut self, new_temp: f32) {
            let temp_diff = (self.cur_temp - &new_temp).abs() < 2.0;
            self.cur_temp = new_temp;
            // Clear NaNs first
            if self.cur_temp.is_nan() && temp_diff { self.cur_temp = new_temp }
            if self.min_temp.is_nan() && temp_diff { self.min_temp = new_temp }
            if self.max_temp.is_nan() && temp_diff { self.max_temp = new_temp }
            // Then check min/max
            if self.cur_temp < self.min_temp { self.min_temp = new_temp; }
            if self.cur_temp > self.max_temp { self.max_temp = new_temp; }
        }
        /// Sets the min, max, and current temp.  No checking or validation is performed.
        pub fn update_unchecked(&mut self, min: f32, max: f32, cur: f32) {
            self.min_temp = min;
            self.max_temp = max;
            self.cur_temp = cur;
        }
        /// Returns the current temp.  NAN if not set
        pub fn get_cur(&self) -> f32 {
            self.cur_temp
        }
        /// Returns the current temp.  NAN if not set
        pub fn get_min(&self) -> f32 {
            self.min_temp
        }
        /// Returns the current temp.  NAN if not set
        pub fn get_max(&self) -> f32 {
            self.max_temp
        }
        /// Clears the temp strut,  Sets all values to NAN.
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
    /// Convert fahrenheit to celcius.  Always returns.
    pub fn f_to_c(temp: f32) -> f32{
        (5.0/9.0)* temp - 32.0
    }

    pub fn c_to_f(temp: f32) -> f32 {
        (9.0/5.0) * (temp + 32.0)
    }

    pub fn dew_point(temp: f32, rh: f32) -> f32 {
        // Magnus-Tetens formula
        // Td = (b * α(T,RH)) / (a - α(T,RH))
        // Td = dewpoint temp
        // RH = RH as decimal.
        // a = 17.27 
        // b = 237.7c
        // T = temp in c
        // α = (T,RH) = ((a * T) / (b + T)) + ln(RH)
        let a = 17.27;
        let b = 237.7;
        let rhd = rh / 100.0;
        let temp_c = f_to_c(temp);
        let dp = (b * alpha(temp_c, rhd))/ (a-alpha(temp_c, rhd));
        c_to_f(dp)
    }
    fn alpha(t: f32, rh: f32) -> f32 {
        let a = 17.27;
        let b = 237.7;
        ((a * t) / (b + t)) + (rh).ln()
    }
}