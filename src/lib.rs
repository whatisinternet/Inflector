#![deny(warnings)]
#![cfg_attr(feature = "unstable", feature(test))]

//! Adds String based inflections for Rust. Snake, kebab, train, camel,
//! sentence, class, and title cases as well as ordinalize,
//! deordinalize, demodulize, deconstantize, and foreign key are supported as
//! both traits and pure functions acting on String types.
//!
//! ```rust
//! use inflector::Inflector;
//! let camel_case_string: String = "some_string".to_camel_case();
//! let is_camel_cased: bool= camel_case_string.is_camel_case();
//! assert!(is_camel_cased == true);
//! ```
extern crate regex;
#[macro_use]
extern crate lazy_static;
/// Provides case inflections
/// - Camel case
/// - Class case
/// - Kebab case
/// - Train case
/// - Screaming snake case
/// - Table case
/// - Sentence case
/// - Snake case
/// - Pascal case
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
/// - Pluralize
/// - Singularize
pub mod string;


use cases::classcase::to_class_case;
use cases::classcase::is_class_case;

use cases::camelcase::to_camel_case;
use cases::camelcase::is_camel_case;

use cases::pascalcase::to_pascal_case;
use cases::pascalcase::is_pascal_case;

use cases::snakecase::to_snake_case;
use cases::snakecase::is_snake_case;

use cases::screamingsnakecase::to_screaming_snake_case;
use cases::screamingsnakecase::is_screaming_snake_case;

use cases::kebabcase::to_kebab_case;
use cases::kebabcase::is_kebab_case;

use cases::traincase::to_train_case;
use cases::traincase::is_train_case;

use cases::sentencecase::to_sentence_case;
use cases::sentencecase::is_sentence_case;

use cases::titlecase::to_title_case;
use cases::titlecase::is_title_case;

use cases::tablecase::to_table_case;
use cases::tablecase::is_table_case;

use numbers::ordinalize::ordinalize;
use numbers::deordinalize::deordinalize;

use suffix::foreignkey::to_foreign_key;
use suffix::foreignkey::is_foreign_key;

use string::demodulize::demodulize;
use string::deconstantize::deconstantize;

use string::pluralize::to_plural;
use string::singularize::to_singular;

pub trait Inflector {
    fn to_class_case(&self) -> String;
    fn is_class_case(&self) -> bool;

    fn to_camel_case(&self) -> String;
    fn is_camel_case(&self) -> bool;

    fn to_pascal_case(&self) -> String;
    fn is_pascal_case(&self) -> bool;

    fn to_table_case(&self) -> String;
    fn is_table_case(&self) -> bool;

    fn to_snake_case(&self) -> String;
    fn is_snake_case(&self) -> bool;

    fn to_screaming_snake_case(&self) -> String;
    fn is_screaming_snake_case(&self) -> bool;

    fn to_kebab_case(&self) -> String;
    fn is_kebab_case(&self) -> bool;

    fn to_train_case(&self) -> String;
    fn is_train_case(&self) -> bool;

    fn to_sentence_case(&self) -> String;
    fn is_sentence_case(&self) -> bool;

    fn to_title_case(&self) -> String;
    fn is_title_case(&self) -> bool;

    fn ordinalize(&self) -> String;
    fn deordinalize(&self) -> String;

    fn to_foreign_key(&self) -> String;
    fn is_foreign_key(&self) -> bool;

    fn demodulize(&self) -> String;
    fn deconstantize(&self) -> String;

    fn to_plural(&self) -> String;
    fn to_singular(&self) -> String;
}

