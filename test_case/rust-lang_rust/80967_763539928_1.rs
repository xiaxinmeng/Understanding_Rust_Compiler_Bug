rust
fn f() -> Option<usize> {
    let x: Option<Vec<u32>> = Some(vec![]);
    Some(x?.len()).filter(|_| true)
}
