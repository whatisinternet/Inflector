use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;
use cases::snakecase::to_snake_case;
use cases::uppercase::to_upper_case;

pub fn to_camel_case<'a>(non_camelized_string: String) -> String {
    if is_camel_case(non_camelized_string.clone()) {
        return non_camelized_string
    } else {
        return to_camel_from_snake(to_snake_case(non_camelized_string));
    }
}

    fn to_camel_from_snake<'a>(non_camelized_string: String) -> String{
        let mut split_string: Vec<&str> = non_camelized_string.split("_").collect();
        let mut out_string: String;
        out_string = format!("{}", split_string.remove(0));
        for string in split_string {
            let mut string_chars: Vec<char> = string.chars().collect();
            let first_char: String = to_upper_case(string.chars().nth(0).unwrap().to_string());
            string_chars.remove(0);
            let end_of_word: String = string_chars.iter().cloned().collect::<String>();
            out_string = format!("{}{}{}", out_string, first_char, end_of_word);
        }
        return out_string;
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
