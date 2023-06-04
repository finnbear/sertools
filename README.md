# sertools

Utilities for `serde`:

- `deserializer.deserialize_t(TypedVisitor::<T>::default())` makes visiting common types easy
- `#[serde(skip_serializing_if = "sertools::is_default")]` allows avoiding serialization of default values
- `struct Foo(T); sertools::transparent!(Foo);` derives `Serialize` and `Deserialize` using the inner type

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.