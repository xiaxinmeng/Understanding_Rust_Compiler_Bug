rust
fn bug() {
    'a: loop {
        async { break 'a 0 }
    }
}
