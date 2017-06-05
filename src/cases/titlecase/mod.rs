#![deny(warnings)]
use cases::case::*;
/// Converts a `&str` to `Title Case` `String`
///
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::titlecase::to_title_case;
///     let mock_string: &str = "foo-bar";
///     let expected_string: String = "Foo Bar".to_string();
///     let asserted_string: String = to_title_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_title_case(non_title_case_string: &str) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: true,
        injectable_char: ' ',
        has_seperator: true,
        inverted: false,
    };
    to_case_camel_like(non_title_case_string, options)
}

/// Determines if a `&str` is `Title Case`
///
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "fooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "foo";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::titlecase::is_title_case;
///     let mock_string: &str = "Foo Bar String That Is Really Really Long";
///     let asserted_bool: bool = is_title_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_title_case(test_string: &str) -> bool {
    test_string == to_title_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_title(b: &mut Bencher) {
        b.iter(|| super::to_title_case("Foo BAR"));
    }

    #[bench]
    fn bench_is_title(b: &mut Bencher) {
        b.iter(|| super::is_title_case("Foo bar"));
    }

    #[bench]
    fn bench_title_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_title_case("foo_bar"));
    }

}

#[cfg(test)]
mod tests {
    define_test_group!(title_tests, to_title_case, is_title_case, titlecase, "Foo Bar", "Foo Bars");
}

