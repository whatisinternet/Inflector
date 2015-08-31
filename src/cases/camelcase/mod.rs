use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;

pub fn to_camel_case<'a>(non_camelized_string: String) -> String {
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

pub fn is_camel_case<'a>(test_string: String) -> bool{
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

#[cfg(test)]
mod camelcase_test;
