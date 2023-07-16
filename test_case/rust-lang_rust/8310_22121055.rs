 rust
// src/libextra/encoding/base64.rs
struct Base64;
impl encoding::Text for Base64 { ... }

// src/libextra/encoding/hex.rs
struct Hex;
impl encoding::Text for Hex { ... }

// src/libextra/encoding/mod.rs

impl SomeHelperTrait for &'self str {
  fn decode<E: Text>(self) -> ~[u8] {
    Text::decode(self)
  }
}

impl SomeHelperTrait for &'self [u8] {
  fn encode<E: Text>(self) -> ~str {
    Text::encode(self)
  }
}
