#![deny(warnings)]
use std::ascii::*;

pub struct CamelOptions {
    pub new_word: bool,
    pub last_char: char,
    pub first_word: bool,
    pub injectable_char: char,
    pub has_seperator: bool,
    pub inverted: bool,
}

pub fn to_case_snake_like(convertable_string: String, replace_with: &str, case: &str) -> String {
    let seperators: &[char] = &[' ', '-', '_'];
    if convertable_string.contains(seperators) {
        to_snake_like_from_snake_like(convertable_string, replace_with, case)
    } else {
        to_snake_like_from_camel_or_class(convertable_string, replace_with, case)
    }
}

pub fn to_case_camel_like(convertable_string: String, camel_options: CamelOptions) -> String {
    let mut new_word: bool = camel_options.new_word;
    let mut first_word: bool = camel_options.first_word;
    let mut last_char: char = camel_options.last_char;
    convertable_string.chars()
        .fold("".to_string(),
              |mut result, character| if character == '-' || character == '_' || character == ' ' {
                  new_word = true;
                  result
              } else if character.is_numeric() {
                  new_word = true;
                  result.push(character);
                  result
              } else if new_word ||
                                                ((last_char.is_lowercase() &&
                                                  character.is_uppercase()) &&
                                                 (last_char != ' ')) {
                  new_word = false;
                  if !first_word && camel_options.has_seperator {
                      result.push(camel_options.injectable_char);
                  }
                  if !camel_options.inverted || first_word {
                      result.push(character.to_ascii_uppercase());
                  } else {
                      result.push(character.to_ascii_lowercase());
                  }
                  first_word = false;
                  result
              } else {
                  last_char = character;
                  result.push(character.to_ascii_lowercase());
                  result
              })
}

#[inline]
fn to_snake_like_from_snake_like(convertable_string: String,
                                 replace_with: &str,
                                 case: &str)
                                 -> String {
    let mut new_word: bool = false;
    let mut last_char: char = ' ';
    convertable_string.chars()
        .fold("".to_string(),
              |mut result, character| if character == '-' || character == '_' || character == ' ' {
                  new_word = true;
                  result.push(replace_with.chars().nth(0).unwrap_or('_'));
                  result
              } else if new_word ||
                                                ((last_char.is_lowercase() &&
                                                  character.is_uppercase()) &&
                                                 (last_char != ' ')) {
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
              })
}

#[inline]
fn to_snake_like_from_camel_or_class(convertable_string: String,
                                     replace_with: &str,
                                     case: &str)
                                     -> String {
    let mut first_character: bool = true;
    convertable_string.chars()
        .enumerate()
        .fold("".to_string(),
              |mut acc, char_with_index| if char_with_index.1 ==
                                            char_with_index.1.to_ascii_uppercase() &&
                                            !first_character &&
                                            (convertable_string.chars()
                  .nth(char_with_index.0 + 1)
                  .unwrap_or('A')
                  .is_lowercase() ||
                                             convertable_string.chars()
                  .nth(char_with_index.0 - 1)
                  .unwrap_or('A')
                  .is_lowercase()) {
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
              })
}

#[macro_export]
macro_rules! define_test_group {
    ($module_name: ident,
     $method: ident,
     $use_mod: ident,
     $expected: expr,
     $expected_plural: expr) => {
        #[cfg(test)]
        mod $module_name {
            use ::cases::$use_mod::$method;
            define_tests![
                $method;
                from_camel_case             => "fooBar"     => $expected,
                from_screaming_snake_case   => "FOO_BAR"    => $expected,
                from_kebab_case             => "foo-bar"    => $expected,
                from_pascal_case            => "FooBar"     => $expected,
                from_sentence_case          => "Foo bar"    => $expected,
                from_snake_case             => "foo_bar"    => $expected,
                from_title_case             => "Foo Bar"    => $expected,
                from_train_case             => "Foo-Bars"   => $expected_plural
            ];
            define_gated_tests![
                $method;
                from_class_case             => "FooBar"     => $expected,
                from_table_case             => "foo_bars"   => $expected_plural
            ];
        }
    }
}

macro_rules! define_gated_tests{
    ($method: ident; $($test_name:ident => $to_convert:expr => $expected:expr ), *) => {
        $(
            #[test]
            #[cfg(not(feature = "lightweight"))]
            fn $test_name() {
                assert_eq!($method($to_convert.to_string()), $expected.to_string())
            }
        )*
    }
}
macro_rules! define_tests{
    ($method: ident; $($test_name:ident => $to_convert:expr => $expected:expr ), *) => {
        $(
            #[test]
            fn $test_name() {
                assert_eq!($method($to_convert.to_string()), $expected.to_string())
            }
        )*
    }
}
