use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;
use cases::snakecase::to_snake_case;

pub fn to_sentence_case<'a>(non_sentence_case_string: String) -> String {
    return to_sentence_from_snake(to_snake_case(non_sentence_case_string));
}
    fn to_sentence_from_snake<'a>(non_sentence_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_sentence_case_string.chars() {
            if character.to_string() != "_" && character.to_string() != "-" && !first_character {
                result = format!("{}{}", result, character.to_ascii_lowercase());
            } else if character.to_string() == "_" || character.to_string() == "-" {
                result = format!("{} ", result);
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        return result
    }

pub fn is_sentence_case<'a>(test_string: String) -> bool{
    let sentence_matcher= Regex::new(r"(^[A-Z])([^-|^_]*[ ][^A-Z][a-z]+)").unwrap();
    let mut is_sentence_case= false;
    if sentence_matcher.is_match(test_string.as_ref())
        && !is_class_case(test_string.clone()){
            is_sentence_case = true;
        }
    return is_sentence_case;
}


#[cfg(test)]
mod sentencecase_test;
