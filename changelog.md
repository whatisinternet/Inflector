# 0.3.0

## Fixes:

- Resolves issues with pluralize not always correctly pluralizing strings.
  Thanks, @weiznich!

## Breaking changes:

- Dropping support for Rust versions below stable

# 0.2.1

## Features:

- Replaced custom implementation of lower and uppercase with Rust default

## Breaking changes:

- Rust 1.2 or greater required

# 0.2.0

## Features:

- Added Pluralize
- Added Singularize
- Added Table case

## Fixes:

- Fixed doc tests to properly run as rust auto wraps doc tests in a function and
  never ran the inner function that was defined.
- Fixed documentation for kebab case
- Fixed several failed tests for various cases which were mainly typos

## Breaking changes:

- Class case now singularizes strings and verifies that strings are
  singularized. Those wishing for the old behaviour should remain on the 0.1.6
  release.


# 0.1.6

## Features:

- Added screaming snake case

# 0.1.5

## Fixes:

- Refactored tests into doc tests.


# 0.1.4

## Features:

- Significant performance improvement for casting strings between different case
  types see #13.

## Fixes:

- Fixed performance issues with string casting.
- Removed heavy reliance on Regex


# 0.1.3

## Fixes:

- Refactored code to mostly pass through snake case then be converted to lower
  the number of moving parts and reduce the complexity for adding a new casting
  as only snake case would need to be modified to convert to most other cases.

## Breaking changes:

- This release is slow in contrast to other crates


# 0.1.2

## Fixes:

- Documentation fixes
- Added uppercase
- Added lowercase
- Added foreign key
- Added sentence case
- Added title case


# 0.1.1

## Features:
- Adds support for foreign key
- Adds demodulize
- Adds deconstantize
- Adds trait based usage


# 0.1.0

## Features:

- Added support for camel case
- Added support for class case
- Added support for kebab case
- Added support for snake case
