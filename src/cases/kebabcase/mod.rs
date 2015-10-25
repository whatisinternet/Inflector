use regex::Regex;

use cases::snakecase::to_snake_case;

pub fn is_kebab_case<'a>(test_string: String) -> bool{
    let kebab_matcher = Regex::new(r"(?:[^_]?=^|[-])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    return kebab_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref())
}

pub fn to_kebab_case<'a>(non_kebab_case_string: String) -> String {
    return to_kebab_from_snake(to_snake_case(non_kebab_case_string));
}
    fn to_kebab_from_snake<'a>(non_kebab_case_string: String) -> String {
        return non_kebab_case_string.replace("_", "-");
    }

#[cfg(test)]
mod kebabcase_test;
