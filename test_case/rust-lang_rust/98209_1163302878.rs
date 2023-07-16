rust
fn main() {
    let foo = vec![1, 2, 3];
    let borrow = &foo;
    std::mem::drop(foo);
    dbg!(borrow);
}
