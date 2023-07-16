rust
fn main() {
    #[allow(dead_code)]
    struct U {
        f1: u32,
        f2: f32,
    }
    let mut u = U { f1: 1, f2: 1.0 };
    let b1 = &mut u.f1;
    *b1 = 5;
    assert_eq!( { u.f1 }, 5);
}
