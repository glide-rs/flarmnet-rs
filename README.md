flarmnet
==============================================================================

[FlarmNet] file decoder/encoder for [Rust]

[FlarmNet]: https://www.flarmnet.org/
[Rust]: https://www.rust-lang.org/


Usage
------------------------------------------------------------------------------

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("data.fln")?;
    let decoded_file = flarmnet::decode_file(&content);
    // ...
    Ok(())
}
```


Related
------------------------------------------------------------------------------

- [flarmnet-js] – Similar library for JavaScript

[flarmnet-js]: https://github.com/Turbo87/flarmnet-js


License
------------------------------------------------------------------------------

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.
