
fn main() {
    let a: f32 = 0.111294314;
    let b: f32 = 0.111294307;
    println!("with precision 6; does {0:.6} equal {1:.6}?", a, b);
    println!("with precision 7; does {0:.7} equal {1:.7}?", a, b);
    println!("with precision 8; does {0:.8} equal {1:.8}?", a, b);
    println!("with precision 9; does {0:.9} equal {1:.9}?", a, b);
    assert_eq!(a, b);
}
