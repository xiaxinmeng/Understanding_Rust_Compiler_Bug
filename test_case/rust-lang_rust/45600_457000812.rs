rust
fn bind(a: &Option<String>, b: Option<String>) {
    match (a, b) {
        (&Some(ref x), Some(y)) => println!("{} {}", x, y), 
        _ => (),
    }
}
