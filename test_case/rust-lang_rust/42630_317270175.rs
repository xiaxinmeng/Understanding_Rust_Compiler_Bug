rust
fn main() {
    let x: u64 = 0;
    match x {
        0x7ff0000000000000 => panic!("1"),
        0x7ff0000000000001 => panic!("2"),
        _ => {}
    };
}
