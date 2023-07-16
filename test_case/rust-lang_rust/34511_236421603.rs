 Rust
fn as_debug(s: &str) -> impl fmt::Debug;

fn example() {
    let mut s = String::new("hello");
    let debug = as_debug(&s);
    s.truncate(0);
    println!("{:?}", debug);
}
