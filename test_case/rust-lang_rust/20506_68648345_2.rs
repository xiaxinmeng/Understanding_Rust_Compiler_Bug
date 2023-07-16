 rust
fn main() {
    let mut arr = range(0, 10u).map(|_| 0.0f64).collect();
    let mut sum = 0.0f64;
    for j in range(0, 10u) {
        arr[j] = 0.0f64;
        sum += arr[j];
    }
}
