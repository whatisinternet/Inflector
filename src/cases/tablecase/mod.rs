use cases::snakecase::to_snake_case;
use cases::sentencecase::to_sentence_case;
use string::pluralize::to_plural;

/// Converts a `String` to `table-case` `String`
///
/// #Examples
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "foo-bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "FOO_BAR".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "foo_bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "Foo Bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "Foo bar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "FooBar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
///
/// ```
/// use inflector::cases::tablecase::to_table_case;
/// let mock_string: String = "fooBar".to_string();
/// let expected_string: String = "foo_bars".to_string();
/// let asserted_string: String = to_table_case(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
pub fn to_table_case(non_table_case_string: String) -> String {
    let sentenceable_string: String = to_sentence_case(non_table_case_string.clone());
    let words: Vec<&str> = sentenceable_string.split(' ').collect();
    let mut sentence: String = "".to_string();
    for (word_index, _) in words.iter().enumerate().take((words.len() - 1)) {
        if word_index == 0 {
            sentence = words[word_index].to_string();
        } else {
            sentence = format!("{} {}", sentence, words[word_index]);
        }
    }
    sentence = format!("{} {}",
                       sentence,
                       to_plural(words[words.len() - 1].to_string()));
    to_snake_case(sentence)
}

/// Determines if a `String` is `table-case`
///
/// #Examples
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "foo_bar_strings".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == true);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
///
/// ```
///     use inflector::cases::tablecase::is_table_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_table_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_table_case(test_string: String) -> bool {
    test_string.clone() == to_table_case(test_string)
}
