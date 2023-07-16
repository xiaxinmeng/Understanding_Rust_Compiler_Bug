 rust
fn bytes<'a>(slice: &'a str) -> () {
    Map {
        iter: slice.as_bytes().iter(),
        f: |&mut: &b| b,
    }
}
