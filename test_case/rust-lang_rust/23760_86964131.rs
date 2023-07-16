 rust
fn main() {
    println!("Type something: ");

    let mut s = String::new();
    let input = std::io::stdin().read_line(&mut s).ok().expect("Failed to read line!");

    println!("Here's what you said: {}", s);
}
