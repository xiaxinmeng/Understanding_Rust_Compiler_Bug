 rust
fn main() {
    let mut a = ();
    let borrow = &a;
    (|| {
        &a;
        &mut a;
    })();
}
