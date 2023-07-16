rust
enum Void {}

fn f() -> Option<Void> { None }

fn main() {
    match f() {
        None => println!("none"),
        // WARNING: unreachable pattern, #[warn(unreachable_patterns)] on by default
        Some(_) => println!("some"),
    }

    // ERROR: irrefutable if-let pattern
    // (previously okay)
    if let None = f() {
        println!("definitely none");
    }

    // ERROR: irrefutable if-let pattern
    // (previously okay)
    if let Some(_) = f() {
        println!("definitely some");
    }
}
