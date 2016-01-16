use regex::Regex;
use regex::{Captures};
use string::constants::UNACCONTABLE_WORDS;

const SINGULAR_RULES: [&'static str; 27] = [
    "ews",
    "",
    "um",
    "sis",
    "sis",
    "fe",
    "",
    "",
    "f",
    "y",
    "eries",
    "ovie",
    "",
    "ouse",
    "",
    "",
    "is",
    "xis",
    "us",
    "",
    "",
    "ex",
    "ix",
    "",
    "",
    "",
    ""
];


/// Converts a `String` to singularized `String`
///
/// #Examples
/// ```
/// use inflector::string::singularize::to_singular;
///
///
/// // singularize_foo_bars_to_foo_bar() {
///     let mock_string: String = "foo_bars".to_string();
///     let expected_string: String = "foo_bar".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::singularize::to_singular;
///
///
/// // singularizes_oxen_to_ox() {
///     let mock_string: String = "oxen".to_string();
///     let expected_string: String = "ox".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::singularize::to_singular;
///
///
/// // singularizes_crates_to_crate() {
///     let mock_string: String = "crates".to_string();
///     let expected_string: String = "crate".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::singularize::to_singular;
///
///
/// // singularizes_boxes_to_box() {
///     let mock_string: String = "boxes".to_string();
///     let expected_string: String = "box".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::singularize::to_singular;
///
///
/// // does_not_singularize_vengeance {
///     let mock_string: String = "vengeance".to_string();
///     let expected_string: String = "vengeance".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::singularize::to_singular;
///
///
/// // does_not_singularize_yoga() {
///     let mock_string: String = "yoga".to_string();
///     let expected_string: String = "yoga".to_string();
///     let asserted_string: String = to_singular(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
//TODO: Refactor
pub fn to_singular<'a>(non_singular_string: String) -> String {
    if UNACCONTABLE_WORDS.contains(&non_singular_string.as_ref()) {
        return non_singular_string;
    } else {
        let singular_regexes: [Regex; 27] = singular_rules_regexes();
        for singular_index in 0..singular_regexes.len() {
            if singular_regexes[singular_index].is_match(&non_singular_string.as_ref()) {
                let cap = singular_regexes[singular_index].captures(&non_singular_string.as_ref()).unwrap();
                if cap.len() > 2 {
                    return format!("{}{}{}", cap.at(1).unwrap_or(""), cap.at(2).unwrap_or(""), SINGULAR_RULES[singular_index]);
                } else {
                    return format!("{}{}", cap.at(1).unwrap_or(""), SINGULAR_RULES[singular_index]);
                }
            }
        }

        return non_singular_string;
    }
}
    fn singular_rules_regexes<'a>() -> [Regex; 27] {
        return [
            Regex::new(r"(n)ews$").unwrap(),
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
            Regex::new(r"(\w*)(ss)$").unwrap()
        ];

    }
