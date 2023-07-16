 rust
fn main() {
    let vector = vec![1];
    let _ = vector
        .iter()
        .map(|_| -> Option<(i32, i32)> {
            let t = vec![1];
            
            // It's expected to be a compile error
            // A correct version: Some((t.into_iter().next().unwrap(), 1))
            Some(t.into_iter().next().unwrap(), 1)
        })
        .collect::<Vec<_>>();
}
