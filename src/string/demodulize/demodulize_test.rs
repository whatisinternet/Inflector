use string::demodulize::*;

#[test] #[allow(non_snake_case)]
fn demodulize_Mapper_as_Mapper() {
    let mock_string: String = "Mapper".to_string();
    let expected_string: String = "Mapper".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn demodulize_namespace_Mapper_as_Mapper() {
    let mock_string: String = "::Mapper".to_string();
    let expected_string: String = "Mapper".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn demodulize_Data_namespace_Mapper_as_Mapper() {
    let mock_string: String = "Data::Mapper".to_string();
    let expected_string: String = "Mapper".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn demodulize_Test_namespace_Data_namespace_Mapper_as_Mapper() {
    let mock_string: String = "Test::Data::Mapper".to_string();
    let expected_string: String = "Mapper".to_string();
    let asserted_string: String = demodulize(mock_string);
    assert!(asserted_string == expected_string);
}

