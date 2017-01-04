use cases::classcase::to_class_case;

/// Deconstantizes a `String`
///
/// ```
///     use inflector::string::deconstantize::deconstantize;
///     let mock_string: String = "Bar".to_string();
///     let expected_string: String = "".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::deconstantize::deconstantize;
///     let mock_string: String = "::Bar".to_string();
///     let expected_string: String = "".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::deconstantize::deconstantize;
///     let mock_string: String = "Foo::Bar".to_string();
///     let expected_string: String = "Foo".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::deconstantize::deconstantize;
///     let mock_string: String = "Test::Foo::Bar".to_string();
///     let expected_string: String = "Foo".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn deconstantize(non_deconstantized_string: String) -> String {
    if non_deconstantized_string.contains("::") {
        let split_string: Vec<&str> = non_deconstantized_string.split("::").collect();
        if split_string.len() > 1 {
            to_class_case(split_string[split_string.len() - 2].to_string())
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}
