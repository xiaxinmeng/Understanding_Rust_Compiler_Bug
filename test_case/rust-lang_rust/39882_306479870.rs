rust
extern crate serde;
#[macro_use]
extern crate serde_json;

fn main() {
    let val = json!({
        "Hello": "world"
    });

    println!("{}", serde_json::to_string(&val).unwrap());
}
