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

// use inflector::cases::classcase::to_class_case;
// use inflector::cases::classcase::is_class_case;

// use inflector::cases::camelcase::to_camel_case;
// use inflector::cases::camelcase::is_camel_case;

// use inflector::cases::snakecase::to_snake_case;
// use inflector::cases::snakecase::is_snake_case;

// use inflector::cases::kebabcase::to_kebab_case;
// use inflector::cases::kebabcase::is_kebab_case;

...
fn main() {
...
  let camel_case_string: String = to_camel_case("some_string".to_string());
...
}

```

-----
# Methods:

```rust
to_class_case (String) -> String
```

```rust
to_camel_case (String) -> String
```

```rust
to_snake_case (String) -> String
```

```rust
to_kebab_case (String) -> String
```

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
