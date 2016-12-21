#![deny(warnings)]
extern crate inflector;

use inflector::Inflector;

macro_rules! str_tests {
    ( $($test_name:ident => $imp_trait:ident => $to_cast:expr => $casted:expr), *) => {
        $(
            #[test]
            fn $test_name() {
                assert_eq!($to_cast.$imp_trait(), $casted)
            }
        )*
    }
}

macro_rules! string_tests {
    ( $($test_name:ident => $imp_trait:ident => $to_cast:expr => $casted:expr), *) => {
        $(
            #[test]
            fn $test_name() {
                assert_eq!($to_cast.to_string().$imp_trait(), $casted)
            }
        )*
    }
}

macro_rules! number_tests {
    ( $($test_name:ident => $imp_trait:ident => $typ:ident => $to_cast:expr => $casted:expr), *) => {
        $(
            #[test]
            fn $test_name() {
                let to_cast: $typ = $to_cast;
                assert_eq!(to_cast.to_string().$imp_trait(), $casted)
            }
        )*
    }
}

macro_rules! benchmarks {
    ( $($test_name:ident => $imp_trait:ident => $to_cast:expr), *) => {
        $(
            #[bench]
            fn $test_name(b: &mut Bencher) {
                b.iter(|| {
                    $to_cast.$imp_trait()
                });
            }
        )*
    }
}


#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    benchmarks![
        benchmark_str_to_class => to_class_case => "foo",
        benchmark_str_is_class => is_class_case => "Foo",
        benchmark_str_to_camel => to_camel_case => "foo_bar",
        benchmark_str_is_camel => is_camel_case => "fooBar",
        benchmark_str_to_table => to_table_case => "fooBar",
        benchmark_str_is_table => is_table_case => "foo_bars",
        benchmark_str_to_screaming_snake => to_screaming_snake_case => "fooBar",
        benchmark_str_is_screaming_snake => is_screaming_snake_case => "FOO_BAR",
        benchmark_str_to_snake => to_snake_case => "fooBar",
        benchmark_str_is_snake => is_snake_case => "foo_bar",
        benchmark_str_to_kebab => to_kebab_case => "fooBar",
        benchmark_str_is_kebab => is_kebab_case => "foo-bar",
        benchmark_str_to_train => to_train_case => "fooBar",
        benchmark_str_is_train => is_train_case => "Foo-Bar",
        benchmark_str_to_sentence => to_sentence_case => "fooBar",
        benchmark_str_is_sentence => is_sentence_case => "Foo bar",
        benchmark_str_to_title => to_title_case => "fooBar",
        benchmark_str_is_title => is_title_case => "Foo Bar",
        benchmark_str_ordinalize  => ordinalize => "1",
        benchmark_str_deordinalize  => deordinalize => "1st",
        benchmark_str_to_foreign_key => to_foreign_key => "Foo::Bar",
        benchmark_str_is_foreign_key => is_foreign_key => "bar_id",
        benchmark_str_demodulize => demodulize => "Foo::Bar",
        benchmark_str_deconstantize => deconstantize => "Foo::Bar",
        benchmark_str_pluralize => to_plural => "crate",
        benchmark_str_singular => to_singular => "crates",
    ];
}

