use string::deconstantize::*;

#[test] #[allow(non_snake_case)]
fn deconstantize_Mapper_as_Mapper() {
    let mock_string: String = "Mapper".to_string();
    let expected_string: String = "".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn deconstantize_namespace_Mapper_as_Mapper() {
    let mock_string: String = "::Mapper".to_string();
    let expected_string: String = "".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn deconstantize_Data_namespace_Mapper_as_Mapper() {
    let mock_string: String = "Data::Mapper".to_string();
    let expected_string: String = "Data".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn deconstantize_Test_namespace_Data_namespace_Mapper_as_Mapper() {
    let mock_string: String = "Test::Data::Mapper".to_string();
    let expected_string: String = "Data".to_string();
    let asserted_string: String = deconstantize(mock_string);
    assert!(asserted_string == expected_string);
}

