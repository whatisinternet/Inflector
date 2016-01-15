use std::ascii::*;
use regex::Regex;

use cases::snakecase::to_snake_case;

/// Converts a `String` to ClassCase `String`
///
/// #Examples
/// ```
/// use inflector::cases::classcase::to_class_case;
///
/// // ClassCase_FooBar_as_FooBar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::to_class_case;
/// // ClassCase_Foo_Bar_as_FooBar() {
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::to_class_case;
///
/// // ClassCase_foo_dash_bar_as_FooBar() {
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::to_class_case;
///
/// // ClassCase_fooBar_as_FooBar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::to_class_case;
///
/// // ClassCase_FOO_BAR_as_FooBar() {
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::to_class_case;
///
/// // ClassCase_foo_bar_as_FooBar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::to_class_case;
///
/// // ClassCase_Foo_space_bar_as_FooBar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_class_case<'a>(non_class_case_string: String) -> String {
    return to_class_from_snake(to_snake_case(non_class_case_string));
}
    fn to_class_from_snake<'a>(non_class_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut new_word: bool = true;

        for character in non_class_case_string.chars() {
            if character.to_string() == "_" {
                new_word = true;
            } else if new_word {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                new_word = false;
            } else {
                result = format!("{}{}", result, character.to_ascii_lowercase());
            }
        }
        return result
    }

/// Determines if a `String` is ClassCase `bool`
///
/// #Examples
/// ```
/// use inflector::cases::classcase::is_class_case;
///
/// // returns_truthy_value_for_is_class_case_when_start_cased() {
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
/// // returns_falsey_value_for_is_class_case_when_downcased() {
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_truthy_value_for_is_class_case_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_falsey_value_for_is_class_case_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_falsey_value_for_is_class_case_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_falsey_value_for_is_class_case_when_screaming_snake() {
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_falsey_value_for_is_class_case_when_snake() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_falsey_value_for_is_class_case_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
/// use inflector::cases::classcase::is_class_case;
///
///
/// // returns_falsey_value_for_is_class_case_when_title() {
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_class_case<'a>(test_string: String) -> bool{
    let class_matcher = Regex::new(r"(^[A-Z])([^-|^_|^ ]*[a-z0-9]+)").unwrap();
    let space_matcher = Regex::new(r" +").unwrap();
    if class_matcher.is_match(test_string.as_ref()) && !space_matcher.is_match(test_string.as_ref()){
        return true;
    }
    return false;
}
