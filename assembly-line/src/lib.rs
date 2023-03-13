pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {speed}")
    let rate: f64 = speed as f64 * 221.0; 
    match speed {
        1..=4 => return rate,
        5..=8 => return rate * 0.9,
        _ => return rate * 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {speed}")
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
