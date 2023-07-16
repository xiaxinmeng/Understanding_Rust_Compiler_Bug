rust
fn main() {
    let mut x = &3;
    let rw = &mut *x;
    *rw = 4;
    println!("x: {:?}", x);
}
