rust
fn fmt_with_vtable(ptr: *const dyn Trait) -> String {
    let (ptr, vtable): (*const u8, *const u8) = unsafe { std::mem::transmute(ptr) };
    format!("{:?} (vtable: {:?})", ptr, vtable)
}
