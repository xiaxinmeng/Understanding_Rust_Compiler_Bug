
extern mod std;
use std::json;
use std::serialize;

fn main() {
    let json = json::from_str("[1]").unwrap();
    let _x: ~[int] = serialize::Decodable::decode(&json::Decoder(json));
}
