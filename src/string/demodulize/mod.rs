/// Demodulize a `String`
///
/// #Examples
/// ```
/// use inflector::string::demodulize::demodulize;
///
/// #[test] #[allow(non_snake_case)]
/// fn demodulize_Bar_as_Bar() {
///     let mock_string: String = "Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::demodulize::demodulize;
///
/// #[test] #[allow(non_snake_case)]
/// fn demodulize_namespace_Bar_as_Bar() {
///     let mock_string: String = "::Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::demodulize::demodulize;
///
/// #[test] #[allow(non_snake_case)]
/// fn demodulize_Foo_namespace_Bar_as_Bar() {
///     let mock_string: String = "Foo::Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::demodulize::demodulize;
///
/// #[test] #[allow(non_snake_case)]
/// fn demodulize_Test_namespace_Foo_namespace_Bar_as_Bar() {
///     let mock_string: String = "Test::Foo::Bar".to_string();
///     let expected_string: String = "Bar".to_string();
///     let asserted_string: String = demodulize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
use cases::classcase::to_class_case;

pub fn demodulize<'a>(non_demodulize_string: String) -> String {
    if non_demodulize_string.contains("::") {
        let split_string: Vec<&str> = non_demodulize_string.split("::").collect();
        return format!("{}", to_class_case(split_string[split_string.len() - 1].to_string()));
    } else {
        return non_demodulize_string;
    }
}

#[cfg(test)]
mod demodulize_test;
