 Rust
fn main() {
    let a = Box::new(2);
    let mut b = 2;
    *{ *a; &mut b } //~ ERROR use of moved value
        +=
    { drop(a); 1 };
}
