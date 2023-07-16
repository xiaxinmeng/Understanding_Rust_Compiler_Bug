 rust
extern crate serialize;
use serialize::json;

#[deriving(Decodable)]
struct Foo {
    field1: String,
    field2: String
}

fn main() {
    let data = r##"
    {
        "field1": "something",
        "non_existing_field": "something_else"
    }
    "##;
    let _ = json::decode::<Foo>(data.as_slice());
}
