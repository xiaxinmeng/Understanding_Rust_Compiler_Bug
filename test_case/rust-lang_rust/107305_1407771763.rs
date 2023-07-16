rust
fn main() {
    let mut a = ();
    let a1 = &mut a;
    let a2 = &mut *a1;

    let p2 = a2 as *const ();
    let p1 = a1 as *const ();
    
    assert_eq!(p1, p2);
}
