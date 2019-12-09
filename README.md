# Fire-rs
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

- [x] normal args
- [x] named args
- [x] doc
- [x] test
- [x] publish to`crates.io`
- [x] CI/CD
- [ ] defalut args
- [ ] multiple functions
- [ ] deal with errors
