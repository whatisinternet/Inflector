use std::ascii::*;
use regex::Regex;

use cases::lowercase::to_lower_case;

pub fn to_snake_case<'a>(non_snake_case_string: String) -> String {
    if non_snake_case_string.contains(" ")
        || non_snake_case_string.contains("_")
        || non_snake_case_string.contains("-") {
        return to_snake_from_sentence_or_kebab(non_snake_case_string);
    } else {
        return to_snake_from_camel_or_class(non_snake_case_string);
    }
}
    fn to_snake_from_camel_or_class <'a>(non_snake_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_snake_case_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{}_{}", result, character.to_ascii_lowercase());
            } else {
                result = format!("{}{}", result, character.to_ascii_lowercase());
                first_character = false;
            }
        }
        return result
    }
    // fn to_snake_from_camel_or_class <'a>(non_snake_case_string: String) -> String {
    //     let re = Regex::new(r"(?P<a>[A-Z0-9])").unwrap();
    //     let result: String = re.replace_all(&non_snake_case_string, "_$a$b").to_string();
    //     return to_lower_case(result.trim_left_matches("_").to_string());
    // }

    fn to_snake_from_sentence_or_kebab<'a>(non_snake_case_string: String) -> String {
        return to_lower_case(non_snake_case_string.replace(" ", "_").replace("-", "_"));
    }

pub fn is_snake_case<'a>(test_string: String) -> bool{
    let snake_matcher = Regex::new(r"(?:[^-|^ ]?=^|[_])([a-z]+)").unwrap();
    if snake_matcher.is_match(test_string.as_ref()){
        return true;
    }
    return false;
}


#[cfg(test)]
mod snakecase_test;
