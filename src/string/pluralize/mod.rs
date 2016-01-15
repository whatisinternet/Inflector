use regex::Regex;

const PLURAL_RULES: [&'static str; 20] = [
    "en",
    "s",
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
    "zes"
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


/// Converts a `String` to pluralized `String`
///
/// #Examples
/// ```
/// use inflector::string::pluralize::to_plural;
///
/// #[test] #[allow(non_snake_case)]
/// fn pluralize_foo_bar_to_foo_bars() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "foo_bars".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
/// #[test] #[allow(non_snake_case)]
/// fn pluralizes_ox_to_oxen() {
///     let mock_string: String = "ox".to_string();
///     let expected_string: String = "oxen".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
/// #[test] #[allow(non_snake_case)]
/// fn pluralizes_crate_to_crates() {
///     let mock_string: String = "crate".to_string();
///     let expected_string: String = "crates".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
/// #[test] #[allow(non_snake_case)]
/// fn pluralizes_boxes_to_boxes() {
///     let mock_string: String = "boxes".to_string();
///     let expected_string: String = "boxes".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
/// #[test] #[allow(non_snake_case)]
/// fn does_not_pluralize_vengence() {
///     let mock_string: String = "vengence".to_string();
///     let expected_string: String = "vengence".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
/// ```
/// use inflector::string::pluralize::to_plural;
///
/// #[test] #[allow(non_snake_case)]
/// fn does_not_pluralize_yoga() {
///     let mock_string: String = "yoga".to_string();
///     let expected_string: String = "yoga".to_string();
///     let asserted_string: String = to_plural(mock_string);
///     assert!(asserted_string == expected_string);
/// }
/// ```
///
pub fn to_plural<'a>(non_plural_string: String) -> String {
    if UNACCONTABLE_WORDS.contains(&non_plural_string.as_ref()) {
        return non_plural_string;
    } else {
        let plural_regexes: [Regex; 20] = plural_rules_regexes();
        for plural_index in 0..plural_regexes.len() {
            if plural_regexes[plural_index].is_match(&non_plural_string.as_ref()) {
                return format!("{}{}", non_plural_string, PLURAL_RULES[plural_index]);
            }
        }

        return format!("{}s", non_plural_string);
    }
}
    fn plural_rules_regexes<'a>() -> [Regex; 20] {
        return [
            Regex::new(r"^(ox)$").unwrap(),
            Regex::new(r"s$").unwrap(),
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
            Regex::new(r"(quiz)$").unwrap()
        ];

    }
