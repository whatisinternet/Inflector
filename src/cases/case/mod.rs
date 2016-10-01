#![deny(warnings)]
use std::ascii::*;

pub fn to_case_snake_like(
    convertable_string: String,
    replace_with: &str,
    case: &str
    ) -> String {
    let seperators: &[char] = &[' ', '-', '_'];
    if convertable_string.contains(seperators) {
        to_snake_like_from_snake_like(convertable_string, replace_with, case)
    } else {
        to_snake_like_from_camel_or_class(convertable_string, replace_with, case)
    }
}

fn to_snake_like_from_snake_like(
    convertable_string: String,
    replace_with: &str,
    case: &str
    ) -> String {
    let mut new_word: bool = false;
    let mut last_char: char = ' ';
    convertable_string
        .chars()
        .fold("".to_string(), |mut result, character|
            if character == '-' || character == '_' || character == ' ' {
                new_word = true;
                result.push(replace_with.chars().nth(0).unwrap_or('_'));
                result
            } else if new_word || (
                (last_char.is_lowercase() && character.is_uppercase()) &&
                (last_char != ' ')
                ){
                new_word = false;
                if case == "lower" {
                    result.push(character.to_ascii_lowercase());
                } else {
                    result.push(character.to_ascii_uppercase());
                }
                result
            } else {
                last_char = character;
                if case == "lower" {
                    result.push(character.to_ascii_lowercase());
                } else {
                    result.push(character.to_ascii_uppercase());
                }
                result
            }
        )
}

fn to_snake_like_from_camel_or_class(
    convertable_string: String,
    replace_with: &str,
    case: &str
    ) -> String {
        let mut first_character: bool = true;
    convertable_string
        .chars()
        .enumerate()
        .fold("".to_string(), |mut acc, char_with_index|
              if char_with_index.1 == char_with_index.1.to_ascii_uppercase() &&
                  !first_character &&
                  (convertable_string.chars().nth(char_with_index.0 + 1).unwrap_or('A').is_lowercase() ||
                  convertable_string.chars().nth(char_with_index.0 - 1).unwrap_or('A').is_lowercase()
                  )
                  {
                      if case == "lower" {
                          acc.push(replace_with.chars().nth(0).unwrap_or('_'));
                          acc.push(char_with_index.1.to_ascii_lowercase());
                          acc
                      } else {
                          acc.push(replace_with.chars().nth(0).unwrap_or('_'));
                          acc.push(char_with_index.1.to_ascii_uppercase());
                          acc
                      }
              } else {
                    first_character = false;
                    if case == "lower" {
                        acc.push(char_with_index.1.to_ascii_lowercase());
                        acc
                    } else {
                        acc.push(char_with_index.1.to_ascii_uppercase());
                        acc
                    }
              }
        )
}
