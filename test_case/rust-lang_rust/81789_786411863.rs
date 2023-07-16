rust
fn main() {
    fn test() {}
    let x: u16 = ({
        test
    }) as u16;
}
