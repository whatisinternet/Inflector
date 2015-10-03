use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;

pub fn to_camel_case<'a>(non_camelized_string: String) -> String {
    if is_camel_case(non_camelized_string.clone()) {
        return non_camelized_string
    } else if is_class_case(non_camelized_string.clone()) {
        return to_camel_from_class(non_camelized_string.clone());
    } else {
        return to_camel_from_snake_or_kebab(non_camelized_string.clone());
    }
}

    fn to_camel_from_snake_or_kebab<'a>(non_camelized_string: String) -> String{
        let mut result:String = "".to_string();
        let mut new_word: bool = false;

        for character in non_camelized_string.chars() {
            if character.to_string() == "_"
                || character.to_string() == "-"
                || character.to_string() == " " {
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

    fn to_camel_from_class<'a>(non_camelized_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;

        for character in non_camelized_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{}{}", result, character);
            } else {
                result = format!("{}{}", result, character.to_ascii_lowercase());
                first_character = false;
            }
        }
        return result
    }

pub fn is_camel_case<'a>(test_string: String) -> bool{
    let camel_matcher = Regex::new(r"(^|[A-Z])([^-|^_|^ ]*[a-z]+)").unwrap();
    let kebab_snake_matcher = Regex::new(r"[-|_| ]").unwrap();
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
