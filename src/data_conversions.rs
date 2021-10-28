/// Converts String to a vec<i64>
///
/// The string shall only have one integer per line.
///
/// ```
/// let valid_input     = "2\n3\n-7";
/// let invalid_input_1 = "2,3,-7";
/// let invalid_input_2 = "2 3 -7";
///
/// assert_eq!(thrigger_support::data_conversions::string_to_vec_i64(valid_input), [2,3,-7]);
/// assert_eq!(thrigger_support::data_conversions::string_to_vec_i64(invalid_input_1), []);
/// assert_eq!(thrigger_support::data_conversions::string_to_vec_i64(invalid_input_2), []);
/// ```
pub fn string_to_vec_i64(input: &str) -> Vec<i64> {
    input.lines()
        .filter_map(|s| match s.parse::<i64>() {
            Ok(i)  => Some(i),
            Err(_) => None,
        }).collect()
}

fn string_to_str_vec(input: &String) -> Vec<&str> {
    input.lines().filter_map(|s| match s {
        ""=>None,
        _=>Some(s),}).collect()
}


