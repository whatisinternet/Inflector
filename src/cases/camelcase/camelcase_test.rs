use cases::camelcase::*;

#[test] #[allow(non_snake_case)]
fn camelize_fooBar_as_fooBar() {
    let mock_string: String = "fooBar".to_string();
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_Foo_Bar_as_fooBar() {
    let mock_string: String = "Foo Bar".to_string();
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_foo_bar_as_fooBar() {
    let mock_string: String = "foo_bar".to_string();
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_Foo_space_bar_as_fooBar() {
    let mock_string: String = "Foo bar".to_string();
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_foo_dash_bar_as_fooBar() {
    let mock_string: String = "foo-bar".to_string();
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_FooBar_as_fooBar() {
    let mock_string: String = "FooBar".to_string();
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_FooBar3_as_fooBar3() {
    let mock_string: String = "FooBar3".to_string();
    let expected_string: String = "fooBar3".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_start_cased() {
    let mock_string: String = "Foo".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_lower_case() {
    let mock_string: String = "foo".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_kebab() {
    let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_class() {
    let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_camel_case_when_camel_with_number() {
    let mock_string: String = "fooBarIsAReallyReally3LongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_camel_case_when_camel() {
    let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_snake() {
    let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_sentence() {
    let mock_string: String = "Foo bar string that is really really long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_title() {
    let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}
