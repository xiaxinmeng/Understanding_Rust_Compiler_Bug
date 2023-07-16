 rust
fn main() {
    let f: Box<Fn(i32) -> i32> = match 1 {
        1 => Box::new(|c| c + 1),
        _ => Box::new(|c| c - 1),
    };

    println!("{}", f(1));
}
