use regex::Regex;
use string::constants::UNACCONTABLE_WORDS;
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
macro_rules! special_cases{
    ($s:ident, $($singular: expr => $plural:expr), *) => {
        match &$s[..] {
            $(
                $singular => {
                    return $plural.to_string();
                },
            )*
            _ => ()
        }
    }
}
pub fn to_singular(non_singular_string: String) -> String {
    if UNACCONTABLE_WORDS.contains(&non_singular_string.as_ref()) {
        non_singular_string
    } else {
        special_cases![non_singular_string,
            "oxen" => "ox",
            "men" => "man",
            "women" => "woman",
            "dice" => "die",
            "yeses" => "yes",
            "feet" => "foot",
            "eaves" => "eave",
            "geese" => "goose",
            "teeth" => "tooth",
            "quizzes" => "quiz"
        ];
        for &(ref rule, replace) in RULES.iter().rev() {
            if let Some(c) = rule.captures(&non_singular_string) {
                if let Some(c) = c.get(1) {
                    return format!("{}{}", c.as_str(), replace);
                }
            }
        }

        format!("{}", non_singular_string)
    }
}


macro_rules! add_rule{
    ($r:ident, $rule:expr => $replace:expr) => {
        $r.push((Regex::new($rule).unwrap(), $replace));
    }
}

macro_rules! rules{
    ($r:ident; $($rule:expr => $replace:expr), *) => {
        $(
            add_rule!{$r, $rule => $replace}
        )*
    }
}


lazy_static!{
    static ref RULES: Vec<(Regex, &'static str)> = {
    let mut r = Vec::new();
    rules![r;
     r"(n)ews$" => "ews",
     r"(\w*)(o)es$" => "",
     r"(\w*)([ti])a$" => "um",
     r"((a)naly|(b)a|(d)iagno|(p)arenthe|(p)rogno|(s)ynop|(t)he)(sis|ses)$" => "sis",
     r"(^analy)(sis|ses)$" => "sis",
     r"(\w*)([^f])ves$" => "fe",
     r"(\w*)(hive)s$" => "",
     r"(\w*)(tive)s$" => "",
     r"(\w*)([lr])ves$" => "f",
     r"(\w*)([^aeiouy]|qu)ies$" => "y",
     r"(s)eries$" => "eries",
     r"(m)ovies$" => "ovie",
     r"(\w*)(x|ch|ss|sh)es$" => "",
     r"(m|l)ice$" => "ouse",
     r"(bus)(es)?$" => "",
     r"(shoe)s$" => "",
     r"(cris|test)(is|es)$" => "is",
     r"^(a)x[ie]s$" => "xis",
     r"(octop|vir)(us|i)$" => "us",
     r"(alias|status)(es)?$" => "",
     r"^(ox)en" => "",
     r"(vert|ind)ices$" => "ex",
     r"(matr)ices$" => "ix",
     r"(quiz)zes$" => "",
     r"(database)s$" => "",
     r"(\w*)s$" => "",
     r"(\w*)(ss)$" => ""
         ];
     r
    };
}

#[test]
fn singularize_string_if_a_regex_will_match() {
    let expected_string: String = "ox".to_string();
    let asserted_string: String = to_singular("oxen".to_string());
    assert!(expected_string == asserted_string);

}

#[test]
fn singularize_string_returns_none_option_if_no_match() {
    let expected_string: String = "bacon".to_string();
    let asserted_string: String = to_singular("bacon".to_string());

    assert!(expected_string == asserted_string);

}
