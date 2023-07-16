rust
fn main() {
    let _: Vec<_> = vec![Ok::<_, ()>(vec![0]); 5]
        .into_iter()
        .filter(|vec| vec.as_ref().unwrap().len() > 5)
        .collect();
}
