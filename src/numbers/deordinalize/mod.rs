/// Deorginalizes a `String`
///
/// #Examples
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "0.1".to_string();
///     let expected_string: String = "0.1".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "-1st".to_string();
///     let expected_string: String = "-1".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "0th".to_string();
///     let expected_string: String = "0".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "1st".to_string();
///     let expected_string: String = "1".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "2nd".to_string();
///     let expected_string: String = "2".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "3rd".to_string();
///     let expected_string: String = "3".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "9th".to_string();
///     let expected_string: String = "9".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "12th".to_string();
///     let expected_string: String = "12".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "12000th".to_string();
///     let expected_string: String = "12000".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "12001th".to_string();
///     let expected_string: String = "12001".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "12002nd".to_string();
///     let expected_string: String = "12002".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "12003rd".to_string();
///     let expected_string: String = "12003".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::deordinalize::deordinalize;
///     let mock_string: String = "12004th".to_string();
///     let expected_string: String = "12004".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn deordinalize(non_ordinalized_string: String) -> String {
    if non_ordinalized_string.contains('.') {
        non_ordinalized_string
    } else {
        non_ordinalized_string
            .trim_right_matches("st")
            .trim_right_matches("nd")
            .trim_right_matches("rd")
            .trim_right_matches("th")
            .to_string()
    }
}
