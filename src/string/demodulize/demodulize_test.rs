use string::demodulize::*;

#[test] #[allow(non_snake_case)]
fn demodulize_Bar_as_Bar() {
    let mock_string: String = "Bar".to_string();
    let expected_string: String = "Bar".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn demodulize_namespace_Bar_as_Bar() {
    let mock_string: String = "::Bar".to_string();
    let expected_string: String = "Bar".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn demodulize_Foo_namespace_Bar_as_Bar() {
    let mock_string: String = "Foo::Bar".to_string();
    let expected_string: String = "Bar".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn demodulize_Test_namespace_Foo_namespace_Bar_as_Bar() {
    let mock_string: String = "Test::Foo::Bar".to_string();
    let expected_string: String = "Bar".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

