 rust
struct S {
    s: str
}

fn main() {
    let hi = "Hi";
    let s = S { s: *hi };
    println!("Hello errors!");
}
