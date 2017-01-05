#![deny(warnings)]
use cases::case::*;
/// Converts a `String` to `SCREAMING_SNAKE_CASE` `String`
///
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "HTTP Foo bar".to_string();
///     let expected_string: String = "HTTP_FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///     let mock_string: String = "fooBar3".to_string();
///     let expected_string: String = "FOO_BAR_3".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_screaming_snake_case(non_snake_case_string: String) -> String {
    to_case_snake_like(non_snake_case_string, "_", "upper")
}

/// Determines of a `String` is `SCREAMING_SNAKE_CASE`
///
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "FOO_BAR1_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///     let mock_string: String = "FOO_BAR_1_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_screaming_snake_case(test_string: String) -> bool {
    test_string == to_screaming_snake_case(test_string.clone())
}


#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_screaming_snake(b: &mut Bencher) {
        b.iter(|| super::to_screaming_snake_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_is_screaming_snake(b: &mut Bencher) {
        b.iter(|| super::is_screaming_snake_case("Foo bar".to_string()));
    }
}

define_test_group!(screaming_snake_tests, to_screaming_snake_case, screamingsnakecase, "FOO_BAR", "FOO_BARS");
