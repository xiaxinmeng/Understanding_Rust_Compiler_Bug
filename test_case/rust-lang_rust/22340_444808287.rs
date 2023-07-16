rust
fn print_first(list: Vec<String>) {
    let x: &str = list
    .first()
    // .map(|s: &String| -> &str { &s[..] })  // OK
    // .map(|s: &String| { &s[..] }) // OK
    // .map(|s| -> &str { &s[..] }) // OK
    .map(|s| { &s[..] })  // OK
    .unwrap_or("");
    println!("First element is {}", x);
}

fn main() {
    print_first(vec![format!("hello"), format!("world")]);
}
