use cases::lowercase::*;

#[test] #[allow(non_snake_case)]
fn upcase_FoObAR_as_foobar() {
    let mock_string: String = "FoObAR".to_string();
    let expected_string: String = "foobar".to_string();
    let asserted_string: String = to_lower_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_lower_case_when_lowercase() {
    let mock_string: String = "foobarisareallyreallylongstring".to_string();
    let asserted_bool: bool = is_lower_case(mock_string);
    assert!(asserted_bool == true);
}
#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_lower_case_when_Startcased() {
    let mock_string: String = "Foobarisareallyreallylongstring".to_string();
    let asserted_bool: bool = is_lower_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_lower_case_when_uppercase() {
    let mock_string: String = "FOOBARSTRINGTHATISREALLYREALLYLONG".to_string();
    let asserted_bool: bool = is_lower_case(mock_string);
    assert!(asserted_bool == false);
}
