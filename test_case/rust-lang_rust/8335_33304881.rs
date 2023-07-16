 rust
impl <A:Encodable> ToJson for A {
  fn to_json(&self) -> ~Json {
    ToJsonEncoder::new().json_encode(self)
  }
}

struct ToJsonEncoder {
// ...
}
impl serialize::Encoder for ToJsonEncoder {
// ...
}
