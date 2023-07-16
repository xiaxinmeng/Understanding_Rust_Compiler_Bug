rust
#[inline(never)]
fn nop(x: usize) -> usize { x }

fn main() {
    fn x() {}
    fn y() {}

    let a = x as usize;
    let b = y as usize;
    
    assert_eq!(nop(a), nop(b));
    assert_eq!(a, b);
}
