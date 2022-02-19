# jsondocckng

Experimental Next Generation Testing for Rustdoc Json Output

This may eventually replace [`src/tools/jsondocck`](https://github.com/rust-lang/rust/tree/master/src/tools/jsondocck)

The core idea is to serialize the json into a rust type, so we can write the tests
in rust instead of jsonpath.

## Usage

```shell
cargo test
```

or

```shell
cargo nextest run
``` 

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
