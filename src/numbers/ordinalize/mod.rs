/// Orginalizes a `String`
///
/// #Examples
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "a".to_string();
///     let expected_string: String = "a".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "0.1".to_string();
///     let expected_string: String = "0.1".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "-1".to_string();
///     let expected_string: String = "-1st".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "0".to_string();
///     let expected_string: String = "0th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "1".to_string();
///     let expected_string: String = "1st".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "2".to_string();
///     let expected_string: String = "2nd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "3".to_string();
///     let expected_string: String = "3rd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "9".to_string();
///     let expected_string: String = "9th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "12".to_string();
///     let expected_string: String = "12th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "12000".to_string();
///     let expected_string: String = "12000th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "12001".to_string();
///     let expected_string: String = "12001st".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "12002".to_string();
///     let expected_string: String = "12002nd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "12003".to_string();
///     let expected_string: String = "12003rd".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::numbers::ordinalize::ordinalize;
///     let mock_string: String = "12004".to_string();
///     let expected_string: String = "12004th".to_string();
///     let asserted_string: String = ordinalize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn ordinalize(non_ordinalized_string: String) -> String {
    let chars: Vec<char> = non_ordinalized_string.clone().chars().collect();
    let last_number: char = chars[chars.len() - 1];
    if is_ordinalizable(last_number) {
        return non_ordinalized_string;
    }
    if chars.len() > 1 {
        if second_last_number_is_one(chars) {
            return format!("{}{}", non_ordinalized_string, "th");
        } else if string_contains_decimal(non_ordinalized_string.clone()) {
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

fn is_ordinalizable(last_number: char) -> bool {
    !last_number.is_numeric()
}

fn second_last_number_is_one(chars: Vec<char>) -> bool {
    let second_last_number: char = chars[chars.len() - 2];
    second_last_number == '1'
}

fn string_contains_decimal(non_ordinalized_string: String) -> bool {
    non_ordinalized_string.contains('.')
}
