use regex::Regex;

use cases::snakecase::to_snake_case;

/// Converts a `String` to a foreign_key
///
/// #Examples
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_foo_bar_as_foo_bar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_Foo_space_bar_as_foo_bar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_Foo_space_Bar_as_foo_bar() {
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_Foo_namespace_Bar_as_foo_bar() {
///     let mock_string: String = "Foo::Bar".to_string();
///     let expected_string: String = "bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_Test_namespace_Foo_namespace_Bar_as_foo_bar() {
///     let mock_string: String = "Test::Foo::Bar".to_string();
///     let expected_string: String = "bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_FooBar_as_foo_bar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_fooBar_as_foo_bar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "foo_bar_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::to_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn foreign_key_fooBar3_as_foo_bar() {
///     let mock_string: String = "fooBar3".to_string();
///     let expected_string: String = "foo_bar_3_id".to_string();
///     let asserted_string: String = to_foreign_key(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
pub fn to_foreign_key<'a>(non_foreign_key_string: String) -> String {
    if is_foreign_key(non_foreign_key_string.clone()){
        return non_foreign_key_string;
    } else {
        if non_foreign_key_string.contains("::") {
            let split_string: Vec<&str> = non_foreign_key_string.split("::").collect();
            return safe_convert(split_string[split_string.len() - 1].to_string());

        } else {
            return safe_convert(non_foreign_key_string);
        }
    }
}
    fn safe_convert<'a>(safe_string: String) -> String{
        let snake_cased: String = to_snake_case(safe_string);
        return format!("{}{}", snake_cased, "_id");
    }

/// Determines if a `String` is a foreign_key
///
/// #Examples
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_foreign_key_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_foreign_key_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_foreign_key_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_foreign_key_when_title() {
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_foreign_key_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_falsey_value_for_is_foreign_key_when_snake() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == false);
/// }
/// ```
/// ```
/// use inflector::suffix::foreignkey::is_foreign_key;
///
/// #[test] #[allow(non_snake_case)]
/// fn returns_truthy_value_for_is_foreign_key_when_foreign_key() {
///     let mock_string: String = "foo_bar_string_that_is_really_really_long_id".to_string();
///     let asserted_bool: bool = is_foreign_key(mock_string);
///     assert!(asserted_bool == true);
/// }
/// ```
pub fn is_foreign_key<'a>(test_string: String) -> bool{
    let foreign_key_matcher= Regex::new(r"(?:[^-|^ ]?=^|[_])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_foreign_key: bool = false;
    if foreign_key_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref())
        && test_string.ends_with("_id"){
            is_foreign_key= true;
        }
    return is_foreign_key;
}
