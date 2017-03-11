#![deny(warnings)]
use cases::case::*;

/// Converts a `&str` to camelCase `String`
///
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "Foo Bar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "foo-bar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "FooBar3";
///     let expected_string: String = "fooBar3".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: &str = "Foo-Bar";
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_camel_case(non_camelized_string: &str) -> String {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    to_case_camel_like(&non_camelized_string, options)
}

/// Determines if a `&str` is camelCase bool``
///
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "Foo";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "foo";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "fooBarIsAReallyReally3LongString";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "fooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: &str = "Foo Bar Is A Really Really Long String";
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_camel_case(test_string: &str) -> bool {
    to_camel_case(&test_string.clone()) == test_string
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_camel0(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "Foo bar";
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_camel1(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "foo_bar";
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_camel2(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "fooBar";
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_is_camel(b: &mut Bencher) {
        b.iter(|| {
            let test_string: &str = "Foo bar";
            super::is_camel_case(test_string)
        });
    }
}


define_test_group!(camel_tests, to_camel_case, is_camel_case, camelcase, "fooBar", "fooBars");
