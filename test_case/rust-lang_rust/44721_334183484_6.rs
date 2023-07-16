rust
fn any_zero<'a>(values: impl IntoIterator<Item=&'a u32>) -> bool {
    for v in values { if *v == 0 { return true; } }
    false
}
