use suffix::foreignkey::*;

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_foreign_key_when_sentence() {
    let mock_string: String = "Data mapper string that is really really long".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_foreign_key_when_kebab() {
    let mock_string: String = "data-mapper-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_foreign_key_when_class() {
    let mock_string: String = "DataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_foreign_key_when_title() {
    let mock_string: String = "Data Mapper Is A Really Really Long String".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_foreign_key_when_camel() {
    let mock_string: String = "dataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_foreign_key_when_snake() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_foreign_key_when_foreign_key() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long_id".to_string();
    let asserted_bool: bool = is_foreign_key(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_data_mapper_as_data_mapper() {
    let mock_string: String = "data_mapper".to_string();
    let expected_string: String = "data_mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_Data_space_mapper_as_data_mapper() {
    let mock_string: String = "Data mapper".to_string();
    let expected_string: String = "data_mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_Data_space_Mapper_as_data_mapper() {
    let mock_string: String = "Data Mapper".to_string();
    let expected_string: String = "data_mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_Data_namespace_Mapper_as_data_mapper() {
    let mock_string: String = "Data::Mapper".to_string();
    let expected_string: String = "mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_Test_namespace_Data_namespace_Mapper_as_data_mapper() {
    let mock_string: String = "Test::Data::Mapper".to_string();
    let expected_string: String = "mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_DataMapper_as_data_mapper() {
    let mock_string: String = "DataMapper".to_string();
    let expected_string: String = "data_mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_dataMapper_as_data_mapper() {
    let mock_string: String = "dataMapper".to_string();
    let expected_string: String = "data_mapper_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn foreign_key_dataMapper3_as_data_mapper() {
    let mock_string: String = "dataMapper3".to_string();
    let expected_string: String = "data_mapper_3_id".to_string();
    let asserted_string: String = to_foreign_key(mock_string);
    assert!(asserted_string == expected_string);
}