str_tests![
    str_to_class => to_class_case => "foo" => "Foo".to_string(),
    str_is_class => is_class_case => "Foo" => true,
    str_to_camel => to_camel_case => "foo_bar" => "fooBar".to_string(),
    str_is_camel => is_camel_case => "fooBar" => true,
    str_to_table => to_table_case => "fooBar" => "foo_bars".to_string(),
    str_is_table => is_table_case => "foo_bars" => true,
    str_to_screaming_snake => to_screaming_snake_case => "fooBar" => "FOO_BAR".to_string(),
    str_is_screaming_snake => is_screaming_snake_case => "FOO_BAR" => true,
    str_to_snake => to_snake_case => "fooBar" => "foo_bar".to_string(),
    str_is_snake => is_snake_case => "foo_bar" => true,
    str_to_kebab => to_kebab_case => "fooBar" => "foo-bar".to_string(),
    str_is_kebab => is_kebab_case => "foo-bar" => true,
    str_to_train => to_train_case => "fooBar" => "Foo-Bar".to_string(),
    str_is_train => is_train_case => "Foo-Bar" => true,
    str_to_sentence => to_sentence_case => "fooBar" => "Foo bar".to_string(),
    str_is_sentence => is_sentence_case => "Foo bar" => true,
    str_to_title => to_title_case => "fooBar" => "Foo Bar".to_string(),
    str_is_title => is_title_case => "Foo Bar" => true,
    str_ordinalize  => ordinalize => "1" => "1st".to_string(),
    str_deordinalize  => deordinalize => "1st" => "1".to_string(),
    str_to_foreign_key => to_foreign_key => "Foo::Bar" => "bar_id".to_string(),
    str_is_foreign_key => is_foreign_key => "bar_id" => true,
    str_demodulize => demodulize => "Foo::Bar" => "Bar".to_string(),
    str_deconstantize => deconstantize => "Foo::Bar" => "Foo".to_string(),
    str_pluralize => to_plural => "crate" => "crates".to_string(),
    str_singular => to_singular => "crates" => "crate".to_string()
];

string_tests![
    string_to_class => to_class_case => "foo" => "Foo".to_string(),
    string_is_class => is_class_case => "Foo" => true,
    string_to_camel => to_camel_case => "foo_bar" => "fooBar".to_string(),
    string_is_camel => is_camel_case => "fooBar" => true,
    string_to_table => to_table_case => "fooBar" => "foo_bars".to_string(),
    string_is_table => is_table_case => "foo_bars" => true,
    string_to_screaming_snake => to_screaming_snake_case => "fooBar" => "FOO_BAR".to_string(),
    string_is_screaming_snake => is_screaming_snake_case => "FOO_BAR" => true,
    string_to_snake => to_snake_case => "fooBar" => "foo_bar".to_string(),
    string_is_snake => is_snake_case => "foo_bar" => true,
    string_to_kebab => to_kebab_case => "fooBar" => "foo-bar".to_string(),
    string_is_kebab => is_kebab_case => "foo-bar" => true,
    string_to_train => to_train_case => "fooBar" => "Foo-Bar".to_string(),
    string_is_train => is_train_case => "Foo-Bar" => true,
    string_to_sentence => to_sentence_case => "fooBar" => "Foo bar".to_string(),
    string_is_sentence => is_sentence_case => "Foo bar" => true,
    string_to_title => to_title_case => "fooBar" => "Foo Bar".to_string(),
    string_is_title => is_title_case => "Foo Bar" => true,
    string_ordinalize  => ordinalize => "1" => "1st".to_string(),
    string_deordinalize  => deordinalize => "1st" => "1".to_string(),
    string_to_foreign_key => to_foreign_key => "Foo::Bar" => "bar_id".to_string(),
    string_is_foreign_key => is_foreign_key => "bar_id" => true,
    string_demodulize => demodulize => "Foo::Bar" => "Bar".to_string(),
    string_deconstantize => deconstantize => "Foo::Bar" => "Foo".to_string(),
    string_pluralize => to_plural => "crate" => "crates".to_string(),
    string_singular => to_singular => "crates" => "crate".to_string()
];

number_tests![
    i8_ordinalize   => ordinalize => i8  => 1 => "1st".to_string(),
    i16_ordinalize  => ordinalize => i16 => 1 => "1st".to_string(),
    i32_ordinalize  => ordinalize => i32 => 1 => "1st".to_string(),
    i64_ordinalize  => ordinalize => i64 => 1 => "1st".to_string(),
    u8_ordinalize   => ordinalize => u8  => 1 => "1st".to_string(),
    u16_ordinalize  => ordinalize => u16 => 1 => "1st".to_string(),
    u32_ordinalize  => ordinalize => u32 => 1 => "1st".to_string(),
    u64_ordinalize  => ordinalize => u64 => 1 => "1st".to_string(),
    isize_ordinalize  => ordinalize => isize => 1 => "1st".to_string(),
    usize_ordinalize  => ordinalize => usize => 1 => "1st".to_string(),
    f32_ordinalize  => ordinalize => f32 => 1.0 => "1st".to_string(),
    f64_ordinalize  => ordinalize => f64 => 1.0 => "1st".to_string()
];
