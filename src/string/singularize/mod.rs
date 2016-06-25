use regex::Regex;
use regex::Captures;
use string::constants::UNACCONTABLE_WORDS;

const SINGULAR_RULES: [&'static str; 27] = ["ews", "", "um", "sis", "sis", "fe", "", "", "f", "y",
                                            "eries", "ovie", "", "ouse", "", "", "is", "xis",
                                            "us", "", "", "ex", "ix", "", "", "", ""];


/// Converts a `String` to singularized `String`
///
/// #Examples
/// ```
///     use inflector::string::singularize::to_singular;
///     let mock_string: String = "foo_bars".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::singularize::to_singular;
///     let mock_string: String = "oxen".to_string();
///     let expected_string: String = "ox".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::singularize::to_singular;
///     let mock_string: String = "crates".to_string();
///     let expected_string: String = "crate".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::singularize::to_singular;
///     let mock_string: String = "boxes".to_string();
///     let expected_string: String = "box".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::singularize::to_singular;
///     let mock_string: String = "vengeance".to_string();
///     let expected_string: String = "vengeance".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::singularize::to_singular;
///     let mock_string: String = "yoga".to_string();
///     let expected_string: String = "yoga".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
pub fn to_singular(non_singular_string: String) -> String {
    if UNACCONTABLE_WORDS.contains(&non_singular_string.as_ref()) {
        non_singular_string
    } else {
        let safe_out_string: String = non_singular_string.to_string();
        let maybe_regex_match: Option<String> = singularize_string(non_singular_string);
        maybe_regex_match.unwrap_or(safe_out_string)
    }
}

fn singularize_string(non_singular_string: String) -> Option<String> {
    let singular_regexes: [Regex; 27] = singular_rules_regexes();
    for singular_index in 0..singular_regexes.len() {
        let maybe_match: Option<String> =
            singularize_string_from_capture_groups_if_match(non_singular_string.to_string(),
                                                            singular_index);

        if maybe_match.is_some() {
            return Some(maybe_match.unwrap_or(non_singular_string));
        }
    }
    return None;
}

fn singularize_string_from_capture_groups_if_match(non_singular_string: String,
                                                   singular_index: usize)
                                                   -> Option<String> {
    let singular_regexes: [Regex; 27] = singular_rules_regexes();

    if singular_regexes[singular_index].is_match(&non_singular_string) {

        let cap: Captures = singular_regexes[singular_index]
            .captures(&non_singular_string)
            .unwrap();

        Some(singularize_string_from_capture_groups(cap, singular_index))
    } else {
        None
    }
}

fn singularize_string_from_capture_groups(capture_groups: Captures,
                                          singular_index: usize)
                                          -> String {
    if capture_groups.len() > 2 {

        return format!("{}{}{}",
                       capture_groups.at(1).unwrap_or(""),
                       capture_groups.at(2).unwrap_or(""),
                       SINGULAR_RULES[singular_index]);
    } else {

        return format!("{}{}",
                       capture_groups.at(1).unwrap_or(""),
                       SINGULAR_RULES[singular_index]);
    }
}

fn singular_rules_regexes() -> [Regex; 27] {
    [Regex::new(r"(n)ews$").unwrap(),
     Regex::new(r"(\w*)(o)es$").unwrap(),
     Regex::new(r"(\w*)([ti])a$").unwrap(),
     Regex::new(r"((a)naly|(b)a|(d)iagno|(p)arenthe|(p)rogno|(s)ynop|(t)he)(sis|ses)$").unwrap(),
     Regex::new(r"(^analy)(sis|ses)$").unwrap(),
     Regex::new(r"(\w*)([^f])ves$").unwrap(),
     Regex::new(r"(\w*)(hive)s$").unwrap(),
     Regex::new(r"(\w*)(tive)s$").unwrap(),
     Regex::new(r"(\w*)([lr])ves$").unwrap(),
     Regex::new(r"(\w*)([^aeiouy]|qu)ies$").unwrap(),
     Regex::new(r"(s)eries$").unwrap(),
     Regex::new(r"(m)ovies$").unwrap(),
     Regex::new(r"(\w*)(x|ch|ss|sh)es$").unwrap(),
     Regex::new(r"(m|l)ice$").unwrap(),
     Regex::new(r"(bus)(es)?$").unwrap(),
     Regex::new(r"(shoe)s$").unwrap(),
     Regex::new(r"(cris|test)(is|es)$").unwrap(),
     Regex::new(r"^(a)x[ie]s$").unwrap(),
     Regex::new(r"(octop|vir)(us|i)$").unwrap(),
     Regex::new(r"(alias|status)(es)?$").unwrap(),
     Regex::new(r"^(ox)en").unwrap(),
     Regex::new(r"(vert|ind)ices$").unwrap(),
     Regex::new(r"(matr)ices$").unwrap(),
     Regex::new(r"(quiz)zes$").unwrap(),
     Regex::new(r"(database)s$").unwrap(),
     Regex::new(r"(\w*)s$").unwrap(),
     Regex::new(r"(\w*)(ss)$").unwrap()]

}

#[test]
fn singularize_string_if_a_regex_will_match() {
    let expected_string: String = "oxen".to_string();
    let asserted_string: Option<String> = singularize_string(expected_string);

    assert!(asserted_string.is_some());

}

#[test]
fn singularize_string_returns_none_option_if_no_match() {
    let expected_string: String = "bacon".to_string();
    let asserted_string: Option<String> = singularize_string(expected_string);

    assert!(asserted_string.is_none());

}

#[test]
fn singularize_string_from_capture_groups_if_match_should_return_a_string_if_a_match() {
    let expected_string: String = "oxen".to_string();
    let singular_index: usize = 20;
    let asserted_string: Option<String> =
        singularize_string_from_capture_groups_if_match(expected_string, singular_index);

    assert!(asserted_string.is_some());

}

#[test]
fn singularize_string_from_capture_groups_if_match_should_return_an_option_none_if_no_match() {
    let expected_string: String = "bacon".to_string();
    let singular_index: usize = 20;
    let asserted_string: Option<String> =
        singularize_string_from_capture_groups_if_match(expected_string, singular_index);

    assert!(asserted_string.is_none());

}

#[test]
fn singularize_string_from_capture_groups_should_return_a_singularized_string() {
    let expected_string: String = "ox".to_string();
    let test_string: String = "oxen".to_string();
    let singular_regexes: [Regex; 27] = singular_rules_regexes();
    let singular_index: usize = 20;
    let cap: Captures = singular_regexes[singular_index]
        .captures(&test_string)
        .unwrap();
    let asserted_string: String = singularize_string_from_capture_groups(cap, singular_index);
    assert!(expected_string == asserted_string);

}

#[test]
fn singular_rules_regexes_should_return_27_regex() {
    let regexes: [Regex; 27] = singular_rules_regexes();
    assert!(regexes.len() == 27);
}
