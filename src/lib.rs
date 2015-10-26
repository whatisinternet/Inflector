//! [![Build Status](https://travis-ci.org/whatisinternet/inflector.svg?branch=master)](https://travis-ci.org/whatisinternet/inflector) [![Crates.io](https://img.shields.io/crates/v/inflector.svg)](https://crates.io/crates/inflector)
//!
//! Adds String based inflections for Rust. Snake, kebab, camel,
//! sentence, class, title, upper, and lower cases as well as ordinalize,
//! deordinalize, demodulize, deconstantize, and foreign key are supported as both traits and pure functions
//! acting on String types.
//!
//! ```rust
//! use inflector::Inflector;
//! #[test]
//! fn should_convert_to_and_verify_camel_cased_string() {
//!   let camel_case_string: String = "some_string".to_camel_case();
//!   let is_camel_cased: bool= camel_case_string.is_camel_case();
//!   assert!(is_camel_cased == true);
//! }
//! ```
extern crate regex;
/// Provides case inflections
/// - Camel case
/// - Class case
/// - Kebab case
/// - Lower case
/// - Sentence case
/// - Snake case
/// - Upper case
pub mod cases;
/// Provides number inflections
/// - Ordinalize
/// - Deordinalize
pub mod numbers;
/// Provides suffix inflections
/// - Foreign key
pub mod suffix;
/// Provides string inflections
/// - Deconstantize
/// - Demodulize
pub mod string;

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

use string::demodulize::demodulize;
use string::deconstantize::deconstantize;


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

    fn demodulize<'a>(&self) -> String;
    fn deconstantize<'a>(&self) -> String;
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
    fn demodulize(&self) -> String{
        return demodulize(self.to_string());
    }
    fn deconstantize(&self) -> String{
        return deconstantize(self.to_string());
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
    fn demodulize(&self) -> String{
        return demodulize(self.to_string());
    }
    fn deconstantize(&self) -> String{
        return deconstantize(self.to_string());
    }
}
