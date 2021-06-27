# hex_color

A Rust library for parsing, serializing, and operating on hex colors.

[![Build Status]][actions]
[![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/workflow/status/seancroach/hex_color/ci?logo=github
[actions]: https://github.com/seancroach/hex_color/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/hex_color?logo=rust
[crates.io]: https://crates.io/crates/hex_color

### Documentation

[Module documentation with examples](https://docs.rs/hex_color). The module documentation also
includes a comprehensive description of the syntax supported for parsing hex colors.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hex_color = "1.0"
```

Here's a simple example that matches a hex color and prints the value of the red, green, and blue
components:

```rust
use hex_color::HexColor;

fn main() {
    let s = "#789ABC";
    let color: HexColor = s.parse();
    println!("rgb({}, {}, {})", color.r, color.g, color.b);
}
```

### License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
