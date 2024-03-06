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
    fn test_validate_string_length_ok(#[case] value: &str, #[case] min: usize, #[case] max: usize) {
        assert!(validate_string_length(value, min, max).is_ok());
    }

    #[rstest]
    #[case::min_1_empty("", 1, 10, "Value must be between 1 and 10 characters long: 0")]
    #[case::max_50_over(
        "123456789012345678901234567890123456789012345678901",
        1,
        50,
        "Value must be between 1 and 50 characters long: 51"
    )]
    fn test_validate_string_length_err(
        #[case] value: &str,
        #[case] min: usize,
        #[case] max: usize,
        #[case] expected_err: &str,
    ) {
        assert_eq!(
            validate_string_length(value, min, max).unwrap_err(),
            expected_err
        );
    }

    #[rstest]
    #[case::min_1(1, 1, 10)]
    #[case::max_50(50, 1, 50)]
    fn test_validate_numeric_range_ok(#[case] value: u32, #[case] min: u32, #[case] max: u32) {
        assert!(validate_numeric_range(value, min, max).is_ok());
    }

    #[rstest]
    #[case::min_1(0, 1, 10, "Value must be between 1 and 10: 0")]
    #[case::max_50_over(51, 1, 50, "Value must be between 1 and 50: 51")]
    fn test_validate_numeric_range_err(
        #[case] value: u32,
        #[case] min: u32,
        #[case] max: u32,
        #[case] expected_err: &str,
    ) {
        assert_eq!(
            validate_numeric_range(value, min, max).unwrap_err(),
            expected_err
        );
    }
}
