rust
fn main() {
    let array = ['w', 'x', 'y', 'z'];
    let mut range = 0..=3;
    while let Some(i) = range.next() {
        println!(
            "after i={}, the remaining elements are {:?}",
            i, &array[range.clone()],
        );
    }
}
