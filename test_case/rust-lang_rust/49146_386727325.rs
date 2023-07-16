rust
let x = 0;
let non_deterministic = &x as *const _ as usize;
if non_deterministic.count_ones() % 2 == 0 {
    // do one thing
} else {
    // do a completely different thing
}