impl<'c> Inflector for String {
    fn to_class_case(&self) -> String {
        to_class_case(self.to_string())
    }
    fn is_class_case(&self) -> bool {
        is_class_case(self.to_string())
    }
    fn to_camel_case(&self) -> String {
        to_camel_case(self.to_string())
    }
    fn is_camel_case(&self) -> bool {
        is_camel_case(self.to_string())
    }
    fn to_pascal_case(&self) -> String {
        to_pascal_case(self.to_string())
    }
    fn is_pascal_case(&self) -> bool {
        is_pascal_case(self.to_string())
    }
    fn to_table_case(&self) -> String {
        to_table_case(self.to_string())
    }
    fn is_table_case(&self) -> bool {
        is_table_case(self.to_string())
    }
    fn to_screaming_snake_case(&self) -> String {
        to_screaming_snake_case(self.to_string())
    }
    fn is_screaming_snake_case(&self) -> bool {
        is_screaming_snake_case(self.to_string())
    }
    fn to_snake_case(&self) -> String {
        to_snake_case(self.to_string())
    }
    fn is_snake_case(&self) -> bool {
        is_snake_case(self.to_string())
    }
    fn to_kebab_case(&self) -> String {
        to_kebab_case(self.to_string())
    }
    fn is_kebab_case(&self) -> bool {
        is_kebab_case(self.to_string())
    }
    fn to_train_case(&self) -> String {
        to_train_case(self.to_string())
    }
    fn is_train_case(&self) -> bool {
        is_train_case(self.to_string())
    }
    fn to_sentence_case(&self) -> String {
        to_sentence_case(self.to_string())
    }
    fn is_sentence_case(&self) -> bool {
        is_sentence_case(self.to_string())
    }
    fn to_title_case(&self) -> String {
        to_title_case(self.to_string())
    }
    fn is_title_case(&self) -> bool {
        is_title_case(self.to_string())
    }
    fn ordinalize(&self) -> String {
        ordinalize(self.to_string())
    }
    fn deordinalize(&self) -> String {
        deordinalize(self.to_string())
    }
    fn to_foreign_key(&self) -> String {
        to_foreign_key(self.to_string())
    }
    fn is_foreign_key(&self) -> bool {
        is_foreign_key(self.to_string())
    }
    fn demodulize(&self) -> String {
        demodulize(self.to_string())
    }
    fn deconstantize(&self) -> String {
        deconstantize(self.to_string())
    }
    fn to_plural(&self) -> String {
        to_plural(self.to_string())
    }
    fn to_singular(&self) -> String {
        to_singular(self.to_string())
    }
}

impl<'c> Inflector for str {
    fn to_class_case(&self) -> String {
        to_class_case(self.to_string())
    }
    fn is_class_case(&self) -> bool {
        is_class_case(self.to_string())
    }
    fn to_camel_case(&self) -> String {
        to_camel_case(self.to_string())
    }
    fn is_camel_case(&self) -> bool {
        is_camel_case(self.to_string())
    }
    fn to_pascal_case(&self) -> String {
        to_pascal_case(self.to_string())
    }
    fn is_pascal_case(&self) -> bool {
        is_pascal_case(self.to_string())
    }
    fn to_table_case(&self) -> String {
        to_table_case(self.to_string())
    }
    fn is_table_case(&self) -> bool {
        is_table_case(self.to_string())
    }
    fn to_screaming_snake_case(&self) -> String {
        to_screaming_snake_case(self.to_string())
    }
    fn is_screaming_snake_case(&self) -> bool {
        is_screaming_snake_case(self.to_string())
    }
    fn to_snake_case(&self) -> String {
        to_snake_case(self.to_string())
    }
    fn is_snake_case(&self) -> bool {
        is_snake_case(self.to_string())
    }
    fn to_kebab_case(&self) -> String {
        to_kebab_case(self.to_string())
    }
    fn is_kebab_case(&self) -> bool {
        is_kebab_case(self.to_string())
    }
    fn to_train_case(&self) -> String {
        to_train_case(self.to_string())
    }
    fn is_train_case(&self) -> bool {
        is_train_case(self.to_string())
    }
    fn to_sentence_case(&self) -> String {
        to_sentence_case(self.to_string())
    }
    fn is_sentence_case(&self) -> bool {
        is_sentence_case(self.to_string())
    }
    fn to_title_case(&self) -> String {
        to_title_case(self.to_string())
    }
    fn is_title_case(&self) -> bool {
        is_title_case(self.to_string())
    }
    fn ordinalize(&self) -> String {
        ordinalize(self.to_string())
    }
    fn deordinalize(&self) -> String {
        deordinalize(self.to_string())
    }
    fn to_foreign_key(&self) -> String {
        to_foreign_key(self.to_string())
    }
    fn is_foreign_key(&self) -> bool {
        is_foreign_key(self.to_string())
    }
    fn demodulize(&self) -> String {
        demodulize(self.to_string())
    }
    fn deconstantize(&self) -> String {
        deconstantize(self.to_string())
    }

    fn to_plural(&self) -> String {
        to_plural(self.to_string())
    }

    fn to_singular(&self) -> String {
        to_singular(self.to_string())
    }
}
