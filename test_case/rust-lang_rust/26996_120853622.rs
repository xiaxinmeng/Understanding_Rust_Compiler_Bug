 rust
fn main() {
    let mut c = (1, "".to_owned());
    let c2 = c;
    c.0 = 2;
    assert_eq!(c2.0, 1);
}
