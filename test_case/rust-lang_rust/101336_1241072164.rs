rust
/// Extract the vtable pointer from a trait object pointer.
///
/// The validity of the resulting pointer is unchanged from that of the input's vtable pointer.
fn get_vtable(p: *const dyn Trait) -> ptr::NonNull<()> {
    let meta = ptr::metadata(p);
    // elided, since this is more strongly typed it's difficult
}
