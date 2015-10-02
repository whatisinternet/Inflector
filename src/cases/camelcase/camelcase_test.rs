use cases::camelcase::*;

#[test] #[allow(non_snake_case)]
fn camelize_dataMapper_as_dataMapper() {
    let mock_string: String = "dataMapper".to_string();
    let expected_string: String = "dataMapper".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_data_mapper_as_dataMapper() {
    let mock_string: String = "data_mapper".to_string();
    let expected_string: String = "dataMapper".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_data_dash_mapper_as_dataMapper() {
    let mock_string: String = "data-mapper".to_string();
    let expected_string: String = "dataMapper".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn camelize_DataMapper_as_dataMapper() {
    let mock_string: String = "DataMapper".to_string();
    let expected_string: String = "dataMapper".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_kebab() {
    let mock_string: String = "data-mapper-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_class() {
    let mock_string: String = "DataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_camel_case_when_camel() {
    let mock_string: String = "dataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_snake() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}
