 rust
yield fn zero2ten() -> struct Zero2TenIter {
    let mut start = 0;
    let mut end   = 10;
} front {
    while start <= end {
        yield start;
        start += 1;
    }
} back {
    while start <= end {
        yield end;
        end -= 1;
    }
}
