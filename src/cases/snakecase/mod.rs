#![deny(warnings)]
use cases::case::*;
/// Converts a `&str` to `snake_case` `String`
///
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "HTTP Foo bar";
///     let expected_string: String = "http_foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "HTTPFooBar";
///     let expected_string: String = "http_foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "Foo Bar";
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: &str = "fooBar3";
///     let expected_string: String = "foo_bar_3".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_snake_case(non_snake_case_string: &str) -> String {
    to_case_snake_like(non_snake_case_string, "_", "lower")
}

/// Determines of a `&str` is `snake_case`
///
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "Foo Bar Is A Really Really Long String";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "FOO_BAR_IS_A_REALLY_REALLY_LONG_STRING";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "fooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "foo_bar1_string_that_is_really_really_long";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: &str = "foo_bar_1_string_that_is_really_really_long";
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_snake_case(test_string: &str) -> bool {
    test_string == to_snake_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_snake_from_title(b: &mut Bencher) {
        b.iter(|| super::to_snake_case("Foo bar"));
    }

    #[bench]
    fn bench_snake_from_camel(b: &mut Bencher) {
        b.iter(|| super::to_snake_case("fooBar"));
    }

    #[bench]
    fn bench_snake_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_snake_case("foo_bar_bar_bar"));
    }

    #[bench]
    fn bench_is_snake(b: &mut Bencher) {
        b.iter(|| super::is_snake_case("Foo bar"));
    }
}

define_test_group!(snake_tests, to_snake_case, snakecase, "foo_bar", "foo_bars");
