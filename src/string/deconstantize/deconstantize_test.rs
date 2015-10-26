use string::deconstantize::*;

#[test] #[allow(non_snake_case)]
fn deconstantize_Bar_as_Bar() {
    let mock_string: String = "Bar".to_string();
    let expected_string: String = "".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn deconstantize_namespace_Bar_as_Bar() {
    let mock_string: String = "::Bar".to_string();
    let expected_string: String = "".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn deconstantize_Foo_namespace_Bar_as_Bar() {
    let mock_string: String = "Foo::Bar".to_string();
    let expected_string: String = "Foo".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn deconstantize_Test_namespace_Foo_namespace_Bar_as_Bar() {
    let mock_string: String = "Test::Foo::Bar".to_string();
    let expected_string: String = "Foo".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

