 rust
const C: *const [u8; 4] = b"abcd";

fn main() {
    match C {
        C => {}
        _ => {}
    }
}
