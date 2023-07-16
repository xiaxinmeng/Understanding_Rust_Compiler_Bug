 rust
const FOO: &'static [u8, ..3] = [1, 2, 3];

fn main() {
    let a = [1u8, 2, 3].as_slice();
    match a {
        FOO => {}
        _ => {}
    }
}
