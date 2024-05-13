# errify

[<img alt="crates.io" src="https://img.shields.io/crates/v/errify?style=flat-square">](https://crates.io/crates/errify)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/errify?style=flat-square">](https://docs.rs/errify)
[<img alt="build" src="https://img.shields.io/github/actions/workflow/status/LazyMechanic/errify/ci.yml?branch=master&style=flat-square">](https://github.com/LazyMechanic/errify/actions)

This library provides the macros that provide error context for the entire function via [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) and [`eyre`](https://docs.rs/eyre/latest/eyre/) crates.

```toml
[dependencies]
errify = "0.2"
```

## Usage example
```rust
use errify::errify;

#[errify("Custom error context, with argument capturing {arg} = {}", arg)]
fn func(arg: i32) -> Result<(), CustomError> {
    // ...
}
```

For more information, see the [documentation](https://docs.rs/errify)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>