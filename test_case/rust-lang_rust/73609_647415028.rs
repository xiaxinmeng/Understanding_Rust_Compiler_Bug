rust
fn main() {
    let mut u = (1,);
    *&mut u.0 = 5;
    assert_eq!( { u.0 }, 5);
}
