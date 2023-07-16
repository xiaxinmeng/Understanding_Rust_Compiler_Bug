 Rust
fn main() {
    let s : [u8, ..(1<<61)] = [0, ..(1<<61)];
    if s[0] == 0 { unreachable!() }
}
