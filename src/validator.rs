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

pub fn validate_numeric_range<T: PartialOrd + Copy + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
) -> Result<(), String> {
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
    #[should_panic(expected = "Value must be between 1 and 10 characters long: 0")]
    #[case::min_less("", 1, 10)]
    #[should_panic(expected = "Value must be between 1 and 50 characters long: 51")]
    #[case::max_over("123456789012345678901234567890123456789012345678901", 1, 50)]
    fn test_validate_string_length(#[case] value: &str, #[case] min: usize, #[case] max: usize) {
        validate_string_length(value, min, max).unwrap();
    }

    #[rstest]
    #[case::min_1(1, 1, 10)]
    #[case::max_50(50, 1, 50)]
    #[should_panic(expected = "Value must be between 1 and 10: 0")]
    #[case::min_less(0, 1, 10)]
    #[should_panic(expected = "Value must be between 1 and 50: 51")]
    #[case::max_over(51, 1, 50)]
    fn test_validate_numeric_range(#[case] value: u32, #[case] min: u32, #[case] max: u32) {
        validate_numeric_range(value, min, max).unwrap();
    }
}
