/// Deconstantizes a `String`
///
/// #Examples
/// ```
/// use inflector::string::deconstantize::deconstantize;
///
///
/// // deconstantize_Bar_as_Bar() {
///     let mock_string: String = "Bar".to_string();
///     let expected_string: String = "".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::deconstantize::deconstantize;
///
///
/// // deconstantize_namespace_Bar_as_Bar() {
///     let mock_string: String = "::Bar".to_string();
///     let expected_string: String = "".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::deconstantize::deconstantize;
///
///
/// // deconstantize_Foo_namespace_Bar_as_Bar() {
///     let mock_string: String = "Foo::Bar".to_string();
///     let expected_string: String = "Foo".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::deconstantize::deconstantize;
///
///
/// // deconstantize_Test_namespace_Foo_namespace_Bar_as_Bar() {
///     let mock_string: String = "Test::Foo::Bar".to_string();
///     let expected_string: String = "Foo".to_string();
///     let asserted_string: String = deconstantize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
use cases::classcase::to_class_case;

pub fn deconstantize<'a>(non_deconstantized_string: String) -> String {
    if non_deconstantized_string.contains("::") {
        let split_string: Vec<&str> = non_deconstantized_string.split("::").collect();
        if split_string.len() > 1 {
            return format!("{}", to_class_case(split_string[split_string.len() - 2].to_string()));
        } else {
            return "".to_string();
        }
    } else {
        return "".to_string();
    }
}
