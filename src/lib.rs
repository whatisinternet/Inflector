extern crate regex;

pub mod cases;
pub mod numbers;
pub mod suffix;

use cases::classcase::to_class_case;
use cases::classcase::is_class_case;

use cases::camelcase::to_camel_case;
use cases::camelcase::is_camel_case;

use cases::snakecase::to_snake_case;
use cases::snakecase::is_snake_case;

use cases::kebabcase::to_kebab_case;
use cases::kebabcase::is_kebab_case;

use cases::sentencecase::to_sentence_case;
use cases::sentencecase::is_sentence_case;

use cases::titlecase::to_title_case;
use cases::titlecase::is_title_case;

use cases::uppercase::to_upper_case;
use cases::uppercase::is_upper_case;

use cases::lowercase::to_lower_case;
use cases::lowercase::is_lower_case;

use numbers::ordinalize::ordinalize;
use numbers::deordinalize::deordinalize;

use suffix::foreignkey::to_foreign_key;
use suffix::foreignkey::is_foreign_key;


pub trait Inflector {
    fn to_class_case<'a>(&self) -> String;
    fn is_class_case<'a>(&self) -> bool;

    fn to_camel_case<'a>(&self) -> String;
    fn is_camel_case<'a>(&self) -> bool;

    fn to_snake_case<'a>(&self) -> String;
    fn is_snake_case<'a>(&self) -> bool;

    fn to_kebab_case<'a>(&self) -> String;
    fn is_kebab_case<'a>(&self) -> bool;

    fn to_sentence_case<'a>(&self) -> String;
    fn is_sentence_case<'a>(&self) -> bool;

    fn to_title_case<'a>(&self) -> String;
    fn is_title_case<'a>(&self) -> bool;

    fn to_upper_case<'a>(&self) -> String;
    fn is_upper_case<'a>(&self) -> bool;

    fn to_lower_case<'a>(&self) -> String;
    fn is_lower_case<'a>(&self) -> bool;

    fn ordinalize<'a>(&self) -> String;
    fn deordinalize<'a>(&self) -> String;

    fn to_foreign_key<'a>(&self) -> String;
    fn is_foreign_key<'a>(&self) -> bool;
}

impl<'c> Inflector for String {
    fn to_class_case(&self) -> String{
        return to_class_case(self.to_string());
    }
    fn is_class_case(&self) -> bool{
        return is_class_case(self.to_string());
    }
    fn to_camel_case(&self) -> String{
        return to_camel_case(self.to_string());
    }
    fn is_camel_case(&self) -> bool{
        return is_camel_case(self.to_string());
    }
    fn to_snake_case(&self) -> String{
        return to_snake_case(self.to_string());
    }
    fn is_snake_case(&self) -> bool{
        return is_snake_case(self.to_string());
    }
    fn to_kebab_case(&self) -> String{
        return to_kebab_case(self.to_string());
    }
    fn is_kebab_case(&self) -> bool{
        return is_kebab_case(self.to_string());
    }
    fn to_sentence_case(&self) -> String{
        return to_sentence_case(self.to_string());
    }
    fn is_sentence_case(&self) -> bool{
        return is_sentence_case(self.to_string());
    }
    fn to_title_case(&self) -> String{
        return to_title_case(self.to_string());
    }
    fn is_title_case(&self) -> bool{
        return is_title_case(self.to_string());
    }
    fn to_upper_case(&self) -> String{
        return to_upper_case(self.to_string());
    }
    fn is_upper_case(&self) -> bool{
        return is_upper_case(self.to_string());
    }
    fn to_lower_case(&self) -> String{
        return to_lower_case(self.to_string());
    }
    fn is_lower_case(&self) -> bool{
        return is_lower_case(self.to_string());
    }
    fn ordinalize(&self) -> String{
        return ordinalize(self.to_string());
    }
    fn deordinalize(&self) -> String{
        return deordinalize(self.to_string());
    }
    fn to_foreign_key(&self) -> String{
        return to_foreign_key(self.to_string());
    }
    fn is_foreign_key(&self) -> bool{
        return is_foreign_key(self.to_string());
    }
}

impl<'c> Inflector for str {
    fn to_class_case(&self) -> String{
        return to_class_case(self.to_string());
    }
    fn is_class_case(&self) -> bool{
        return is_class_case(self.to_string());
    }
    fn to_camel_case(&self) -> String{
        return to_camel_case(self.to_string());
    }
    fn is_camel_case(&self) -> bool{
        return is_camel_case(self.to_string());
    }
    fn to_snake_case(&self) -> String{
        return to_snake_case(self.to_string());
    }
    fn is_snake_case(&self) -> bool{
        return is_snake_case(self.to_string());
    }
    fn to_kebab_case(&self) -> String{
        return to_kebab_case(self.to_string());
    }
    fn is_kebab_case(&self) -> bool{
        return is_kebab_case(self.to_string());
    }
    fn to_sentence_case(&self) -> String{
        return to_sentence_case(self.to_string());
    }
    fn is_sentence_case(&self) -> bool{
        return is_sentence_case(self.to_string());
    }
    fn to_title_case(&self) -> String{
        return to_title_case(self.to_string());
    }
    fn is_title_case(&self) -> bool{
        return is_title_case(self.to_string());
    }
    fn to_upper_case(&self) -> String{
        return to_upper_case(self.to_string());
    }
    fn is_upper_case(&self) -> bool{
        return is_upper_case(self.to_string());
    }
    fn to_lower_case(&self) -> String{
        return to_lower_case(self.to_string());
    }
    fn is_lower_case(&self) -> bool{
        return is_lower_case(self.to_string());
    }
    fn ordinalize(&self) -> String{
        return ordinalize(self.to_string());
    }
    fn deordinalize(&self) -> String{
        return deordinalize(self.to_string());
    }
    fn to_foreign_key(&self) -> String{
        return to_foreign_key(self.to_string());
    }
    fn is_foreign_key(&self) -> bool{
        return is_foreign_key(self.to_string());
    }
}

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
