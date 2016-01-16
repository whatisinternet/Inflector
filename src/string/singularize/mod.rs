use regex::Regex;

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

const UNACCONTABLE_WORDS: [&'static str; 202] = [
    "accommodation",
    "adulthood",
    "advertising",
    "advice",
    "aggression",
    "aid",
    "air",
    "aircraft",
    "alcohol",
    "anger",
    "applause",
    "arithmetic",
    "assistance",
    "athletics",

    "bacon",
    "baggage",
    "beef",
    "biology",
    "blood",
    "botany",
    "bread",
    "butter",

    "carbon",
    "cardboard",
    "cash",
    "chalk",
    "chaos",
    "chess",
    "crossroads",
    "countryside",

    "dancing",
    "deer",
    "dignity",
    "dirt",
    "dust",

    "economics",
    "education",
    "electricity",
    "engineering",
    "enjoyment",
    "envy",
    "equipment",
    "ethics",
    "evidence",
    "evolution",

    "fame",
    "fiction",
    "flour",
    "flu",
    "food",
    "fuel",
    "fun",
    "furniture",

    "gallows",
    "garbage",
    "garlic",
    "genetics",
    "gold",
    "golf",
    "gossip",
    "grammar",
    "gratitude",
    "grief",
    "guilt",
    "gymnastics",

    "happiness",
    "hardware",
    "harm",
    "hate",
    "hatred",
    "health",
    "heat",
    "help",
    "homework",
    "honesty",
    "honey",
    "hospitality",
    "housework",
    "humour",
    "hunger",
    "hydrogen",

    "ice",
    "importance",
    "inflation",
    "information",
    "innocence",
    "iron",
    "irony",

    "jam",
    "jewelry",
    "judo",

    "karate",
    "knowledge",

    "lack",
    "laughter",
    "lava",
    "leather",
    "leisure",
    "lightning",
    "linguine",
    "linguini",
    "linguistics",
    "literature",
    "litter",
    "livestock",
    "logic",
    "loneliness",
    "luck",
    "luggage",

    "macaroni",
    "machinery",
    "magic",
    "management",
    "mankind",
    "marble",
    "mathematics",
    "mayonnaise",
    "measles",
    "methane",
    "milk",
    "money",
    "mud",
    "music",
    "mumps",

    "nature",
    "news",
    "nitrogen",
    "nonsense",
    "nurture",
    "nutrition",

    "obedience",
    "obesity",
    "oxygen",

    "pasta",
    "patience",
    "physics",
    "poetry",
    "pollution",
    "poverty",
    "pride",
    "psychology",
    "publicity",
    "punctuation",

    "quartz",

    "racism",
    "relaxation",
    "reliability",
    "research",
    "respect",
    "revenge",
    "rice",
    "rubbish",
    "rum",

    "safety",
    "scenery",
    "seafood",
    "seaside",
    "series",
    "shame",
    "sheep",
    "shopping",
    "sleep",
    "smoke",
    "smoking",
    "snow",
    "soap",
    "software",
    "soil",
    "spaghetti",
    "species",
    "steam",
    "stuff",
    "stupidity",
    "sunshine",
    "symmetry",

    "tennis",
    "thirst",
    "thunder",
    "timber",
    "traffic",
    "transportation",
    "trust",

    "underwear",
    "unemployment",
    "unity",

    "validity",
    "veal",
    "vegetation",
    "vegetarianism",
    "vengeance",
    "violence",
    "vitality",

    "warmth",
    "wealth",
    "weather",
    "welfare",
    "wheat",
    "wildlife",
    "wisdom",
    "yoga",

    "zinc",
    "zoology"
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
