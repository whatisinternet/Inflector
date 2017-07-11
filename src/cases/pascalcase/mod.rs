#![deny(warnings)]
use cases::case::*;
/// Converts a `&str` to pascalCase `String`
///
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "Foo Bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "foo-bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::pascalcase::to_pascal_case;
///     let mock_string: &str = "FooBar3";
///     let expected_string: String = "FooBar3".to_string();
///     let asserted_string: String = to_pascal_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_pascal_case(non_pascalized_string: &str) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    to_case_camel_like(non_pascalized_string, options)
}

/// Determines if a `&str` is pascalCase bool``
///
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "Foo";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "foo";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "FooBarIsAReallyReally3LongString";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::pascalcase::is_pascal_case;
///     let mock_string: &str = "Foo Bar Is A Really Really Long String";
///     let asserted_bool: bool = is_pascal_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_pascal_case(test_string: &str) -> bool {
    to_pascal_case(test_string.clone()) == test_string
}

#[cfg(all(feature = "unstable", test))]
mod benchmarks {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_pascal0(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "Foo bar";
            super::to_pascal_case(test_string)
        });
    }

    #[bench]
    fn bench_pascal1(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "foo_bar";
            super::to_pascal_case(test_string)
        });
    }

    #[bench]
    fn bench_pascal2(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "fooBar";
            super::to_pascal_case(test_string)
        });
    }

    #[bench]
    fn bench_is_pascal(b: &mut Bencher) {
        b.iter(|| {
            let test_string: &str = "Foo bar";
            super::is_pascal_case(test_string)
        });
    }
}

#[cfg(all(test))]
mod tests {
    define_test_group!(pascal_tests,
                    to_pascal_case,
                    is_pascal_case,
                    pascalcase,
                    "FooBar",
                    "FooBars");
}

