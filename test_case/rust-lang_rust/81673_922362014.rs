Rust
fn main() {
    let mut foo = 42;
    let r = &foo;

    loop {
        println!("{}", r);
        foo += 1;
    }
}
