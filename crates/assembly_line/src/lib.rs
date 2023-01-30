const BASE_RATE: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0..=4 => BASE_RATE * (speed as f64),
        5..=8 => BASE_RATE * (speed as f64) * 0.9,
        9..=10 => BASE_RATE * (speed as f64) * 0.77,
        unknown_speed => panic!("Speed {} is not in range 0..=10", unknown_speed),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_rate_per_hour() {
        assert_eq!(production_rate_per_hour(0), 0.0);

        assert_eq!(production_rate_per_hour(1), 221.0);
        assert_eq!(production_rate_per_hour(4), 884.0);

        assert_eq!(production_rate_per_hour(5), 994.5);
        assert_eq!(production_rate_per_hour(8), 1591.2);

        assert_eq!(production_rate_per_hour(9), 1531.53);
        assert_eq!(production_rate_per_hour(10), 1701.7);
    }

    #[test]
    #[should_panic]
    fn test_production_rate_per_hour_invalid_speed() {
        production_rate_per_hour(11);
    }

    #[test]
    fn test_working_items_per_minute() {
        assert_eq!(working_items_per_minute(0), 0);

        assert_eq!(working_items_per_minute(1), 3);
        assert_eq!(working_items_per_minute(4), 14);

        assert_eq!(working_items_per_minute(5), 16);
        assert_eq!(working_items_per_minute(8), 26);

        assert_eq!(working_items_per_minute(9), 25);
        assert_eq!(working_items_per_minute(10), 28);
    }

    #[test]
    #[should_panic]
    fn test_working_items_per_minute_invalid_speed() {
        working_items_per_minute(11);
    }
}
