Rust
fn main() {
    let x = SimpleWrapper { value: 10 };
    let foo = |wrapper: &SimpleWrapper| wrapper;
    let y = foo(&x);
}

struct SimpleWrapper {
    value: i32,
}
