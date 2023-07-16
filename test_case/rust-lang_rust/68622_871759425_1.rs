rust
fn main() {
    let mut a = 0;
    let x;
    let y;

    {
        let b = 1;
        x = &b as *const _ as usize;
    }

    a = 2;
    y = &a as *const _ as usize;

    assert!(!(x == y));
}
