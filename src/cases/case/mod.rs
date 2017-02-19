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
        .fold("".to_string(), |mut result, character|
              if char_is_seperator(&character) {
                  new_word = true;
                  result
              } else if character.is_numeric() {
                  new_word = true;
                  result.push(character);
                  result
              } else if last_char_lower_current_is_upper_or_new_word(new_word, &last_char, &character) {
                  new_word = false;
                  result = append_on_new_word(result, first_word, character, &camel_options);
                  first_word = false;
                  result
              } else {
                  last_char = character;
                  result.push(character.to_ascii_lowercase());
                  result
              })
}

#[inline]
fn to_snake_like_from_snake_like(convertable_string: String, replace_with: &str, case: &str) -> String {
    let mut new_word: bool = false;
    let mut last_char: char = ' ';
    convertable_string.chars()
        .fold("".to_string(), |mut result, character|
              if char_is_seperator(&character) {
                  new_word = true;
                  result.push(replace_with.chars().nth(0).unwrap_or('_'));
                  result
              } else {
                  new_word = false;
                  last_char = character;
                  snake_like_no_seperator(result, character, case)
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
        .fold("".to_string(), |acc, char_with_index|
              if requires_seperator(char_with_index, first_character, &convertable_string) {
                  snake_like_with_seperator(acc, replace_with, char_with_index.1, case)
              } else {
                  first_character = false;
                  snake_like_no_seperator(acc, char_with_index.1, case)
              })
}

#[inline]
fn append_on_new_word(mut result: String, first_word: bool, character: char, camel_options: &CamelOptions) -> String {
    if not_first_word_and_has_seperator(first_word, &camel_options) {
        result.push(camel_options.injectable_char);
    }
    if first_word_or_not_inverted(first_word, &camel_options) {
        result.push(character.to_ascii_uppercase());
    } else {
        result.push(character.to_ascii_lowercase());
    }
    result
}

#[inline]
fn not_first_word_and_has_seperator(first_word: bool, camel_options: &CamelOptions) -> bool {
    !first_word && camel_options.has_seperator
}

#[inline]
fn first_word_or_not_inverted(first_word: bool, camel_options: &CamelOptions) -> bool {
    !camel_options.inverted || first_word
}


#[inline]
fn last_char_lower_current_is_upper_or_new_word(new_word: bool, last_char: &char, character: &char) -> bool{
    new_word ||
        ((last_char.is_lowercase() && character.is_uppercase()) &&
         (*last_char != ' '))
}

#[inline]
fn char_is_seperator(character: &char) -> bool {
    *character == '-' || *character == '_' || *character == ' '
}

#[inline]
fn requires_seperator(char_with_index: (usize, char), first_character: bool, convertable_string: &str) -> bool {
    char_is_uppercase(char_with_index.1) &&
        !first_character &&
        next_or_previous_char_is_lowercase(&convertable_string, char_with_index.0)
}

#[inline]
fn snake_like_no_seperator(mut accumlator: String, current_char: char, case: &str) -> String {
    if case == "lower" {
        accumlator.push(current_char.to_ascii_lowercase());
        accumlator
    } else {
        accumlator.push(current_char.to_ascii_uppercase());
        accumlator
    }
}

#[inline]
fn snake_like_with_seperator(mut accumlator: String, replace_with: &str, current_char: char, case: &str) -> String {
    if case == "lower" {
        accumlator.push(replace_with.chars().nth(0).unwrap_or('_'));
        accumlator.push(current_char.to_ascii_lowercase());
        accumlator
    } else {
        accumlator.push(replace_with.chars().nth(0).unwrap_or('_'));
        accumlator.push(current_char.to_ascii_uppercase());
        accumlator
    }
}

#[inline]
fn next_or_previous_char_is_lowercase(convertable_string: &str, char_with_index: usize) -> bool {
    convertable_string.chars().nth(char_with_index + 1).unwrap_or('A').is_lowercase() ||
        convertable_string.chars().nth(char_with_index - 1).unwrap_or('A').is_lowercase()
}
#[inline]
fn char_is_uppercase(test_char: char) -> bool {
    test_char == test_char.to_ascii_uppercase()
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

#[test]
fn test_char_is_uppercase_when_it_is() {
    assert_eq!(char_is_uppercase('A'), true)
}

#[test]
fn test_char_is_uppercase_when_it_is_not() {
    assert_eq!(char_is_uppercase('a'), false)
}

#[test]
fn test_next_or_previous_char_is_lowercase_true() {
    assert_eq!(next_or_previous_char_is_lowercase("TestWWW", 3), true)
}

#[test]
fn test_next_or_previous_char_is_lowercase_false() {
    assert_eq!(next_or_previous_char_is_lowercase("TestWWW", 5), false)
}

#[test]
fn snake_like_with_seperator_lowers() {
    assert_eq!(snake_like_with_seperator("".to_string(), "^", 'c', "lower"), "^c".to_string())
}

#[test]
fn snake_like_with_seperator_upper() {
    assert_eq!(snake_like_with_seperator("".to_string(), "^", 'c', "upper"), "^C".to_string())
}

#[test]
fn snake_like_no_seperator_lower() {
    assert_eq!(snake_like_no_seperator("".to_string(), 'C', "lower"), "c".to_string())
}

#[test]
fn snake_like_no_seperator_upper() {
    assert_eq!(snake_like_no_seperator("".to_string(), 'c', "upper"), "C".to_string())
}

#[test]
fn requires_seperator_upper_not_first_wrap_is_safe_current_upper() {
    assert_eq!(requires_seperator((2, 'C'), false, "test"), true)
}

#[test]
fn requires_seperator_upper_not_first_wrap_is_safe_current_lower() {
    assert_eq!(requires_seperator((2, 'c'), false, "test"), false)
}

#[test]
fn requires_seperator_upper_first_wrap_is_safe_current_upper() {
    assert_eq!(requires_seperator((0, 'T'), true, "Test"), false)
}

#[test]
fn requires_seperator_upper_first_wrap_is_safe_current_lower() {
    assert_eq!(requires_seperator((0, 't'), true, "Test"), false)
}

#[test]
fn requires_seperator_upper_first_wrap_is_safe_current_lower_next_is_too() {
    assert_eq!(requires_seperator((0, 't'), true, "test"), false)
}

#[test]
fn test_char_is_seperator_dash() {
    assert_eq!(char_is_seperator(&'-'), true)
}

#[test]
fn test_char_is_seperator_underscore() {
    assert_eq!(char_is_seperator(&'_'), true)
}

#[test]
fn test_char_is_seperator_space() {
    assert_eq!(char_is_seperator(&' '), true)
}

#[test]
fn test_char_is_seperator_when_not() {
    assert_eq!(char_is_seperator(&'A'), false)
}

#[test]
fn test_last_char_lower_current_is_upper_or_new_word_with_new_word() {
    assert_eq!(last_char_lower_current_is_upper_or_new_word(true, &' ', &'-'), true)
}

#[test]
fn test_last_char_lower_current_is_upper_or_new_word_last_char_space() {
    assert_eq!(last_char_lower_current_is_upper_or_new_word(false, &' ', &'-'), false)
}

#[test]
fn test_last_char_lower_current_is_upper_or_new_word_last_char_lower_current_upper() {
    assert_eq!(last_char_lower_current_is_upper_or_new_word(false, &'a', &'A'), true)
}

#[test]
fn test_last_char_lower_current_is_upper_or_new_word_last_char_upper_current_upper() {
    assert_eq!(last_char_lower_current_is_upper_or_new_word(false, &'A', &'A'), false)
}

#[test]
fn test_last_char_lower_current_is_upper_or_new_word_last_char_upper_current_lower() {
    assert_eq!(last_char_lower_current_is_upper_or_new_word(false, &'A', &'a'), false)
}

#[test]
fn test_first_word_or_not_inverted_with_first_word() {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    assert_eq!(first_word_or_not_inverted(true, &options), true)
}

#[test]
fn test_first_word_or_not_inverted_not_first_word_not_inverted() {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    assert_eq!(first_word_or_not_inverted(false, &options), true)
}

#[test]
fn test_first_word_or_not_inverted_not_first_word_is_inverted() {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: true,
    };
    assert_eq!(first_word_or_not_inverted(false, &options), false)
}

#[test]
fn test_not_first_word_and_has_seperator_is_first_and_not_seperator() {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    assert_eq!(not_first_word_and_has_seperator(true, &options), false)
}

#[test]
fn test_not_first_word_and_has_seperator_not_first_and_not_seperator() {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false,
    };
    assert_eq!(not_first_word_and_has_seperator(false, &options), false)
}

#[test]
fn test_not_first_word_and_has_seperator_not_first_and_has_seperator() {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: true,
        inverted: false,
    };
    assert_eq!(not_first_word_and_has_seperator(false, &options), true)
}
