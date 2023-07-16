Rust
'a: {
    let x;
    if cond() {
        return // should we generate EndRegion('a) here?
    }
    x = &'a y;
}
