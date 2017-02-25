#![deny(warnings)]
/// Provides demodulize a string.
///
/// Example string `Foo::Bar` becomes `Bar`
#[cfg(not(feature = "without_full"))]
pub mod demodulize;
/// Provides deconstantizea string.
///
/// Example string `Foo::Bar` becomes `Foo`
#[cfg(not(feature = "without_full"))]
pub mod deconstantize;
/// Provides conversion to plural strings.
///
/// Example string `FooBar` -> `FooBars`
#[cfg(not(feature = "without_full"))]
pub mod pluralize;
/// Provides conversion to singular strings.
///
/// Example string `FooBars` -> `FooBar`
#[cfg(not(feature = "without_full"))]
pub mod singularize;

mod constants;
