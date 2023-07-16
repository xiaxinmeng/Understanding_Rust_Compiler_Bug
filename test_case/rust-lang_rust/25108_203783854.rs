 rust
use std::collections::HashMap;

extern crate rustc_serialize;

#[derive(Hash, Eq, PartialEq, RustcEncodable, RustcDecodable)]
struct DefId(u32);

fn main() {
    let mut coll = HashMap::new();
    coll.insert(DefId(0), ());
    let json = rustc_serialize::json::as_json(&coll).to_string();
    println!("json = {}", json);
}
