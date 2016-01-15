use regex::Regex;

use cases::snakecase::to_snake_case;

/// Converts a `String` to kebab-case `String`
///
/// #Examples
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_foo_dash_bar_as_foo_dash_bar() {
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_FOO_BAR_as_foo_bar() {
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_foo_bar_as_foo_bar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_Foo_space_Bar_as_foo_bar() {
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_Foo_space_bar_as_foo_bar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_FooBar_as_foo_bar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::to_kebab_case;
///
///
/// // kebab_case_fooBar_as_foo_bar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn is_kebab_case<'a>(test_string: String) -> bool{
    let kebab_matcher = Regex::new(r"(?:[^_]?=^|[-])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    return kebab_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref())
}

/// Determines if a `String` is kebab-case
///
/// #Examples
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_truthy_value_for_is_kebab_case_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_falsey_value_for_is_kebab_case_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_falsey_value_for_is_kebab_case_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_falsey_value_for_is_kebab_case_when_screaming_snake() {
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_falsey_value_for_is_kebab_case_when_snake() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_falsey_value_for_is_kebab_case_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::kebabcase::is_kebab_case;
///
///
/// // returns_falsey_value_for_is_kebab_case_when_title() {
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn to_kebab_case<'a>(non_kebab_case_string: String) -> String {
    return to_kebab_from_snake(to_snake_case(non_kebab_case_string));
}
    fn to_kebab_from_snake<'a>(non_kebab_case_string: String) -> String {
        return non_kebab_case_string.replace("_", "-");
    }
