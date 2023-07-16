rust
fn main() {
    let mut a;
    let x;
    let mut y;

    if false {
        a = 1;
        y = &a as *const _ as usize;
    }

    {
        let b = 0;
        x = &b as *const _ as usize;
    }

    a = 2;
    y = &a as *const _ as usize;

    assert!(!(x == y));
}
