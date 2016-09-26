/// Converts a `String` to uppercase `String`
///
/// #Examples
/// ```
///     use inflector::cases::uppercase::to_upper_case;
///     let mock_string: String = "FoObAR".to_string();
///     let expected_string: String = "FOOBAR".to_string();
///     let asserted_string: String = to_upper_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
#[deprecated(since="0.3.2", note="Please use the standard `.to_uppercase()`")]
pub fn to_upper_case(non_upper_string: String) -> String {
    non_upper_string.chars()
        .flat_map(char::to_uppercase)
        .collect()
}

/// Determines if a `String` is UPPERCASE
///
/// #Examples
/// ```
///     use inflector::cases::uppercase::is_upper_case;
///     let mock_string: String = "FOOBARSTRINGTHATISREALLYREALLYLONG".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::uppercase::is_upper_case;
///     let mock_string: String = "foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::uppercase::is_upper_case;
///     let mock_string: String = "Foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_upper_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
#[deprecated(since="0.3.2", note="Please use the standard `.is_uppercase()`")]
pub fn is_upper_case(test_string: String) -> bool {
    test_string == to_upper_case(test_string.to_owned())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_upper(b: &mut Bencher) {
        b.iter(|| super::to_upper_case("Foo BAR".to_string()));
    }

    #[bench]
    fn bench_is_upper(b: &mut Bencher) {
        b.iter(|| super::is_upper_case("Foo bar".to_string()));
    }
}
