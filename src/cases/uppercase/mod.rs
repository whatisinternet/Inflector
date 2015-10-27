use std::ascii::*;
use regex::Regex;

/// Converts a `String` to uppercase `String`
///
/// #Examples
/// ```
/// use inflector::cases::uppercase::to_upper_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn upcase_FoObAR_as_foobar() {
///     let mock_string: String = "FoObAR".to_string();
///     let expected_string: String = "FOOBAR".to_string();
///     let asserted_string: String = to_upper_case(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
pub fn to_upper_case<'a>(non_camelized_string: String) -> String {
    let mut result:String = "".to_string();
    for character in non_camelized_string.chars() {
        result = format!("{}{}", result, character.to_ascii_uppercase());
    }
    return result
}

/// Determines if a `String` is UPPERCASE
///
/// #Examples
/// ```
/// use inflector::cases::uppercase::is_upper_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_upper_case_when_uppercase() {
///     let mock_string: String = "FOOBARSTRINGTHATISREALLYREALLYLONG".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == true);
/// }
/// ```
/// ```
/// use inflector::cases::uppercase::is_upper_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_truthy_value_for_is_upper_case_when_uppercase() {
///     let mock_string: String = "foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::uppercase::is_upper_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_upper_case_when_Startcased() {
///     let mock_string: String = "Foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```

pub fn is_upper_case<'a>(test_string: String) -> bool{
    let upper_matcher = Regex::new(r"^[A-Z| |_|-]+$").unwrap();
    let mut is_upper_case = false;
    if upper_matcher.is_match(test_string.as_ref()){
        is_upper_case = true;
    }
    return is_upper_case;
}
