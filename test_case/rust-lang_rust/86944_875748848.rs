rust
pub fn main() {
    let x = 1;
    let y = 1;
    let result = (1 |x, y| x + y);
    assert_eq!(result, (1, 3));
}
