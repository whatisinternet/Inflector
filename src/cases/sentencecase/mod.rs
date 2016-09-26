#![deny(warnings)]
use cases::case::*;
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
    let snake_cased: String = to_case_snake_like(non_sentence_case_string, " ", "lower");
    let split_on_first: (&str, &str) = snake_cased.split_at(1);
    format!("{}{}", split_on_first.0.to_uppercase(), split_on_first.1)
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
pub fn is_sentence_case(test_string: String) -> bool {
    test_string == to_sentence_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_sentence(b: &mut Bencher) {
        b.iter(|| super::to_sentence_case("Foo BAR".to_string()));
    }

    #[bench]
    fn bench_is_sentence(b: &mut Bencher) {
        b.iter(|| super::is_sentence_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_sentence_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_sentence_case("foo_bar".to_string()));
    }
}
