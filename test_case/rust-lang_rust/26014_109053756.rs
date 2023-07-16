 rust
fn main() {
    let f = match 1 {
        1 => Box::new(|c| c + 1),
        _ => Box::new(|c| c - 1),
    };

    println!("{}", f(1));
}
