rust
#![feature(slice_patterns)]

fn main() {
    let some_slice: &[u8] = &[];
    match some_slice {
        &[] => {}
        &[_a] => {}
        &[_a, _b] => {}
        &[_a, _b, _c, ..] => {}
    }
}
