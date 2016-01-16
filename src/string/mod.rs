/// Provides demodulize a string.
///
/// Example string "Foo::Bar" becomes "Bar"
pub mod demodulize;
/// Provides deconstantizea string.
///
/// Example string "Foo::Bar" becomes "Foo"
pub mod deconstantize;
/// Provides conversion to plural strings.
///
/// Example string "FooBar" -> "FooBars"
pub mod pluralize;
/// Provides conversion to singular strings.
///
/// Example string "FooBars" -> "FooBar"
pub mod singularize;

mod constants;
