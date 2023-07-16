
extern mod std;

use std::json;
use std::serialize::Decodable;

trait JD : Decodable<json::Decoder> { }

fn exec<T: JD>() {
    let doc = result::unwrap(json::from_str(""));
    let mut decoder = json::Decoder(doc);
    let _v: T = Decodable::decode(&mut decoder);
    fail!()
}

fn main() {}
