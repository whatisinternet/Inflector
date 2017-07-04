#![cfg(test)]
#[macro_export]
macro_rules! define_test_group {
    ($module_name: ident,
    $method: ident,
    $is_method: ident,
    $use_mod: ident,
    $expected: expr,
    $expected_plural: expr) => {
        #[cfg(test)]
        mod $module_name {
            use ::cases::$use_mod::$method;
            use ::cases::$use_mod::$is_method;
            define_tests![
                $method;
                from_camel_case             => "fooBar"     => $expected,
                from_screaming_snake_case   => "FOO_BAR"    => $expected,
                from_kebab_case             => "foo-bar"    => $expected,
                from_pascal_case            => "FooBar"     => $expected,
                from_sentence_case          => "Foo bar"    => $expected,
                from_snake_case             => "foo_bar"    => $expected,
                from_title_case             => "Foo Bar"    => $expected,
                from_train_case             => "Foo-Bars"   => $expected_plural
            ];
            define_is_tests![
                $is_method;
                test_is => $expected
            ];
            define_is_not_tests![
                $is_method;
                test_is_not => "fOOOOBB_-Bar"
            ];
            define_gated_tests![
                $method;
                from_class_case             => "FooBar"     => $expected,
                from_table_case             => "foo_bars"   => $expected_plural
            ];
        }
    }
}

#[cfg(not(feature = "unstable"))]
macro_rules! define_gated_tests{
    ($method: ident; $($test_name:ident => $to_convert:expr => $expected:expr ), *) => {
        $(
            #[test]
            #[cfg(feature = "heavyweight")]
            fn $test_name() {
                assert_eq!($method($to_convert), $expected.to_owned())
            }
        )*
    }
}

#[cfg(not(feature = "unstable"))]
macro_rules! define_tests{
    ($method: ident; $($test_name:ident => $to_convert:expr => $expected:expr ), *) => {
        $(
            #[test]
            fn $test_name() {
                assert_eq!($method($to_convert), $expected.to_owned())
            }
        )*
    }
}

#[cfg(not(feature = "unstable"))]
macro_rules! define_is_tests{
    ($method: ident; $($test_name:ident => $expected:expr ), *) => {
        $(
            #[test]
            fn $test_name() {
                assert_eq!($method($expected), true)
            }
        )*
    }
}

#[cfg(not(feature = "unstable"))]
macro_rules! define_is_not_tests{
    ($method: ident; $($test_name:ident => $expected:expr ), *) => {
        $(
            #[test]
            fn $test_name() {
                assert_eq!($method($expected), false)
            }
        )*
    }
}
