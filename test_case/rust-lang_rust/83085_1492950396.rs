`rust
fn main() {
    const BOO: &[u8; 0] = &[];
    match &[] {
        BOO => (),
        b"" => (),
        _ => (), 
    }
}
