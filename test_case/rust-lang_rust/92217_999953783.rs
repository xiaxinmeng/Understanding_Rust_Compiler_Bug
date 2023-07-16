rust
fn bar() {
    let val = match Some(true) {
        Some(value) => value,
        None => return
    };
}
