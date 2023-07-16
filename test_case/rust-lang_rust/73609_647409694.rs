rust
fn main() {
    #[allow(dead_code)]
    union U {
        f1: u32,
        f2: f32,
    }
    let mut u = U { f1: 1 };
    unsafe {
        let b1 = &mut u.f1;
        *b1 = 5;
    }
    assert_eq!(unsafe { u.f1 }, 5);
}
