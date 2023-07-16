rust
fn main() {
    let mut b = vec![1, 2, 3];
    let mut kept = None;

    'outer: for _ in 0..10 {
        for num in b.iter_mut() {
            kept.insert(num);
            break 'outer;
        }
    }
}
