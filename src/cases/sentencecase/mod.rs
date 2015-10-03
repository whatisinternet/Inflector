use std::ascii::*;
use regex::Regex;

pub fn to_sentence_case<'a>(non_sentence_case_string: String) -> String {
    if is_sentence_case(non_sentence_case_string.clone()) {
        return non_sentence_case_string
    } else {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_sentence_case_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{} {}", result, character.to_ascii_lowercase());
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        return result
    }
}

pub fn is_sentence_case<'a>(test_string: String) -> bool{
    let sentence_matcher= Regex::new(r"(^[A-Z])([^-|^_|]*[a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_sentence_case= false;
    if sentence_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref()) {
            is_sentence_case = true;
        }
    return is_sentence_case;
}


#[cfg(test)]
mod sentencecase_test;
