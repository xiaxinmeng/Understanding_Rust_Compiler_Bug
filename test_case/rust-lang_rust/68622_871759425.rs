rust
fn main() {
    let a;
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
