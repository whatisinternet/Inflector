use std::ascii::*;
use regex::Regex;

pub fn to_snake_case<'a>(non_snake_case_string: String) -> String {
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

pub fn is_snake_case<'a>(test_string: String) -> bool{
    let snake_matcher = Regex::new(r"(?:[^-]?=^|[_])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_snake_case = false;
    if snake_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref()) {
            is_snake_case = true;
        }
    return is_snake_case;
}


#[cfg(test)]
mod snakecase_test;
