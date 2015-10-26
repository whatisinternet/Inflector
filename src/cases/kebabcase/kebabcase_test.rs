use cases::kebabcase::*;

#[test] #[allow(non_snake_case)]
fn kebab_case_foo_dash_bar_as_foo_dash_bar() {
    let mock_string: String = "foo-bar".to_string();
    let expected_string: String = "foo-bar".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_foo_bar_as_foo_bar() {
    let mock_string: String = "foo_bar".to_string();
    let expected_string: String = "foo-bar".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_Foo_space_Bar_as_foo_bar() {
    let mock_string: String = "Foo Bar".to_string();
    let expected_string: String = "foo-bar".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_Foo_space_bar_as_foo_bar() {
    let mock_string: String = "Foo bar".to_string();
    let expected_string: String = "foo-bar".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_FooBar_as_foo_bar() {
    let mock_string: String = "FooBar".to_string();
    let expected_string: String = "foo-bar".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_fooBar_as_foo_bar() {
    let mock_string: String = "fooBar".to_string();
    let expected_string: String = "foo-bar".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_kebab_case_when_kebab() {
    let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_class() {
    let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_camel() {
    let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_snake() {
    let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_sentence() {
    let mock_string: String = "Foo bar string that is really really long".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_title() {
    let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == false);
}
