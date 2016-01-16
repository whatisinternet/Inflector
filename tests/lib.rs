extern crate inflector;

use inflector::Inflector;

#[test]
fn string_trait_to_class_case() {
    assert_eq!("foo".to_string().to_class_case(), "Foo".to_string());
}

#[test]
fn string_trait_is_class_case() {
    assert_eq!("FooFoo".to_string().is_class_case(), true);
}

#[test]
fn string_trait_to_camel_case() {
    assert_eq!("fooFoo".to_string().to_camel_case(), "fooFoo".to_string());
}

#[test]
fn string_trait_is_camel_case() {
    assert_eq!("fooFoo".to_string().is_camel_case(), true);
}

#[test]
fn string_trait_to_table_case() {
    assert_eq!("fooFoo".to_string().to_table_case(), "foo_foos".to_string());
}

#[test]
fn string_trait_is_table_case() {
    assert_eq!("foo_foos".to_string().is_table_case(), true);
}

#[test]
fn string_trait_to_screaming_snake_case() {
    assert_eq!("fooFoo".to_string().to_screaming_snake_case(), "FOO_FOO".to_string());
}

#[test]
fn string_trait_is_screaming_snake_case() {
    assert_eq!("FOO_FOO".to_string().is_screaming_snake_case(), true);
}
#[test]
fn string_trait_to_snake_case() {
    assert_eq!("fooFoo".to_string().to_snake_case(), "foo_foo".to_string());
}

#[test]
fn string_trait_is_snake_case() {
    assert_eq!("foo_foo".to_string().is_snake_case(), true);
}

#[test]
fn string_trait_to_kebab_case() {
    assert_eq!("fooFoo".to_string().to_kebab_case(), "foo-foo".to_string());
}

#[test]
fn string_trait_is_kebab_case() {
    assert_eq!("foo-foo".to_string().is_kebab_case(), true);
}

#[test]
fn string_trait_to_sentence_case() {
    assert_eq!("fooFoo".to_string().to_sentence_case(), "Foo foo".to_string());
}

#[test]
fn string_trait_is_sentence_case() {
    assert_eq!("Foo foo".to_string().is_sentence_case(), true);
}

#[test]
fn string_trait_to_title_case() {
    assert_eq!("fooFoo".to_string().to_title_case(), "Foo Foo".to_string());
}

#[test]
fn string_trait_is_title_case() {
    assert_eq!("Foo Foo".to_string().is_title_case(), true);
}

#[test]
fn string_trait_to_upper_case() {
    assert_eq!("fooFoo".to_string().to_upper_case(), "FOOFOO".to_string());
}

#[test]
fn string_trait_is_upper_case() {
    assert_eq!("FOOFOO".to_string().is_upper_case(), true);
}

#[test]
fn string_trait_to_lower_case() {
    assert_eq!("fooFoo".to_string().to_lower_case(), "foofoo".to_string());
}

#[test]
fn string_trait_is_lower_case() {
    assert_eq!("foo".to_string().is_lower_case(), true);
}

#[test]
fn string_trait_ordinalize() {
    assert_eq!("1".to_string().ordinalize(), "1st".to_string());
}

#[test]
fn string_trait_deordinalize() {
    assert_eq!("1st".to_string().deordinalize(), "1".to_string());
}

#[test]
fn string_trait_to_foreign_key() {
    assert_eq!("Foo::Bar".to_string().to_foreign_key(), "bar_id".to_string());
}

#[test]
fn string_trait_demodulize() {
    assert_eq!("Foo::Bar".to_string().demodulize(), "Bar".to_string());
}

#[test]
fn string_trait_deconstantize() {
    assert_eq!("Foo::Bar".to_string().deconstantize(), "Foo".to_string());
}

#[test]
fn string_trait_to_plural() {
    assert_eq!("crate".to_string().to_plural(), "crates".to_string());
}

#[test]
fn string_trait_to_singular() {
    assert_eq!("crates".to_string().to_singular(), "crate".to_string());
}



//-----------------------------------------------------------------------------------------//



#[test]
fn str_trait_to_class_case() {
    assert_eq!("foo".to_class_case(), "Foo".to_string());
}

#[test]
fn str_trait_is_class_case() {
    assert_eq!("FooFoo".is_class_case(), true);
}

#[test]
fn str_trait_to_camel_case() {
    assert_eq!("fooFoo".to_camel_case(), "fooFoo".to_string());
}

#[test]
fn str_trait_is_camel_case() {
    assert_eq!("fooFoo".is_camel_case(), true);
}

#[test]
fn str_trait_to_table_case() {
    assert_eq!("fooFoo".to_table_case(), "foo_foos".to_string());
}

#[test]
fn str_trait_is_table_case() {
    assert_eq!("foo_foos".is_table_case(), true);
}

#[test]
fn str_trait_to_screaming_snake_case() {
    assert_eq!("fooFoo".to_screaming_snake_case(), "FOO_FOO".to_string());
}

#[test]
fn str_trait_is_screaming_snake_case() {
    assert_eq!("FOO_FOO".is_screaming_snake_case(), true);
}
#[test]
fn str_trait_to_snake_case() {
    assert_eq!("fooFoo".to_snake_case(), "foo_foo".to_string());
}

#[test]
fn str_trait_is_snake_case() {
    assert_eq!("foo_foo".is_snake_case(), true);
}

#[test]
fn str_trait_to_kebab_case() {
    assert_eq!("fooFoo".to_kebab_case(), "foo-foo".to_string());
}

#[test]
fn str_trait_is_kebab_case() {
    assert_eq!("foo-foo".is_kebab_case(), true);
}

#[test]
fn str_trait_to_sentence_case() {
    assert_eq!("fooFoo".to_sentence_case(), "Foo foo".to_string());
}

#[test]
fn str_trait_is_sentence_case() {
    assert_eq!("Foo foo".is_sentence_case(), true);
}

#[test]
fn str_trait_to_title_case() {
    assert_eq!("fooFoo".to_title_case(), "Foo Foo".to_string());
}

#[test]
fn str_trait_is_title_case() {
    assert_eq!("Foo Foo".is_title_case(), true);
}

#[test]
fn str_trait_to_upper_case() {
    assert_eq!("fooFoo".to_upper_case(), "FOOFOO".to_string());
}

#[test]
fn str_trait_is_upper_case() {
    assert_eq!("FOOFOO".is_upper_case(), true);
}

#[test]
fn str_trait_to_lower_case() {
    assert_eq!("fooFoo".to_lower_case(), "foofoo".to_string());
}

#[test]
fn str_trait_is_lower_case() {
    assert_eq!("foo".is_lower_case(), true);
}

#[test]
fn str_trait_ordinalize() {
    assert_eq!("1".ordinalize(), "1st".to_string());
}

#[test]
fn str_trait_deordinalize() {
    assert_eq!("1st".deordinalize(), "1".to_string());
}

#[test]
fn str_trait_to_foreign_key() {
    assert_eq!("Foo::Bar".to_foreign_key(), "bar_id".to_string());
}

#[test]
fn str_trait_demodulize() {
    assert_eq!("Foo::Bar".demodulize(), "Bar".to_string());
}

#[test]
fn str_trait_deconstantize() {
    assert_eq!("Foo::Bar".deconstantize(), "Foo".to_string());
}

#[test]
fn str_trait_to_plural() {
    assert_eq!("ox".to_plural(), "oxen".to_string());
}

#[test]
fn str_trait_to_singular() {
    assert_eq!("oxen".to_singular(), "ox".to_string());
}
