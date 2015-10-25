use regex::Regex;

use cases::snakecase::to_snake_case;
use cases::uppercase::to_upper_case;

pub fn to_title_case<'a>(non_title_case_string: String) -> String {
    if is_title_case(non_title_case_string.clone()) {
        return non_title_case_string
    } else {
        return to_title_from_snake(to_snake_case(non_title_case_string));
    }
}
    fn to_title_from_snake<'a>(non_camelized_string: String) -> String{
        let split_string: Vec<&str> = non_camelized_string.split("_").collect();
        let mut out_string: String = "".to_string();
        for string in split_string {
            if string != "" {
                let mut string_chars: Vec<char> = string.chars().collect();
                let first_char: &str = &to_upper_case(string_chars.iter().nth(0).unwrap().to_string());
                string_chars.remove(0);
                let end_of_word: &str = &string_chars.iter().cloned().collect::<String>();
                out_string = out_string + " " + first_char + end_of_word;
            }
        }
        return out_string.trim().to_string();
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
