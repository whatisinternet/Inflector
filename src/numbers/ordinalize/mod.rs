/// Orginalizes a `String`
///
/// #Examples
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_return_the_orgional_string_if_not_a_number_ordinalize(){
///     let mock_string: String = "a".to_string();
///     let expected_string: String = "a".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_0_point_1_as_0_point_1(){
///     let mock_string: String = "0.1".to_string();
///     let expected_string: String = "0.1".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_negative_1_as_negative_1st(){
///     let mock_string: String = "-1".to_string();
///     let expected_string: String = "-1st".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_0_as_0th(){
///     let mock_string: String = "0".to_string();
///     let expected_string: String = "0th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_1_as_1st(){
///     let mock_string: String = "1".to_string();
///     let expected_string: String = "1st".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_2_as_2nd(){
///     let mock_string: String = "2".to_string();
///     let expected_string: String = "2nd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_3_as_3rd(){
///     let mock_string: String = "3".to_string();
///     let expected_string: String = "3rd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_9_as_9th(){
///     let mock_string: String = "9".to_string();
///     let expected_string: String = "9th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_12_as_12th(){
///     let mock_string: String = "12".to_string();
///     let expected_string: String = "12th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_12000_as_12000th(){
///     let mock_string: String = "12000".to_string();
///     let expected_string: String = "12000th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_12001_as_12001th(){
///     let mock_string: String = "12001".to_string();
///     let expected_string: String = "12001st".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_12002_as_12002nd(){
///     let mock_string: String = "12002".to_string();
///     let expected_string: String = "12002nd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_12003_as_12003rd(){
///     let mock_string: String = "12003".to_string();
///     let expected_string: String = "12003rd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::numbers::ordinalize::ordinalize;
///
///
/// // should_ordinalize_12004_as_12004th(){
///     let mock_string: String = "12004".to_string();
///     let expected_string: String = "12004th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn ordinalize<'a>(non_ordinalized_string: String) -> String {
    let chars: Vec<char>= non_ordinalized_string.clone().chars().collect();
    let last_number: char= chars[chars.len() - 1];
    if !last_number.is_numeric() {
        return non_ordinalized_string;
    }
    if chars.len() > 1 {
        let second_last_number: char= chars[chars.len() - 2];
        if second_last_number == '1'{
            return format!("{}{}", non_ordinalized_string, "th");
        } else if non_ordinalized_string.contains(".") {
            return non_ordinalized_string;
        }
    }
    match last_number {
        '1' => format!("{}{}", non_ordinalized_string, "st"),
        '2' => format!("{}{}", non_ordinalized_string, "nd"),
        '3' => format!("{}{}", non_ordinalized_string, "rd"),
        _ => format!("{}{}", non_ordinalized_string, "th"),
    }
}
