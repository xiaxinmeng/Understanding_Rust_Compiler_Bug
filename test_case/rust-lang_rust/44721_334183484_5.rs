rust
fn any_zero(values: impl IntoIterator<Item=u32>) -> bool {
    for v in values { if v == 0 { return true; } }
    false
}
