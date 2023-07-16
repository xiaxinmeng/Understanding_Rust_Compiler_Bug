rust
fn main() {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let value = (1..7)
        .map(|n| {
            a.push(n);
            n * 10
        })
        .zip((2..9).map(|n| {
            b.push(n * 100);
            n * 1000
        }))
        .skip(1)
        .nth(3);
    assert_eq!(value, Some((50, 6000)));
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
    assert_eq!(b, vec![200, 300, 400, 500, 600]);
}
