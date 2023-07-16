rust
struct S {
    i: i32,
    s: String,
}

fn main() {
    let s = S { i: 0, s: String::new() };
    println!("{}", s.i);
}
