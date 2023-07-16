 rust
yield fn zero2ten() -> struct Zero2TenIter {
    let mut x = 0;
    while x <= 10 {
        yield x;
        x += 1;
    }
}
