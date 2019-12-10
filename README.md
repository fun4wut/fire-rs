# Fire-rs

[![](https://img.shields.io/github/workflow/status/fun4wut/fire-rs/Test%20and%20Publish)](https://github.com/fun4wut/fire-rs/actions)
[![](https://img.shields.io/badge/crates.io-v0.2.2--alpha.0-orange.svg?longCache=true)](https://crates.io/crates/fire-rs)
[![](https://docs.rs/fire-rs/badge.svg?version=0.2.2-alpha.0)](https://docs.rs/fire-rs)



Use the attribute macro to convert a ordinary function into a command line application (aka `cli`).

Inspired by [Python-fire](<https://github.com/google/python-fire>)

## Usage
```rust
use fire_rs::fire;
#[fire]
fn foo(a: i32, b: f64, c: String) {
    println!("{} is {}", a as f64 + b, c);
}
fn main() {
   foo_fire();
}
```
Run `cargo run -- 1 2.1 cool` or `cargo run -- a 1 --b 2.1 --c cool`,
the program will output `3.1 is cool`

## TODO

- [x] Normal args
- [x] Named args
- [x] Doc
- [x] Test
- [x] Publish to `crates.io`
- [x] CI/CD
- [ ] `&str` problem
- [ ] `unused_` warning
- [ ] Default args
- [ ] Multiple functions
- [ ] Deal with errors
