rust
fn f() -> Option<usize> {
    let x: Vec<u32> = Some(vec![])?;
    true.then(|| x.len())
}
