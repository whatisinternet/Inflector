use std::ascii::*;
use regex::Regex;

use cases::lowercase::to_lower_case;

/// Converts a `String` to snake_case `String`
///
/// #Examples
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_foo_bar_as_foo_bar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_HTTP_Foo_space_bar_as_foo_bar() {
///     let mock_string: String = "HTTP Foo bar".to_string();
///     let expected_string: String = "http_foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_Foo_space_bar_as_foo_bar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_Foo_space_Bar_as_foo_bar() {
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_FooBar_as_foo_bar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_FOO_BAR_as_foo_bar() {
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_fooBar_as_foo_bar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::snakecase::to_snake_case;
///
///
/// // snake_case_fooBar3_as_foo_bar() {
///     let mock_string: String = "fooBar3".to_string();
///     let expected_string: String = "foo_bar_3".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_snake_case<'a>(non_snake_case_string: String) -> String {
    if non_snake_case_string.contains(" ")
        || non_snake_case_string.contains("_")
        || non_snake_case_string.contains("-") {
        return to_snake_from_sentence_or_kebab(non_snake_case_string);
    } else {
        return to_snake_from_camel_or_class(non_snake_case_string);
    }
}
    fn to_snake_from_camel_or_class <'a>(non_snake_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_snake_case_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{}_{}", result, character.to_ascii_lowercase());
            } else {
                result = format!("{}{}", result, character.to_ascii_lowercase());
                first_character = false;
            }
        }
        return result
    }

    fn to_snake_from_sentence_or_kebab<'a>(non_snake_case_string: String) -> String {
        return to_lower_case(non_snake_case_string.replace(" ", "_").replace("-", "_"));
    }

/// Determines of a `String` is snake_case
///
/// #Examples
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_falsey_value_for_is_snake_case_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_falsey_value_for_is_snake_case_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_falsey_value_for_is_snake_case_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_falsey_value_for_is_snake_case_when_title() {
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_falsey_value_for_is_snake_case_when_screaming() {
///     let mock_string: String = "FOO_BAR_IS_A_REALLY_REALLY_LONG_STRING".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_falsey_value_for_is_snake_case_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_truthy_value_for_is_snake_case_when_snake() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_truthy_value_for_is_snake_case_when_snake_with_a_number() {
///     let mock_string: String = "foo_bar1_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
/// use inflector::cases::snakecase::is_snake_case;
///
///
/// // returns_truthy_value_for_is_snake_case_when_snake_with_a_number_snaked() {
///     let mock_string: String = "foo_bar_1_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_snake_case<'a>(test_string: String) -> bool{
    let snake_matcher = Regex::new(r"(?:[^-|^ ]?=^|[_])([a-z]+)").unwrap();
    if snake_matcher.is_match(test_string.as_ref()){
        return true;
    }
    return false;
}
