rust
extern crate serde;
extern crate serde_json;

fn use_after_free<T: serde::de::DeserializeOwned>() -> T {
    let json = r#" "oh my" "#.to_owned();
    serde_json::from_str(&json).unwrap()
}

fn main() {
    println!("{}", use_after_free::<&'static str>());
}
