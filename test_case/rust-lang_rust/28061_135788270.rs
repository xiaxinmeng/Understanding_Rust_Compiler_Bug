 rust
enum E {
    Container(String)
}

fn main() {
    let e = E::Container(String::from("data"));
    match e {
        E::Container(ref s) if *s == "data" => println!("Match"),
        _ => println!("No match"),
    }
}
