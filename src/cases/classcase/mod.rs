use std::ascii::*;
use regex::Regex;

use cases::snakecase::to_snake_case;

pub fn to_class_case<'a>(non_class_case_string: String) -> String {
    return to_class_from_snake(to_snake_case(non_class_case_string));
}
    fn to_class_from_snake<'a>(non_class_case_string: String) -> String {
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

pub fn is_class_case<'a>(test_string: String) -> bool{
    let class_matcher = Regex::new(r"(^[A-Z])([^-|^_|^ ]*[a-z]+)").unwrap();
    let space_matcher = Regex::new(r" +").unwrap();
    if class_matcher.is_match(test_string.as_ref()) && !space_matcher.is_match(test_string.as_ref()){
        return true;
    }
    return false;
}

#[cfg(test)]
mod classcase_test;
