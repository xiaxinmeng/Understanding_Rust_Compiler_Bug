 Rust
#![feature(slice_patterns)]

fn main() {
    match "ab".to_string().as_bytes() {
        b"\n" => (),
        &[b1, b2] => (),
        _ => ()
    }
}
