use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;
use cases::snakecase::to_snake_case;

/// Converts a `String` to camelCase `String`
///
/// #Examples
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_fooBar_as_fooBar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_FOO_BAR_as_fooBar() {
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_Foo_Bar_as_fooBar() {
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_foo_bar_as_fooBar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_Foo_space_bar_as_fooBar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_foo_dash_bar_as_fooBar() {
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_FooBar_as_fooBar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::camelcase::to_camel_case;
///
/// // camelize_FooBar3_as_fooBar3() {
///     let mock_string: String = "FooBar3".to_string();
///     let expected_string: String = "fooBar3".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_camel_case(non_camelized_string: String) -> String {
    to_camel_from_snake(to_snake_case(non_camelized_string))
}

    fn to_camel_from_snake(non_camelized_string: String) -> String{
        let mut result:String = "".to_string();
        let mut new_word: bool = false;

        for character in non_camelized_string.chars() {
            if character.to_string() == "_" {
                new_word = true;
            } else if new_word {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                new_word = false;
            } else {
                result = format!("{}{}", result, character.to_ascii_lowercase());
            }
        }
        result
    }

/// Determines if a `String` is camelCase bool``
///
/// #Examples
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_start_cased() {
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
///
/// // returns_falsey_value_for_is_camel_case_when_lower_case() {
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_truthy_value_for_is_camel_case_when_camel_with_number() {
///     let mock_string: String = "fooBarIsAReallyReally3LongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_truthy_value_for_is_camel_case_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_screaming_snake() {
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_snake() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
/// use inflector::cases::camelcase::is_camel_case;
///
/// // returns_falsey_value_for_is_camel_case_when_title() {
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_camel_case(test_string: String) -> bool{
    let camel_matcher = Regex::new(r"(^|[A-Z])([^-|^_|^ ]*[a-z0-9]+[A-Z][a-z0-9]+)").unwrap();
    let kebab_snake_matcher = Regex::new(r"[-|_| ]").unwrap();
    if camel_matcher.is_match(test_string.as_ref())
        && !kebab_snake_matcher.is_match(test_string.as_ref())
        && !is_class_case(test_string){
            return true;
        }
    false
}
