#![deny(warnings)]
#[cfg(feature = "heavyweight")]
use cases::case::*;
#[cfg(feature = "heavyweight")]
use string::singularize::to_singular;
#[cfg(feature = "heavyweight")]
/// Converts a `&str` to `ClassCase` `String`
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "FooBars";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "Foo Bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "foo-bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "foo_bars";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_class_case(non_class_case_string: &str) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    let class_plural: String = to_case_camel_like(non_class_case_string, options);
    let split: (&str, &str) =
        class_plural.split_at(class_plural.rfind(char::is_uppercase).unwrap_or(0));
    format!("{}{}", split.0, to_singular(split.1))
}

#[cfg(feature = "heavyweight")]
/// Determines if a `&str` is `ClassCase` `bool`
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "Foo";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "foo";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongStrings";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "foo_bar_is_a_really_really_long_strings";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "fooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: &str = "Foo Bar Is A Really Really Long String";
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_class_case(test_string: &str) -> bool {
    test_string == to_class_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
#[cfg(feature = "heavyweight")]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_class_case(b: &mut Bencher) {
        b.iter(|| super::to_class_case("Foo bar"));
    }

    #[bench]
    fn bench_is_class(b: &mut Bencher) {
        b.iter(|| super::is_class_case("Foo bar"));
    }

    #[bench]
    fn bench_class_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_class_case("foo_bar"));
    }
}

#[cfg(feature = "heavyweight")]
define_test_group!(class_tests, to_class_case, classcase, "FooBar", "FooBar");
