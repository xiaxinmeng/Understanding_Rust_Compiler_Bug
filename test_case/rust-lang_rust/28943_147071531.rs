 rust
fn test() -> Vec<u8> {
    fn print<T: std::fmt::Debug>(t: T) { println!("{:?}", t); }
    let x = 5;
    print(x);
    let y = x + x;
    vec![y]
}
