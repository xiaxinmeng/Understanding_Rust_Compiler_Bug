 rust
fn main() {
    let mut c = (1, "".to_owned());
    match c {
        c2 => { println!("{}", c.0); c.0 = 2; assert_eq!(c2.0, 1); }
    }
}
