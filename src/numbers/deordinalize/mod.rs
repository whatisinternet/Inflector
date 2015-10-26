/// Deorginalizes a `String`
///
/// #Examples
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_0_point_1_as_0_point_1(){
///     let mock_string: String = "0.1".to_string();
///     let expected_string: String = "0.1".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_negative_1st_as_negative_1(){
///     let mock_string: String = "-1st".to_string();
///     let expected_string: String = "-1".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_0th_as_0(){
///     let mock_string: String = "0th".to_string();
///     let expected_string: String = "0".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_1st_as_1(){
///     let mock_string: String = "1st".to_string();
///     let expected_string: String = "1".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_2nd_as_2(){
///     let mock_string: String = "2nd".to_string();
///     let expected_string: String = "2".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_3rd_as_3(){
///     let mock_string: String = "3rd".to_string();
///     let expected_string: String = "3".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_9th_as_9th(){
///     let mock_string: String = "9th".to_string();
///     let expected_string: String = "9".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_12th_as_12(){
///     let mock_string: String = "12th".to_string();
///     let expected_string: String = "12".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_12000th_as_12000(){
///     let mock_string: String = "12000th".to_string();
///     let expected_string: String = "12000".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_12001th_as_12001(){
///     let mock_string: String = "12001th".to_string();
///     let expected_string: String = "12001".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_12002nd_as_12002(){
///     let mock_string: String = "12002nd".to_string();
///     let expected_string: String = "12002".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_12003rd_as_12003(){
///     let mock_string: String = "12003rd".to_string();
///     let expected_string: String = "12003".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::numbers::deordinalize::deordinalize;
///
/// #[test]
/// fn should_deordinalize_12004th_as_12004(){
///     let mock_string: String = "12004th".to_string();
///     let expected_string: String = "12004".to_string();
///     let asserted_string: String = deordinalize(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
pub fn deordinalize<'a>(non_ordinalized_string: String) -> String {
    if non_ordinalized_string.contains(".")  {
        return non_ordinalized_string;
    } else {
        return non_ordinalized_string
            .trim_right_matches("st")
            .trim_right_matches("nd")
            .trim_right_matches("rd")
            .trim_right_matches("th")
            .to_string();
    }
}

#[cfg(test)]
mod deordinalize_test;
