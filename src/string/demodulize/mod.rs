#[cfg(not(feature = "without_full"))]
use cases::classcase::to_class_case;

#[cfg(not(feature = "without_full"))]
/// Demodulize a `String`
///
/// ```
///     use inflector::string::demodulize::demodulize;
///     let mock_string: String = "Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::demodulize::demodulize;
///     let mock_string: String = "::Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::demodulize::demodulize;
///     let mock_string: String = "Foo::Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::demodulize::demodulize;
///     let mock_string: String = "Test::Foo::Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn demodulize(non_demodulize_string: String) -> String {
    if non_demodulize_string.contains("::") {
        let split_string: Vec<&str> = non_demodulize_string.split("::").collect();
        to_class_case(split_string[split_string.len() - 1].to_string())
    } else {
        non_demodulize_string
    }
}
