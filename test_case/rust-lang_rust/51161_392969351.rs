rust
fn main () {
    let changes: Vec<(Vec<u32>, Vec<u32>)> = vec![];
    let _ = changes
        .iter()
        .map(|(mut old, mut new)| {
            (new, old)
        })
        .collect::<Vec<_>>();
}
