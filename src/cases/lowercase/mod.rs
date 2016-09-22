/// Converts a `String` to lowercase `String`
///
/// #Examples
/// ```
///     use inflector::cases::lowercase::to_lower_case;
///     let mock_string: String = "FoObAR".to_string();
///     let expected_string: String = "foobar".to_string();
///     let asserted_string: String = to_lower_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_lower_case(non_lower_string: String) -> String {
    // See https://github.com/calebmer/inflections/blob/master/src/case.rs#L37 for where this
    // implementation comes from.
    non_lower_string.chars()
        .flat_map(char::to_lowercase)
        .collect()
}

/// Determines if a `String` is lowercase
///
/// #Examples
/// ```
///     use inflector::cases::lowercase::is_lower_case;
///     let mock_string: String = "foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_lower_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::lowercase::is_lower_case;
///     let mock_string: String = "Foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_lower_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::lowercase::is_lower_case;
///     let mock_string: String = "FOOBARSTRINGTHATISREALLYREALLYLONG".to_string();
///     let asserted_bool: bool = is_lower_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_lower_case(test_string: String) -> bool {
    test_string == to_lower_case(test_string.clone())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_lower(b: &mut Bencher) {
        b.iter(|| super::to_lower_case("Foo BAR".to_string()));
    }

    #[bench]
    fn bench_is_lower(b: &mut Bencher) {
        b.iter(|| super::is_lower_case("Foo bar".to_string()));
    }
}
