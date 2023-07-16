rust
fn stuff(x: Enum) -> i32 {
    let Enum::Variant(value) = x;
    value
}
