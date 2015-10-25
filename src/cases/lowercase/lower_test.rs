use cases::lowercase::*;

#[test] #[allow(non_snake_case)]
fn upcase_DatAMaPper_as_DATAMAPPER() {
    let mock_string: String = "DatAMaPper".to_string();
    let expected_string: String = "datamapper".to_string();
    let asserted_string: String = to_lower_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_lower_case_when_lowercase() {
    let mock_string: String = "datamapperisareallyreallylongstring".to_string();
    let asserted_bool: bool = is_lower_case(mock_string);
    assert!(asserted_bool == true);
}
#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_lower_case_when_Startcased() {
    let mock_string: String = "Datamapperisareallyreallylongstring".to_string();
    let asserted_bool: bool = is_lower_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_lower_case_when_uppercase() {
    let mock_string: String = "DATAMAPPERSTRINGTHATISREALLYREALLYLONG".to_string();
    let asserted_bool: bool = is_lower_case(mock_string);
    assert!(asserted_bool == false);
}
