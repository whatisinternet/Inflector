use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;
use cases::camelcase::is_camel_case;

pub fn to_title_case<'a>(non_title_case_string: String) -> String {
    if is_title_case(non_title_case_string.clone()) {
        return non_title_case_string
    } else if
        is_camel_case(non_title_case_string.clone())
        || is_class_case(non_title_case_string.clone()){
        return to_title_from_class_or_camel(non_title_case_string)
    } else {
        return to_title_from_snake_or_kebab(non_title_case_string)
    }
}
    fn to_title_from_class_or_camel<'a>(non_title_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_title_case_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{} {}", result, character);
            } else if !first_character {
                result = format!("{}{}", result, character.to_ascii_lowercase());
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        return result
    }

    fn to_title_from_snake_or_kebab<'a>(non_title_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        let mut new_word: bool = true;
        for character in non_title_case_string.chars() {
            if new_word && !first_character {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                new_word = false;
            } else if character.to_string() != "_" && character.to_string() != "-" && !first_character {
                result = format!("{}{}", result, character.to_ascii_lowercase());
            } else if character.to_string() == "_" || character.to_string() == "-" {
                result = format!("{} ", result);
                new_word = true;
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
                new_word = false;
            }
        }
        println!("{}", result);
        return result
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
