use cases::snakecase::*;

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_sentence() {
    let mock_string: String = "Foo bar string that is really really long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_kebab() {
    let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_class() {
    let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_title() {
    let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_camel() {
    let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_snake_case_when_snake() {
    let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_snake_case_when_snake_with_a_number() {
    let mock_string: String = "foo_bar1_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_snake_case_when_snake_with_a_number_snaked() {
    let mock_string: String = "foo_bar_1_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn snake_case_foo_bar_as_foo_bar() {
    let mock_string: String = "foo_bar".to_string();
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn snake_case_HTTP_Foo_space_bar_as_foo_bar() {
    let mock_string: String = "HTTP Foo bar".to_string();
    let expected_string: String = "http_foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn snake_case_Foo_space_bar_as_foo_bar() {
    let mock_string: String = "Foo bar".to_string();
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn snake_case_Foo_space_Bar_as_foo_bar() {
    let mock_string: String = "Foo Bar".to_string();
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn snake_case_FooBar_as_foo_bar() {
    let mock_string: String = "FooBar".to_string();
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn snake_case_fooBar_as_foo_bar() {
    let mock_string: String = "fooBar".to_string();
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn snake_case_fooBar3_as_foo_bar() {
    let mock_string: String = "fooBar3".to_string();
    let expected_string: String = "foo_bar_3".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}
