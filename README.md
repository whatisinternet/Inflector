# Rust Inflector

[![Build
Status](https://travis-ci.org/whatisinternet/inflector.svg)](https://travis-ci.org/whatisinternet/inflector)

Provides ActiveSupport style inflection for Rust. Still very much a work in
progress.

Unlike ActiveSupport in Rails: Any supported case will be able to transition into any other supported case. For example:

```rust
...
let class_cased: String = to_upper_case(to_lower_case(to_class_case(to_camel_case(to_snake_case(to_kebab_case("SomeString")))))); //-> "SOMESTRING"
...
```

-----
## TODO:

- [x] Snake case
- [x] Kebab case
- [x] Camel case
- [x] Class case
- [x] Sentence case
- [x] Upper case
- [x] Lower case
- [] Table case
- [] Pluralize
- [] Singularize
- [] Ordinalize and reverse
- [] Constantize and reverse

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

// use cases::sentencecase::to_sentence_case;
// use cases::sentencecase::is_sentence_case;

// use cases::uppercase::to_upper_case;
// use cases::uppercase::is_upper_case;

// use cases::lowercase::to_lower_case;
// use cases::lowercase::is_lower_case;

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
to_class_case(String) -> String
```

```rust
to_camel_case(String) -> String
```

```rust
to_snake_case(String) -> String
```

```rust
to_kebab_case(String) -> String
```

```rust
to_sentence_case(String) -> String
```

```rust
to_upper_case(String) -> String
```

```rust
to_lower_case(String) -> String
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

```rust
is_sentence_case(String) -> bool
```

```rust
is_upper_case(String) -> bool
```

```rust
is_lower_case(String) -> bool
```

## Contributing

1. Fork it ( https://github.com/whatisinternet/inflector/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request
