pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_minutes_in_oven() {
        assert_eq!(expected_minutes_in_oven(), 40);
    }

    #[test]
    fn test_remaining_minutes_in_oven() {
        assert_eq!(remaining_minutes_in_oven(30), 10);
        assert_eq!(remaining_minutes_in_oven(40), 0);
        assert_eq!(remaining_minutes_in_oven(50), -10);
    }

    #[test]
    fn test_preparation_time_in_minutes() {
        assert_eq!(preparation_time_in_minutes(1), 2);
        assert_eq!(preparation_time_in_minutes(4), 8);
    }

    #[test]
    fn test_elapsed_time_in_minutes() {
        assert_eq!(elapsed_time_in_minutes(1, 30), 32);
        assert_eq!(elapsed_time_in_minutes(2, 20), 24);
        assert_eq!(elapsed_time_in_minutes(3, 40), 46);
    }
}
