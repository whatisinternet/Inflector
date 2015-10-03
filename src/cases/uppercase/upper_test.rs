use cases::uppercase::*;

#[test] #[allow(non_snake_case)]
fn upcase_DatAMaPper_as_DATAMAPPER() {
    let mock_string: String = "DatAMaPper".to_string();
    let expected_string: String = "DATAMAPPER".to_string();
    let asserted_string: String = to_upper_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_upper_case_when_uppercase() {
    let mock_string: String = "DATAMAPPERISAREALLYREALLYLONGSTRING".to_string();
    let asserted_bool: bool = is_upper_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_upper_case_when_lowercase() {
    let mock_string: String = "data mapper string that is really really long".to_string();
    let asserted_bool: bool = is_upper_case(mock_string);
    assert!(asserted_bool == false);
}
