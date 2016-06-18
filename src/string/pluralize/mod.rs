#![cfg_attr(feature="clippy", allow(trivial_regex))]
use regex::Regex;
use string::constants::UNACCONTABLE_WORDS;

const PLURAL_RULES: [&'static str; 20] = [
    "en",
    "es",
    "i",
    "i",
    "es",
    "ses",
    "oes",
    "a",
    "a",
    "ses",
    "ves",
    "s",
    "ies",
    "es",
    "ices",
    "ice",
    "ice",
    "",
    "zes",
    ""
];


/// Converts a `String` to pluralized `String`
///
/// #Examples
/// ```
/// use inflector::string::pluralize::to_plural;
///
///
/// // pluralize_foo_bar_to_foo_bars() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo_bars".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
///
/// // pluralizes_ox_to_oxen() {
///     let mock_string: String = "ox".to_string();
///     let expected_string: String = "oxen".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
///
/// // pluralizes_crate_to_crates() {
///     let mock_string: String = "crate".to_string();
///     let expected_string: String = "crates".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
///
/// // pluralizes_boxes_to_boxes() {
///     let mock_string: String = "boxes".to_string();
///     let expected_string: String = "boxes".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
///
/// // does_not_pluralize_vengeance) {
///     let mock_string: String = "vengeance".to_string();
///     let expected_string: String = "vengeance".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
///
/// // does_not_pluralize_yoga() {
///     let mock_string: String = "yoga".to_string();
///     let expected_string: String = "yoga".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
pub fn to_plural(non_plural_string: String) -> String {
    if UNACCONTABLE_WORDS.contains(&non_plural_string.as_ref()) {
        non_plural_string
    } else {
        let plural_regexes: [Regex; 20] = plural_rules_regexes();
        for plural_index in 0..plural_regexes.len() {
            if plural_regexes[plural_index].is_match(&non_plural_string){
                return format!("{}{}", non_plural_string, PLURAL_RULES[plural_index]);
            }
        }

        format!("{}s", non_plural_string)
    }
}
    fn plural_rules_regexes() -> [Regex; 20] {
        [
            Regex::new(r"^(ox)$").unwrap(),
            Regex::new(r"^(ax|test)is$").unwrap(),
            Regex::new(r"(octop|vir)us$").unwrap(),
            Regex::new(r"(octop|vir)i$").unwrap(),
            Regex::new(r"(alias|status)$").unwrap(),
            Regex::new(r"(bu)s$").unwrap(),
            Regex::new(r"(buffal|tomat)o$").unwrap(),
            Regex::new(r"([ti])um$").unwrap(),
            Regex::new(r"([ti])a$").unwrap(),
            Regex::new(r"sis$").unwrap(),
            Regex::new(r"(?:([^f])fe|([lr])f)$").unwrap(),
            Regex::new(r"(hive)$").unwrap(),
            Regex::new(r"([^aeiouy]|qu)y$").unwrap(),
            Regex::new(r"(x|ch|ss|sh)$").unwrap(),
            Regex::new(r"(matr|vert|ind)(?:ix|ex)$").unwrap(),
            Regex::new(r"(m|l)ouse$").unwrap(),
            Regex::new(r"(m|l)ice$").unwrap(),
            Regex::new(r"^(oxen)$").unwrap(),
            Regex::new(r"(quiz)$").unwrap(),
            Regex::new(r"s$").unwrap(),
        ]

    }
