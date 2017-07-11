#![deny(warnings)]
use cases::case::*;
/// Converts a `&str` to `Sentence case` `String`
///
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: &str = "Foo bar";
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: &str = "FooBar";
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: &str = "fooBar";
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: &str = "FOO_BAR";
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: &str = "foo_bar";
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::to_sentence_case;
///     let mock_string: &str = "foo-bar";
///     let expected_string: String = "Foo bar".to_string();
///     let asserted_string: String = to_sentence_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_sentence_case(non_sentence_case_string: &str) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: true,
        injectable_char: ' ',
        has_seperator: true,
        inverted: true,
    };
    to_case_camel_like(non_sentence_case_string, options)
}
/// Determines of a `&str` is `Sentence case`
///
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "foo-bar-string-that-is-really-really-long";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "FooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "fooBarIsAReallyReallyLongString";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "Foo Bar Is A Really Really Long String";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "foo_bar_string_that_is_really_really_long";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "Foo";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "foo";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
///     use inflector::cases::sentencecase::is_sentence_case;
///     let mock_string: &str = "Foo bar string that is really really long";
///     let asserted_bool: bool = is_sentence_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_sentence_case(test_string: &str) -> bool {
    test_string == to_sentence_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod benchmarks {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_sentence(b: &mut Bencher) {
        b.iter(|| super::to_sentence_case("Foo BAR"));
    }

    #[bench]
    fn bench_is_sentence(b: &mut Bencher) {
        b.iter(|| super::is_sentence_case("Foo bar"));
    }

    #[bench]
    fn bench_sentence_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_sentence_case("foo_bar"));
    }

}

#[cfg(test)]
mod tests {
    define_test_group!(sentence_tests,
                    to_sentence_case,
                    is_sentence_case,
                    sentencecase,
                    "Foo bar",
                    "Foo bars");
}
