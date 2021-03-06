# Fire-rs

[![](https://img.shields.io/github/workflow/status/fun4wut/fire-rs/Test%20and%20Publish)](https://github.com/fun4wut/fire-rs/actions)
[![](https://img.shields.io/badge/crates.io-v0.2.2--alpha.0-orange.svg?longCache=true)](https://crates.io/crates/fire-rs)
[![](https://docs.rs/fire-rs/badge.svg?version=0.2.2-alpha.0)](https://docs.rs/fire-rs)

受到 [Python-fire](<https://github.com/google/python-fire>) 的启发，写了Rust版本。

利用过程宏，在编译期修改 `AST` 来达成。

几个重要的package：

1. syn：用于解析AST
2. quote：将元素转化为 `TokenStream`

## Usage

```rust
use fire_rs::fire;

#[fire]
fn hello(a: i64, b: i64, c: String) {
    println!("{} is {}", a + b, c)
}

fn main() {
    hello_fire();// 利用宏生成出的新函数
}

```

命令行输入 `cargo run -- 2 4 cool` 或 `cargo run -- --a 2 --b 4 --c cool`

输出 `6 is cool`



## TODO

- [x] 支持普通函数
- [x] 支持命名参数
- [x] 文档
- [x] 测试
- [x] 发布至`crates.io`
- [x] CI/CD
- [ ] &str用作参数
- [ ] 取消`unused`警告
- [ ] 支持默认参数
- [ ] 多函数
- [ ] 错误处理