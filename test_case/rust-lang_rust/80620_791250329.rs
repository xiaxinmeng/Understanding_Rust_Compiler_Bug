rust
#![feature(destructuring_assignment)]

pub fn foo1(mut x: i32, m: i32) -> Option<i32> {
    if m <= 0 { return None; }
    if x < 0 || x >= m { return None; }

    let mut y = x;
    x = m;
    let mut a = 0;
    let mut b = 1;
    while y != 0 {
        (x, y, a, b) = (y, x % y, b, a - x / y * b);
    }
    if x == 1 {
        Some((a + m) % m)
    } else {
        None
    }
}

pub fn foo2(mut x: i32, m: i32) -> Option<i32> {
    if m <= 0 || x < 0 || x >= m { return None; }

    let mut y = x;
    x = m;
    let mut a = 0;
    let mut b = 1;
    while y != 0 {
        (x, y, a, b) = (y, x % y, b, a - x / y * b);
    }
    if x == 1 {
        Some((a + m) % m)
    } else {
        None
    }
}
