# Simple Serde
Simple serde is as its said, a simplified implementation of multiple repositories for
serialization and deserialization.

In Short the goal is to have a single tool for serialization and deserialization, with a common
interface.

# Usage
Simple Serde uses `.encode` and `.decode` for encoding and decoding. Decode can be done on any
`Vec<u8>` or `&[u8]` this allows for the cleanest implementation.
The same goes for anything that needs to be serialized/encoded. Any type that implements the
`#[derive(Serialize)]` can easily be encoded using `.encode`

## Encode/Decode
`.encode` and `.decode` both takes a `ContentType` which defines what you are encoding/decoding
from/to.
an example would be `[some Vec<u8>].decode("bson")` or `my_struct.encode("bson")`.
This is possible as `ContentType` implements the `TryFrom` trait for `&str`, `String`.  
In case the implementation is unable to decode what type you are trying to encode/decode from/to
an `Err` result with `Error::UnknownContentTypeMatchFromStr` will be returned from the
encoder/decoder

Anything coming out of the encoder will be of type `Vec<u8>` further the `Vec<u8>` is wrapped in
a struct called `Encoded` this allow for further simplifications on implementation like,
`TryToString` which will automatically try to convert `Encoded` to a `String`, in addition
`Encoded` had implemented the `Deref` and `DerefMut` traits to make it easier to gain access to
encapsulated data.

## Supported formats
- Bson
- Cbor
- FlexBuffers
- Json
- Json5
- Lexpr
- MessagePack
- Pickle
- Postcard
- Ron
- Toml
- Url
- Yaml
- Xml

further all string definitions of `ContentType` is case insensitive, and has an alternate
- `application/[format]`
- `application/x-[format]`

## Serialization/Encode example
```rust
use std::ops::Deref;
use serde::Serialize;
#[macro_use]
use serde_derive;
use simple_serde::{Encoded, SimpleEncoder, TryToString};

#[derive(Serialize)]
struct Foo {
    bar: String,
}

let my_foo = Foo {
  bar: "foobar".to_string(),
};

let encoded: Encoded = my_foo
  .encode("yaml")
  .expect("Should have been encoded in yaml");

assert_eq!(
    &vec![45, 45, 45, 10, 98, 97, 114, 58, 32, 102, 111, 111, 98, 97, 114, 10],
    encoded.deref()
);
assert_eq!(r#"---
bar: foobar
"#, encoded.try_to_string().unwrap())
```
## Deserialization/Decode example
```rust
use std::ops::Deref;
use serde::Deserialize;
#[macro_use]
use serde_derive;
use simple_serde::{Decoded, SimpleDecoder};

#[derive(Deserialize, Debug, PartialEq)]
struct Foo {
    bar: String,
}

let my_foo = Foo {
  bar: "foobar".to_string(),
};

let v_u8_data = &vec![45, 45, 45, 10, 98, 97, 114, 58, 32, 102, 111, 111, 98, 97, 114, 10];
let string_data = r#"---
bar: foobar
"#;

let decoded_from_v_u8: Decoded<Foo> = v_u8_data.decode("yaml").expect("Should have decoded the Vec<u8>");
let decoded_from_string: Decoded<Foo> = string_data.decode("yaml").expect("Should have decoded the String");

assert_eq!(
    Foo{bar: "foobar".to_string()},
    decoded_from_v_u8.into()
);
assert_eq!(
    Foo{bar: "foobar".to_string()},
    decoded_from_string.into()
);
```
# Contribute
Any merge requests are welcomed!

# License
MIT