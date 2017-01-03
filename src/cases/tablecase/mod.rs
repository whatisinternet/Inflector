#![deny(warnings)]
use string::pluralize::to_plural;
use cases::case::*;
/// Converts a `String` to `table-case` `String`
///
/// #Examples
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "foo-bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "FOO_BAR".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "foo_bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "Foo Bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "Foo bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "FooBar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "fooBar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
pub fn to_table_case(non_table_case_string: String) -> String {
    let snaked: String = to_case_snake_like(non_table_case_string, "_", "lower");
    let split: (&str, &str) = snaked.split_at(snaked.rfind('_').unwrap_or(0));
    format!("{}{}", split.0, to_plural(split.1.to_string()))
}
/// Determines if a `String` is `table-case`
///
/// #Examples
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "foo_bar_strings".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == true);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_table_case(test_string: String) -> bool {
    test_string.clone() == to_table_case(test_string)
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_table_case(b: &mut Bencher) {
        b.iter(|| super::to_table_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_is_table_case(b: &mut Bencher) {
        b.iter(|| super::is_table_case("Foo bar".to_string()));
    }
}

#[cfg(test)]
mod table_tests{
    use ::cases::tablecase::to_table_case;
    define_tests![
        to_table_case;
        test_camel_case_to_table_case=>             "fooBar"    => "foo_bars",
        test_class_case_to_table_case =>            "FooBar"    => "foo_bars",
        test_screaming_snake_case_to_table_case =>  "FOO_BAR"   => "foo_bars",
        test_kebab_case_to_table_case =>            "foo-bar"   => "foo_bars",
        test_pascal_case_to_table_case =>           "FooBar"    => "foo_bars",
        test_sentence_case_to_table_case =>         "Foo bar"   => "foo_bars",
        test_snake_case_to_table_case =>            "foo_bar"   => "foo_bars",
        test_title_case_to_table_case =>            "Foo Bar"   => "foo_bars",
        test_table_case_to_table_case =>            "foo_bars"  => "foo_bars",
        test_train_case_to_table_case =>            "Foo-Bars"  => "foo_bars"
    ];
}
