use regex::Regex;

use cases::snakecase::to_snake_case;
use cases::uppercase::to_upper_case;

pub fn to_class_case<'a>(non_class_case_string: String) -> String {
    if is_class_case(non_class_case_string.clone()){
        return non_class_case_string
    } else {
        return to_class_from_snake(to_snake_case(non_class_case_string));
    }
}
    fn to_class_from_snake<'a>(non_class_case_string: String) -> String{
        let split_string: Vec<&str> = non_class_case_string.split("_").collect();
        let mut out_string: String = "".to_string();
        for string in split_string {
            if string != "" {
                let mut string_chars: Vec<char> = string.chars().collect();
                let first_char: &str = &to_upper_case(string_chars.iter().nth(0).unwrap().to_string());
                string_chars.remove(0);
                let end_of_word: &str = &string_chars.iter().cloned().collect::<String>();
                out_string = out_string + first_char + end_of_word;
            }
        }
        return out_string;
    }

pub fn is_class_case<'a>(test_string: String) -> bool{
    let class_matcher = Regex::new(r"(^[A-Z])([^-|^_|^ ]*[a-z]+)").unwrap();
    let space_matcher = Regex::new(r" +").unwrap();
    if class_matcher.is_match(test_string.as_ref()) && !space_matcher.is_match(test_string.as_ref()){
        return true;
    }
    return false;
}

#[cfg(test)]
mod classcase_test;
