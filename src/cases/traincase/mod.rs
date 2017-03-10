#![deny(warnings)]
use cases::case::*;
/// Determines if a `&str` is `Train-Case`
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "Foo-Bar-String-That-Is-Really-Really-Long";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "fooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::is_train_case;
///     let mock_string: &str = "Foo Bar Is A Really Really Long String";
///     let asserted_bool: bool = is_train_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_train_case(test_string: &str) -> bool {
    test_string == to_train_case(test_string.clone())
}


/// Converts a `&str` to `Train-Case` `String`
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "foo-bar";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "Foo Bar";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::traincase::to_train_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "Foo-Bar".to_string();
///     let asserted_string: String = to_train_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_train_case(non_train_case_string: &str) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: true,
        injectable_char: '-',
        has_seperator: true,
        inverted: false,
    };
    to_case_camel_like(non_train_case_string, options)
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_train(b: &mut Bencher) {
        b.iter(|| super::to_train_case("Foo bar"));
    }

    #[bench]
    fn bench_is_train(b: &mut Bencher) {
        b.iter(|| super::is_train_case("Foo bar"));
    }

    #[bench]
    fn bench_train_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_train_case("test_test_test"));
    }
}
define_test_group!(train_tests, to_train_case, traincase, "Foo-Bar", "Foo-Bars");
