#![deny(warnings)]
use cases::case::*;
/// Determines if a `String` is `Train-Case`
///
/// #Examples
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "Foo-Bar-String-That-Is-Really-Really-Long".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_train_case(test_string: String) -> bool {
    test_string == to_train_case(test_string.clone())
}


/// Converts a `String` to `Train-Case` `String`
///
/// #Examples
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_train_case(non_train_case_string: String) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: true,
        injectable_char: '-',
        has_seperator: true
    };
    to_case_camel_like(non_train_case_string, options)
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_train(b: &mut Bencher) {
        b.iter(|| super::to_train_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_is_train(b: &mut Bencher) {
        b.iter(|| super::is_train_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_train_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_train_case("test_test_test".to_string()));
    }
}
