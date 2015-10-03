use std::ascii::*;
use regex::Regex;

pub fn to_upper_case<'a>(non_camelized_string: String) -> String {
    let mut result:String = "".to_string();
    for character in non_camelized_string.chars() {
        result = format!("{}{}", result, character.to_ascii_uppercase());
    }
    return result
}

pub fn is_upper_case<'a>(test_string: String) -> bool{
    let upper_matcher = Regex::new(r"^[A-Z| |_|-]+$").unwrap();
    let mut is_upper_case = false;
    if upper_matcher.is_match(test_string.as_ref()){
        is_upper_case = true;
    }
    return is_upper_case;
}

#[cfg(test)]
mod upper_test;
