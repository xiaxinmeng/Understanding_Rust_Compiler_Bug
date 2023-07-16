rust
fn gimme_static_mut() -> &'static mut u32 {
    match (123443,) {
       (ref mut x,) => x,
    }
}
