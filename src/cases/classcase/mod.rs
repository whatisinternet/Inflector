use std::ascii::*;
use regex::Regex;

use cases::camelcase::is_camel_case;

pub fn to_class_case<'a>(non_class_case_string: String) -> String {
    if is_class_case(non_class_case_string.clone()){
        return non_class_case_string
    } else if is_camel_case(non_class_case_string.clone()) {
        return to_class_from_camel(non_class_case_string.clone());
    } else {
        return to_class_from_snake_or_kebab(non_class_case_string.clone());
    }
}

    fn to_class_from_camel<'a>(non_class_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;

        for character in non_class_case_string.chars() {
            if character == character.to_ascii_lowercase() && !first_character {
                result = format!("{}{}", result, character);
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        return result
    }

    fn to_class_from_snake_or_kebab<'a>(non_class_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut new_word: bool = true;

        for character in non_class_case_string.chars() {
            if character.to_string() == "_"
                || character.to_string() == "-"
                || character.to_string() == " "  {
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
    let mut is_class_case = false;
    if class_matcher.is_match(test_string.as_ref()) && !space_matcher.is_match(test_string.as_ref()){
        is_class_case = true;
    }
    return is_class_case;
}

#[cfg(test)]
mod classcase_test;
