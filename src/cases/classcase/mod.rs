use std::ascii::*;
use regex::Regex;

pub fn to_class_case<'a>(non_class_case_string: String) -> String {
    let mut result:String = "".to_string();
    let mut new_word: bool = true;

    for character in non_class_case_string.chars() {
        if character.to_string() == "_" || character.to_string() == "-"  {
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

pub fn is_class_case<'a>(test_string: String) -> bool{
    let class_matcher = Regex::new(r"(^[A-Z])([^-|^_]*[a-z]+)").unwrap();
    let mut is_class_case = false;
    if class_matcher.is_match(test_string.as_ref()){
        is_class_case = true;
    }
    return is_class_case;
}

#[cfg(test)]
mod classcase_test;
