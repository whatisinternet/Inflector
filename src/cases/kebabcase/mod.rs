use regex::Regex;

use cases::snakecase::to_snake_case;
use cases::lowercase::to_lower_case;

pub fn is_kebab_case<'a>(test_string: String) -> bool{
    let kebab_matcher = Regex::new(r"(?:[^_]?=^|[-])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    return kebab_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref())
}

pub fn to_kebab_case<'a>(non_kebab_case_string: String) -> String {
    let snake_case_string = to_snake_case(non_kebab_case_string);
    return to_kebab_from_snake(snake_case_string);
}
    fn to_kebab_from_snake<'a>(non_kebab_case_string: String) -> String {
        return to_lower_case(non_kebab_case_string.replace("_", "-"));
    }

#[cfg(test)]
mod kebabcase_test;
