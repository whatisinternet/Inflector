use regex::Regex;

use cases::snakecase::to_snake_case;

pub fn to_foreign_key<'a>(non_foreign_key_string: String) -> String {
    if is_foreign_key(non_foreign_key_string.clone()){
        return non_foreign_key_string;
    } else {
        if non_foreign_key_string.contains("::") {
            let split_string: Vec<&str> = non_foreign_key_string.split("::").collect();
            return safe_convert(split_string[split_string.len() - 1].to_string());

        } else {
            return safe_convert(non_foreign_key_string);
        }
    }
}
    fn safe_convert<'a>(safe_string: String) -> String{
        let snake_cased: String = to_snake_case(safe_string);
        return format!("{}{}", snake_cased, "_id");
    }

pub fn is_foreign_key<'a>(test_string: String) -> bool{
    let foreign_key_matcher= Regex::new(r"(?:[^-|^ ]?=^|[_])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_foreign_key: bool = false;
    if foreign_key_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref())
        && test_string.ends_with("_id"){
            is_foreign_key= true;
        }
    return is_foreign_key;
}


#[cfg(test)]
mod foreignkey_test;
