#![deny(warnings)]
use std::ascii::*;
/// Converts a `String` to pascalCase `String`
///
/// #Examples
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: String = "FooBar3".to_string();
///     let expected_string: String = "FooBar3".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_pascal_case(non_pascalized_string: String) -> String {
    let mut new_word: bool = true;
    let mut last_char: char = ' ';
    non_pascalized_string
        .chars()
        .fold("".to_string(), |mut result, character|
            if character == '-' || character == '_' || character == ' ' {
                new_word = true;
                result
            } else if character.is_numeric() {
                new_word = true;
                result.push(character);
                result
            } else if new_word || (
                (last_char.is_lowercase() && character.is_uppercase()) &&
                (last_char != ' ')
                ){
                new_word = false;
                result.push(character.to_ascii_uppercase());
                result
            } else {
                last_char = character;
                result.push(character.to_ascii_lowercase());
                result
            }
        )
}

/// Determines if a `String` is pascalCase bool``
///
/// #Examples
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "FooBarIsAReallyReally3LongString".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_pascal_case(test_string: String) -> bool {
    to_pascal_case(test_string.clone()) == test_string
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_pascal0(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "Foo bar".to_string();
            super::to_pascal_case(test_string)
        });
    }

    #[bench]
    fn bench_pascal1(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "foo_bar".to_string();
            super::to_pascal_case(test_string)
        }
        );
    }

    #[bench]
    fn bench_pascal2(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "fooBar".to_string();
            super::to_pascal_case(test_string)
        });
    }

    #[bench]
    fn bench_is_pascal(b: &mut Bencher) {
        b.iter(|| {
            let test_string: String  = "Foo bar".to_string();
            super::is_pascal_case(test_string)
        }
        );
    }
}
