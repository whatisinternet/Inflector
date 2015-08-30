extern crate regex;

use std::ascii::*;
use regex::Regex;

fn to_class_case<'a>(non_class_case_string: String) -> String {
    let mut result:String = "".to_string();
    let mut new_word: bool = true;

    for character in non_class_case_string.chars() {
        if character.to_string() == "_" {
            new_word = true;
        } else if new_word {
            result = format!("{}{}", result, character.to_ascii_uppercase());
            new_word = false;
        } else {
            result = format!("{}{}", result, character.to_ascii_lowercase());
        }
    }
    return result
}

fn to_camel_case<'a>(non_camelized_string: String) -> String {
    let mut result:String = "".to_string();
    let mut new_word: bool = false;

    for character in non_camelized_string.chars() {
        if character.to_string() == "_" {
            new_word = true;
        } else if new_word {
            result = format!("{}{}", result, character.to_ascii_uppercase());
            new_word = false;
        } else {
            result = format!("{}{}", result, character.to_ascii_lowercase());
        }
    }
    return result
}

fn to_snake_case<'a>(non_snake_case_string: String) -> String {
    let mut result:String = "".to_string();
    let mut first_character: bool = true;
    for character in non_snake_case_string.chars() {
        if character == character.to_ascii_uppercase() && !first_character {
            result = format!("{}_{}", result, character.to_ascii_lowercase());
        } else {
            result = format!("{}{}", result, character.to_ascii_lowercase());
            first_character = false;
        }
    }
    return result
}

fn to_kebab_case<'a>(non_kebab_case_string: String) -> String {
    let mut kebab_string: String = non_kebab_case_string.clone();
    if is_camel_case(non_kebab_case_string.clone()) {
        kebab_string = to_kebab_from_camel(non_kebab_case_string.clone());
    }else if is_snake_case(non_kebab_case_string.clone()) {
        kebab_string = to_kebab_from_snake(non_kebab_case_string.clone());
    }
    return kebab_string;
}

fn to_kebab_from_camel<'a>(non_kebab_case_string: String) -> String {
    let mut result:String = "".to_string();
    let mut first_character: bool = true;
    for character in non_kebab_case_string.chars() {
        if character == character.to_ascii_uppercase() && !first_character {
            result = format!("{}-{}", result, character.to_ascii_lowercase());
        } else {
            result = format!("{}{}", result, character.to_ascii_lowercase());
            first_character = false;
        }
    }
    return result
}
fn to_kebab_from_snake<'a>(non_kebab_case_string: String) -> String {
    return non_kebab_case_string.replace("_", "-");
}

fn is_class_case<'a>(test_string: String) -> bool{
    let class_matcher = Regex::new(r"(^[A-Z])([^-|^_]*[a-z]+)").unwrap();
    let mut is_class_case = false;
    if class_matcher.is_match(test_string.as_ref()){
        is_class_case = true;
    }
    return is_class_case;
}

fn is_camel_case<'a>(test_string: String) -> bool{
    let camel_matcher = Regex::new(r"(^|[A-Z])([^-|^_]*[a-z]+)").unwrap();
    let kebab_snake_matcher = Regex::new(r"[-|_]").unwrap();
    let mut is_camel_case = false;
    if camel_matcher.is_match(test_string.as_ref())
        && !kebab_snake_matcher.is_match(test_string.as_ref())
        && !is_class_case(test_string){
            is_camel_case = true;
        }
    return is_camel_case;
}

fn is_snake_case<'a>(test_string: String) -> bool{
    let snake_matcher = Regex::new(r"(?:[^-]?=^|[_])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_snake_case = false;
    if snake_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref()) {
            is_snake_case = true;
        }
    return is_snake_case;
}

fn is_kebab_case<'a>(test_string: String) -> bool{
    let kebab_matcher = Regex::new(r"(?:[^_]?=^|[-])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_kebab_case = false;
    if kebab_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref()) {
            is_kebab_case = true;
        }
    return is_kebab_case;
}

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
    let asserted_string: String = to_class_case(mock_string);
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
fn kebab_case_dataMapper_as_data_mapper() {
    let mock_string: String = "dataMapper".to_string();
    let expected_string: String = "data-mapper".to_string();
    let asserted_string: String = to_kebab_case(mock_string);
    assert!(asserted_string == expected_string);
}
