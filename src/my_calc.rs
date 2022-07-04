pub mod my_calc {
    pub fn calc_ctof(current_temperature: f64) -> f64 {
        current_temperature * 1.8 + 32.0
    }

    pub fn calc_ftoc(current_temperature: f64) -> f64 {
        (current_temperature -32.0) / 1.8
    }
}

