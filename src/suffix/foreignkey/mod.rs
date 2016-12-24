use cases::snakecase::to_snake_case;

/// Converts a `String` to a `foreign_key`
///
/// #Examples
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "Foo::Bar".to_string();
///     let expected_string: String = "bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "Test::Foo::Bar".to_string();
///     let expected_string: String = "bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::to_foreign_key;
///     let mock_string: String = "fooBar3".to_string();
///     let expected_string: String = "foo_bar_3_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_foreign_key(non_foreign_key_string: String) -> String {
    if non_foreign_key_string.contains("::") {
        let split_string: Vec<&str> = non_foreign_key_string.split("::").collect();
        safe_convert(split_string[split_string.len() - 1].to_string())
    } else {
        safe_convert(non_foreign_key_string)
    }
}
fn safe_convert(safe_string: String) -> String {
    let snake_cased: String = to_snake_case(safe_string);
    if snake_cased.ends_with("_id") {
        snake_cased
    } else {
        format!("{}{}", snake_cased, "_id")
    }
}

/// Determines if a `String` is a `foreign_key`
///
/// #Examples
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::suffix::foreignkey::is_foreign_key;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long_id".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_foreign_key(test_string: String) -> bool {
    to_foreign_key(test_string.clone()) == test_string
}
