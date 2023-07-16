rust
fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut ref_a = &mut a;
    let (ref mut ref_ref_a, ()) = (ref_a, ());
    println!("{}", ref_a); // use of moved value
    ref_a = &mut b;
}
