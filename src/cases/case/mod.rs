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
    let mut prev_character_was_ucase: bool = false;
    let mut fragment: String = String::new();
    let mut result: Vec<String> = vec![];

    for character in convertable_string.chars() {
        let character_is_ucase = character == character.to_ascii_uppercase();

        if fragment.len() != 0 {
            if character_is_ucase && !prev_character_was_ucase {
                result.push(fragment);
                fragment = String::new();
            } else if !character_is_ucase && prev_character_was_ucase {
                let count = fragment.chars().count();

                // last uppercase letter in a run of uppercase 
                // letters is the start of a new capitalised word:

                let ucase_run: String = fragment.chars().take(count - 1).collect();
                let new_fragment: String = fragment.chars().skip(count - 1).collect();

                if ucase_run.len() != 0 {
                    result.push(ucase_run);
                }

                fragment = new_fragment;
            }
        }

        if case == "lower" {
            fragment.push(character.to_ascii_lowercase());
        } else {
            fragment.push(character.to_ascii_uppercase());
        }

        prev_character_was_ucase = character_is_ucase;
    }

    if fragment.len() != 0 {
        result.push(fragment);
    }

    result.join(&replace_with.chars().nth(0).unwrap_or('_').to_string())
}
