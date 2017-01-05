#![deny(warnings)]
use cases::case::*;
/// Determines if a `String` is `kebab-case`
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::is_kebab_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_kebab_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_kebab_case(test_string: String) -> bool {
    test_string == to_kebab_case(test_string.clone())
}

/// Converts a `String` to `kebab-case` `String`
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::kebabcase::to_kebab_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "foo-bar".to_string();
///     let asserted_string: String = to_kebab_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_kebab_case(non_kebab_case_string: String) -> String {
    to_case_snake_like(non_kebab_case_string, "-", "lower")
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_kebab(b: &mut Bencher) {
        b.iter(|| super::to_kebab_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_is_kebab(b: &mut Bencher) {
        b.iter(|| super::is_kebab_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_kebab_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_kebab_case("test_test_test".to_string()));
    }
}

define_test_group!(kebab_tests, to_kebab_case, kebabcase, "foo-bar", "foo-bars");
