 rust
pub fn test(foo: bool) -> u8 {
    match foo {
        true => *Box::new(0),
        false => 0
    }
}

fn main() {
    test(true);
}
