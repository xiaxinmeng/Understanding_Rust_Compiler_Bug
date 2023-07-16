 rust
fn main() {
    let x = 4096 as *u8;
    match x {
        _ => fail!("foo")
    }
}
