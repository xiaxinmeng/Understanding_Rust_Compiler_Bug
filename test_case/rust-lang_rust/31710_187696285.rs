 rust
fn main() {
    let mut x = unsafe {std::mem::zeroed()};
    let mut y = &[1, 2, 3][x];
    y = &vec![];
    x = ..;
}
