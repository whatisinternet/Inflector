#![deny(warnings)]
use std::ascii::*;
/// Converts a `String` to `Title Case` `String`
///
/// #Examples
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_title_case(non_title_case_string: String) -> String {
    let mut new_word: bool = true;
    let mut last_char: char = ' ';
    non_title_case_string
        .chars()
        .fold("".to_string(), |result, character|
            if character == '-' || character == '_' || character == ' ' {
                new_word = true;
                result
            } else if new_word || (
                (last_char.is_lowercase() && character.is_uppercase()) &&
                (last_char != ' ')
                ){
                new_word = false;
                format!("{} {}", result, character.to_ascii_uppercase())
                    .trim()
                    .to_string()
            } else {
                last_char = character;
                format!("{}{}", result, character.to_ascii_lowercase())
            }
        )
}
/// Determines if a `String` is `Title Case`
///
/// #Examples
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: String = "Foo Bar String That Is Really Really Long".to_string();
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_title_case(test_string: String) -> bool {
    test_string == to_title_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_title(b: &mut Bencher) {
        b.iter(|| super::to_title_case("Foo BAR".to_string()));
    }

    #[bench]
    fn bench_is_title(b: &mut Bencher) {
        b.iter(|| super::is_title_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_title_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_title_case("foo_bar".to_string()));
    }
}
