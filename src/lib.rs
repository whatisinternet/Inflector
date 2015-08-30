extern crate regex;

use std::ascii::*;
use regex::Regex;

mod classcase;
mod camelcase;
mod snakecase;
mod kebabcase;

use classcase::to_class_case;
use classcase::is_class_case;

use camelcase::to_camel_case;
use camelcase::is_camel_case;

use snakecase::to_snake_case;
use snakecase::is_snake_case;

use kebabcase::to_kebab_case;
use kebabcase::is_kebab_case;


#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_class_case_when_class() {
    let mock_string: String = "DataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_class_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_class_case_when_kebab() {
    let mock_string: String = "data-mapper-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_class_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_class_case_when_camel() {
    let mock_string: String = "dataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_class_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_class_case_when_snake() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_class_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_kebab() {
    let mock_string: String = "data-mapper-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_class() {
    let mock_string: String = "DataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_camel_case_when_camel() {
    let mock_string: String = "dataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_camel_case_when_snake() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_camel_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_kebab_case_when_kebab() {
    let mock_string: String = "data-mapper-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_class() {
    let mock_string: String = "DataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_camel() {
    let mock_string: String = "dataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_kebab_case_when_snake() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_kebab_case(mock_string);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_kebab() {
    let mock_string: String = "data-mapper-string-that-is-really-really-long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_class() {
    let mock_string: String = "DataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_falsey_value_for_is_snake_case_when_camel() {
    let mock_string: String = "dataMapperIsAReallyReallyLongString".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    println!("{}",asserted_bool);
    assert!(asserted_bool == false);
}

#[test] #[allow(non_snake_case)]
fn returns_truthy_value_for_is_snake_case_when_snake() {
    let mock_string: String = "data_mapper_string_that_is_really_really_long".to_string();
    let asserted_bool: bool = is_snake_case(mock_string);
    assert!(asserted_bool == true);
}

#[test] #[allow(non_snake_case)]
fn ClassCase_data_mapper_as_DataMapper() {
    let mock_string: String = "data_mapper".to_string();
    let expected_string: String = "DataMapper".to_string();
    let asserted_string: String = classcase::to_class_case(mock_string);
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
fn snake_case_dataMapper_as_data_mapper() {
    let mock_string: String = "DataMapper".to_string();
    let expected_string: String = "data_mapper".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_data_mapper_as_data_mapper() {
    let mock_string: String = "data_mapper".to_string();
    let expected_string: String = "data-mapper".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_DataMapper_as_data_mapper() {
    let mock_string: String = "DataMapper".to_string();
    let expected_string: String = "data-mapper".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}

#[test] #[allow(non_snake_case)]
fn kebab_case_dataMapper_as_data_mapper() {
    let mock_string: String = "dataMapper".to_string();
    let expected_string: String = "data-mapper".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}
