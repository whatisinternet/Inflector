#![deny(warnings)]
/// Provides demodulize a string.
///
/// Example string `Foo::Bar` becomes `Bar`
#[cfg(not(feature = "lightweight"))]
pub mod demodulize;
/// Provides deconstantizea string.
///
/// Example string `Foo::Bar` becomes `Foo`
#[cfg(not(feature = "lightweight"))]
pub mod deconstantize;
/// Provides conversion to plural strings.
///
/// Example string `FooBar` -> `FooBars`
#[cfg(not(feature = "lightweight"))]
pub mod pluralize;
/// Provides conversion to singular strings.
///
/// Example string `FooBars` -> `FooBar`
#[cfg(not(feature = "lightweight"))]
pub mod singularize;

mod constants;
