 rust
fn main() {
    let a = "Cats are cute".to_string();

    match &*a {
        "Cats are cute" => println!("They are"),
        _               => panic!("Nope"),
    }
}
