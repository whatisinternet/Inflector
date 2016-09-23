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
    if non_snake_case_string.contains(' ') ||
        non_snake_case_string.contains('_') ||
            non_snake_case_string.contains('-') {
        let seperators: &[char] = &[' ', '-', '_'];
        let tokens: Vec<&str> = non_snake_case_string.split(seperators).collect();
        tokens.join("_").to_lowercase()
    } else {
        let converted: Vec<String> = non_snake_case_string
            .chars()
            .enumerate()
            .map(|(i, s)|
                 if (s.is_uppercase() == true || s.is_numeric()) && (i > 0 as usize) {
                     format!("_{}", s)
                 } else {
                     s.to_string()
                 }
                 )
            .collect();

        converted
            .join("")
            .to_lowercase()
    }
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

#[cfg(test)]
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
        b.iter(|| super::to_snake_case("foo_bar".to_string()));
    }

    #[bench]
    fn bench_is_snake(b: &mut Bencher) {
        b.iter(|| super::is_snake_case("Foo bar".to_string()));
    }
}
