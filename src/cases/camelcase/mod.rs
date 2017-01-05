#![deny(warnings)]
use cases::case::*;

/// Converts a `String` to camelCase `String`
///
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "FooBar3".to_string();
///     let expected_string: String = "fooBar3".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::cases::camelcase::to_camel_case;
///     let mock_string: String = "Foo-Bar".to_string();
///     let expected_string: String = "fooBar".to_string();
///     let asserted_string: String = to_camel_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_camel_case(non_camelized_string: String) -> String {
    let options = CamelOptions {
        new_word: false,
        last_char: ' ',
        first_word: false,
        injectable_char: ' ',
        has_seperator: false,
        inverted: false
    };
    to_case_camel_like(non_camelized_string, options)
}

/// Determines if a `String` is camelCase bool``
///
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "fooBarIsAReallyReally3LongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == true);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
///
///
/// ```
/// ```
///     use inflector::cases::camelcase::is_camel_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_camel_case(mock_string);
///     assert!(asserted_bool == false);
/// ```
pub fn is_camel_case(test_string: String) -> bool {
    to_camel_case(test_string.clone()) == test_string
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_camel0(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "Foo bar".to_string();
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_camel1(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "foo_bar".to_string();
            super::to_camel_case(test_string)
        }
        );
    }

    #[bench]
    fn bench_camel2(b: &mut Bencher) {
        b.iter(|| {
            let test_string = "fooBar".to_string();
            super::to_camel_case(test_string)
        });
    }

    #[bench]
    fn bench_is_camel(b: &mut Bencher) {
        b.iter(|| {
            let test_string: String  = "Foo bar".to_string();
            super::is_camel_case(test_string)
        }
        );
    }
}


#[cfg(test)]
mod camel_tests{
    use ::cases::camelcase::to_camel_case;
    define_tests![
        to_camel_case;
        test_camel_case_to_camel_case => "fooBar" => "fooBar",
        test_class_case_to_camel_case => "FooBar" => "fooBar",
        test_screaming_snake_case_to_camel_case => "FOO_BAR" => "fooBar",
        test_kebab_case_to_camel_case => "foo-bar" => "fooBar",
        test_pascal_case_to_camel_case => "FooBar" => "fooBar",
        test_sentence_case_to_camel_case => "Foo bar" => "fooBar",
        test_snake_case_to_camel_case => "foo_bar" => "fooBar",
        test_title_case_to_camel_case => "Foo Bar" => "fooBar",
        test_table_case_to_camel_case => "foo_bars" => "fooBars",
        test_train_case_to_camel_case => "Foo-Bars" => "fooBars"
    ];
}
