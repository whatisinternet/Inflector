use regex::Regex;

use cases::classcase::is_class_case;
use cases::snakecase::to_snake_case;
use cases::uppercase::to_upper_case;

pub fn to_sentence_case<'a>(non_sentence_case_string: String) -> String {
    if is_sentence_case(non_sentence_case_string.clone()) {
        return non_sentence_case_string
    } else {
        return to_sentence_from_snake_or_kebab(to_snake_case(non_sentence_case_string));
    }
}
    fn to_sentence_from_snake_or_kebab<'a>(non_sentence_case_string: String) -> String{
        let mut split_string: Vec<&str> = non_sentence_case_string.split("_").collect();
        let first_word: &str = split_string.remove(0);
        let mut string_chars: Vec<char> = first_word.chars().collect();
        let mut out_string: String = "".to_string();

        if first_word != "" {
            let first_char: &str = &to_upper_case(string_chars.iter().nth(0).unwrap().to_string());
            string_chars.remove(0);
            let end_of_word: &str = &string_chars.iter().cloned().collect::<String>();
            out_string =  first_char.to_string() + end_of_word;
        }

        for string in split_string {
            out_string = out_string + " " +  string;
        }
        return out_string;
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
