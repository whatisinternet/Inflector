# Rust Inflector

[![Build
Status](https://travis-ci.org/whatisinternet/inflector.svg)](https://travis-ci.org/whatisinternet/inflector)

Provides ActiveSupport style inflection for Rust. Still very much a work in
progress.

-----

## Installation:

### Compile yourself:

1. Install [Rust and cargo](http://doc.crates.io/)
2. git clone git@github.com:whatisinternet/inflector.git
3. Library: cd inflector && cargo build --release --lib
4. You can find the library in target/release

## Usage:

```rust
...
extern crate inflector;

use inflector::*;
...
fn main() {
...
  let camel_case_string: String = to_camel_case("some_string".to_string());
...
}

```

-----
# Methods:

The methods work in a very basic sense.

For example:
```rust
to_snake_case("camelCase".to_string())
// --> "camel_case"
//
// However
to_snake_case("camel-case".to_string())
// --> "camel-case"
```

```rust
to_class_case (String) -> String
```
Converts strings to ClassCase.

```rust
to_camel_case (String) -> String
```
Converts strings to camelCase.

```rust
to_snake_case (String) -> String
```
Converts strings to snake_case.

```rust
to_kebab_case (String) -> String
```
Converts strings to kebab-case.

```rust
is_class_case(String) -> bool
```

```rust
is_camel_case(String) -> bool
```

```rust
is_snake_case(String) -> bool
```

```rust
is_kebab_case(String) -> bool
```

## Contributing

1. Fork it ( https://github.com/whatisinternet/inflector/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request
