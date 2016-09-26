use std::ascii::*;
/// Converts a `String` to camelCase `String`
///
/// #Examples
/// ```0
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```1
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```2
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```3
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```4
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```5
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```6
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```7
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "FooBar3".to_string();
///     let expected_string: String = "fooBar3".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_camel_case(non_camelized_string: String) -> String {
    let mut new_word: bool = false;
    let mut last_char: char = ' ';
    non_camelized_string
        .chars()
        .fold("".to_string(), |mut result, character|
            if character == '-' || character == '_' || character == ' ' {
                new_word = true;
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

/// Determines if a `String` is camelCase bool``
///
/// #Examples
/// ```0
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```1
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```2
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```3
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```4
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "fooBarIsAReallyReally3LongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```5
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```6
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```7
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```8
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```9
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_camel_case(test_string: String) -> bool {
    to_camel_case(test_string.clone()) == test_string
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_camel0(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "Foo bar".to_string();
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_camel1(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "foo_bar".to_string();
            super::to_camel_case(test_string)
        }
        );
    }

    #[bench]
    fn bench_camel2(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "fooBar".to_string();
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_is_camel(b: &mut Bencher) {
        b.iter(|| {
            let test_string: String  = "Foo bar".to_string();
            super::is_camel_case(test_string)
        }
        );
    }
}
