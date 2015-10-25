use std::ascii::*;
use regex::Regex;

pub fn to_lower_case<'a>(non_camelized_string: String) -> String {
    let mut result:String = "".to_string();
    if !is_lower_case(non_camelized_string.clone()) {
        for character in non_camelized_string.chars() {
            result = format!("{}{}", result, character.to_ascii_lowercase());
        }
    } else {
        result = non_camelized_string;
    }
    return result
}

pub fn is_lower_case<'a>(test_string: String) -> bool{
    let lower_matcher = Regex::new(r"^[a-z| |_|-]+$").unwrap();
    let mut is_lower_case = false;
    if lower_matcher.is_match(test_string.as_ref()){
        is_lower_case = true;
    }
    return is_lower_case;
}

#[cfg(test)]
mod lower_test;
