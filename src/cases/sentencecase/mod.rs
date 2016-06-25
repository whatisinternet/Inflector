use std::ascii::*;
use cases::snakecase::to_snake_case;

/// Converts a `String` to `Sentence case` `String`
///
/// #Examples
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_sentence_case(non_sentence_case_string: String) -> String {
    to_sentence_from_snake(to_snake_case(non_sentence_case_string))
}
    fn to_sentence_from_snake(non_sentence_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_sentence_case_string.chars() {
            if character.to_string() != "_" && character.to_string() != "-" && !first_character {
                result = format!("{}{}", result, character.to_ascii_lowercase());
            } else if character.to_string() == "_" || character.to_string() == "-" {
                result = format!("{} ", result);
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        result
    }

/// Determines of a `String` is `Sentence case`
///
/// #Examples
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_sentence_case(test_string: String) -> bool{
    test_string == to_sentence_case(test_string.clone())
}
