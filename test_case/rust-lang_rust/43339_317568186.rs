Rust
#[cfg(target_pointer_width = "32")]
fn bar(foo: *const u8) {
    let x = foo as u32; // No warning
}

fn baz(foo: *const u8) {
    let x = foo as u32; // warning: pointer cast is not portable ...
}
