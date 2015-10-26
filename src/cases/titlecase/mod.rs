use std::ascii::*;
use regex::Regex;

use cases::snakecase::to_snake_case;

/// Converts a `String` to Title Case `String`
///
/// #Examples
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn title_case_Foo_bar_as_Foo_Bar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn title_case_FooBar_as_Foo_Bar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn title_case_fooBar_as_Foo_Bar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn title_case_foo_bar_as_Foo_Bar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn title_case_foo_dash_bar_as_Foo_Bar() {
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
pub fn to_title_case<'a>(non_title_case_string: String) -> String {
    return to_title_from_snake(to_snake_case(non_title_case_string));
}

    fn to_title_from_snake<'a>(non_sentence_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_sentence_case_string.chars() {
            if character.to_string() != "_" && character.to_string() != "-" && !first_character {
                result = format!("{}{}", result, character.to_ascii_lowercase());
                first_character = false
            } else if character.to_string() == "_" || character.to_string() == "-" {
                first_character = true;
            } else {
                result = format!("{} {}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        return result.trim().to_string();
    }

/// Determines if a `String` is Title Case
///
/// #Examples
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_title_case_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_title_case_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_title_case_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_title_case_when_snake() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_title_case_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_title_case_when_lower() {
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::cases::titlecase::to_title_case;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_truthy_value_for_is_title_case_when_title() {
///     let mock_string: String = "Foo Bar String That Is Really Really Long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == true);
/// }
/// ```
pub fn is_title_case<'a>(test_string: String) -> bool{
    let title_matcher= Regex::new(r"(^[A-Z][a-z0-9]+)([^-|^_]*[ ][A-Z][a-z0-9]+)").unwrap();
    let mut is_title_case= false;
    if title_matcher.is_match(test_string.as_ref()) {
        is_title_case = true;
    }
    return is_title_case;
}
