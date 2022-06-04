// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    return error_rate(speed) * cars_produced(speed);
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed) as u32 / 60;
}

fn error_rate(speed: u8) -> f64 {
    if speed >= 9 {
        return 0.77;
    } else if speed >= 5 {
        return 0.9;
    }

    return 1.0;
}

fn cars_produced(speed: u8) -> f64 {
    return 221.0 * speed as f64;
}
