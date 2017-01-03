#![deny(warnings)]
use cases::case::*;
/// Converts a `String` to `snake_case` `String`
///
/// #Examples
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "HTTP Foo bar".to_string();
///     let expected_string: String = "http_foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "HTTPFooBar".to_string();
///     let expected_string: String = "http_foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::snakecase::to_snake_case;
///     let mock_string: String = "fooBar3".to_string();
///     let expected_string: String = "foo_bar_3".to_string();
///     let asserted_string: String = to_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_snake_case(non_snake_case_string: String) -> String {
    to_case_snake_like(non_snake_case_string, "_", "lower")
}

/// Determines of a `String` is `snake_case`
///
/// #Examples
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "FOO_BAR_IS_A_REALLY_REALLY_LONG_STRING".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "foo_bar1_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::snakecase::is_snake_case;
///     let mock_string: String = "foo_bar_1_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_snake_case(test_string: String) -> bool {
    test_string == to_snake_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_snake_from_title(b: &mut Bencher) {
        b.iter(|| super::to_snake_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_snake_from_camel(b: &mut Bencher) {
        b.iter(|| super::to_snake_case("fooBar".to_string()));
    }

    #[bench]
    fn bench_snake_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_snake_case("foo_bar_bar_bar".to_string()));
    }

    #[bench]
    fn bench_is_snake(b: &mut Bencher) {
        b.iter(|| super::is_snake_case("Foo bar".to_string()));
    }
}

#[cfg(test)]
mod snake_tests{
    use ::cases::snakecase::to_snake_case;
    define_tests![
        to_snake_case;
        test_camel_case_to_snake_case=>             "fooBar"    => "foo_bar",
        test_class_case_to_snake_case =>            "FooBar"    => "foo_bar",
        test_screaming_snake_case_to_snake_case =>  "FOO_BAR"   => "foo_bar",
        test_kebab_case_to_snake_case =>            "foo-bar"   => "foo_bar",
        test_pascal_case_to_snake_case =>           "FooBar"    => "foo_bar",
        test_sentence_case_to_snake_case =>         "Foo bar"   => "foo_bar",
        test_snake_case_to_snake_case =>            "foo_bar"   => "foo_bar",
        test_title_case_to_snake_case =>            "Foo Bar"   => "foo_bar",
        test_table_case_to_snake_case =>            "foo_bars"  => "foo_bars",
        test_train_case_to_snake_case =>            "Foo-Bars"  => "foo_bars"
    ];
}
