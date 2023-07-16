 Rust
struct Foo(&'static u32);

fn main() {
    let x = 42;
    Foo(&x);
}
