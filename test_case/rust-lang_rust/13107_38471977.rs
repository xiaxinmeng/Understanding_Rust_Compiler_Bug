 rust

#[deriving(Decodable)]
struct Bar {
  foo: uint
}

let json_str = "{\"foo\":false}";
// assume we've merged Parser and Decoder (a future PR)
let decoder = json::Decoder::from_str(json_str);
let bar = match Decodable::decode(decoder) {
  Ok(b) => b,
  Err(e) => fail!("{}", e)
};
