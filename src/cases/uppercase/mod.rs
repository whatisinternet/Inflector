/// Converts a `String` to uppercase `String`
///
/// #Examples
/// ```
///     use inflector::cases::uppercase::to_upper_case;
///     let mock_string: String = "FoObAR".to_string();
///     let expected_string: String = "FOOBAR".to_string();
///     let asserted_string: String = to_upper_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_upper_case(non_upper_string: String) -> String {
    non_upper_string
        .chars()
        .flat_map(char::to_uppercase)
        .collect()
}

/// Determines if a `String` is UPPERCASE
///
/// #Examples
/// ```
///     use inflector::cases::uppercase::is_upper_case;
///     let mock_string: String = "FOOBARSTRINGTHATISREALLYREALLYLONG".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::uppercase::is_upper_case;
///     let mock_string: String = "foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::uppercase::is_upper_case;
///     let mock_string: String = "Foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```

pub fn is_upper_case(test_string: String) -> bool{
    test_string == to_upper_case(test_string.to_owned())
}
