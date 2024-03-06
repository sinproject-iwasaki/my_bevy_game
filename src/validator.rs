pub fn validate_string_length(value: &str, min: usize, max: usize) -> Result<(), String> {
    if (min..=max).contains(&value.len()) {
        Ok(())
    } else {
        Err(format!(
            "Value must be between {} and {} characters long: {}",
            min,
            max,
            value.len()
        ))
    }
}

pub fn validate_numeric_range(value: u32, min: u32, max: u32) -> Result<(), String> {
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "Value must be between {} and {}: {}",
            min, max, value
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::min_1("T", 1, 10)]
    #[case::max_50("12345678901234567890123456789012345678901234567890", 1, 50)]
    #[should_panic]
    #[case::min_1_empty("", 1, 10)]
    #[should_panic]
    #[case::max_50_over("123456789012345678901234567890123456789012345678901", 1, 50)]
    fn test_validate_string_length(#[case] value: &str, #[case] min: usize, #[case] max: usize) {
        assert!(validate_string_length(value, min, max).is_ok());
    }

    #[rstest]
    #[case::min_1(1, 1, 10)]
    #[case::max_50(50, 1, 50)]
    #[should_panic]
    #[case::min_0(0, 1, 10)]
    #[should_panic]
    #[case::max_51(51, 1, 50)]
    fn test_validate_numeric_range(#[case] value: u32, #[case] min: u32, #[case] max: u32) {
        assert!(validate_numeric_range(value, min, max).is_ok());
    }
}
