rust
|p| {
    for _ in 0..31 {
        black_box(p.cmp(&i));
    }
    p.cmp(&i)
}
