rust
fn gimme_static() -> &'static u32 {
    let ref x = 1234543;
    x
}
