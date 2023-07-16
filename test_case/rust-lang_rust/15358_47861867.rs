 rust
// foo.rs

#[deriving(Encodable, Decodable)]
struct Serie {
  metric: String,
  type_: String
}
