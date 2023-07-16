rust
fn f(items: &mut [i16]) {
    for item in items.iter_mut() {
        item = 1;
    }
}
