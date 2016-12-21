# Thank you!

This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## Getting started
- If you don't have rust installed install [rustup](https://www.rustup.rs/)
- For normal development master must run on stable
  - `rustup toolchain install stable`
  - `rustup default stable`

- For development with benchmarks run nightly
  - `rustup toolchain install nightly`
  - `rustup default nightly`

## Github steps
1. Fork it ( https://github.com/whatisinternet/inflector/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

### Running the tests
- `cargo test`

### Running the benchmarks
- `cargo bench --features=unstable`

## Is this an issues?

- Please ensure you fill out an [issue](https://github.com/whatisinternet/inflector/issues)
- Be available for questions.

## Are you submitting documentation?

- Awesome!
- Has everything been run through spell check?

## Are you submitting code?

- Have you added doc tests and do they pass?
- Do all other tests pass?
- Have you added trait tests (if applicable)
- Have you filled out the pull request template?


## Adding a trait
Traits now both easy to add and easy to test. Just follow the next steps:

### Adding the trait
- `src/lib.rs`
1. Add the function signature to the Inflector trait
    ```rust
    //...
    use string::singularize::to_singular;
    //...
    pub trait Inflector { // Or InflectorNumbers
        //...
        fn your_trait(&self) -> [return_type];
        //...
    }
    ```
2. Add the function name an return type to either `implement_string_for` or
   `implement_number_for`
    ```rust
      your_trait => [return_type]
    ```
3. Add a benchmark following the current convention

### Add the trait tests
- `tests/lib.rs`
1. Add your trait following the current convention and the test will be
   automatically generated

Thank you for your help! :heart:
