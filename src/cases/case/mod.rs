use std::ascii::*;

pub fn to_case_snake_like(
    convertable_string: String,
    replace_with: &str,
    case: &str
    ) -> String {
    let seperators: &[char] = &[' ', '-', '_'];
    if convertable_string.contains(seperators) {
        if case == "lower" {
            convertable_string.replace(seperators, replace_with).to_lowercase()
        } else {
            convertable_string.replace(seperators, replace_with).to_uppercase()
        }
    } else {
        to_snake_like_from_camel_or_class(convertable_string, replace_with, case)
    }
}


fn to_snake_like_from_camel_or_class(
    convertable_string: String,
    replace_with: &str,
    case: &str
    ) -> String {
    let mut first_character: bool = true;
    convertable_string
        .chars()
        .fold("".to_string(), |acc, character|
              if character == character.to_ascii_uppercase() && !first_character {
                  if case == "lower" {
                    format!("{}{}{}", acc, replace_with, character.to_ascii_lowercase())
                  } else {
                    format!("{}{}{}", acc, replace_with, character.to_ascii_uppercase())
                  }
              } else {
                  first_character = false;
                  if case == "lower" {
                    format!("{}{}", acc, character.to_ascii_lowercase())
                  } else {
                    format!("{}{}", acc, character.to_ascii_uppercase())
                  }
              }
        )
}
