 rust
#[feature(managed_boxes)];

extern mod extra;
use extra::serialize::Decodable;
use extra::json;

#[deriving(Decodable)]
struct Foo {
    x: ~[Foo]
}

fn main() {
    let mut x = json::Decoder(json::from_str(r#"{"x": 1}"#).unwrap());
    let _: Foo = Decodable::decode(&mut x);
}
