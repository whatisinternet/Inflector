use std::ascii::*;
use regex::Regex;

use cases::snakecase::to_snake_case;

pub fn to_title_case<'a>(non_title_case_string: String) -> String {
    return to_title_from_snake(to_snake_case(non_title_case_string));
}

    fn to_title_from_snake<'a>(non_sentence_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_sentence_case_string.chars() {
            if character.to_string() != "_" && character.to_string() != "-" && !first_character {
                result = format!("{}{}", result, character.to_ascii_lowercase());
                first_character = false
            } else if character.to_string() == "_" || character.to_string() == "-" {
                first_character = true;
            } else {
                result = format!("{} {}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        return result.trim().to_string();
    }

pub fn is_title_case<'a>(test_string: String) -> bool{
    let title_matcher= Regex::new(r"(^[A-Z][a-z]+)([^-|^_]*[ ][A-Z][a-z]+)").unwrap();
    let mut is_title_case= false;
    if title_matcher.is_match(test_string.as_ref()) {
        is_title_case = true;
    }
    return is_title_case;
}

#[cfg(test)]
mod titlecase_test;
