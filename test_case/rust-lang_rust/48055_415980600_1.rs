rust
// Call ABI signature for `<dyn Trait as CloneAs<dyn Trait>>::clone_as`
fn(
     // opaque pointer passed to `ret` as the first argument
    ret_opaque: *(),
    // called to return the unsized value
    ret: fn(
        // `ret_opaque` from above
        opaque: *(),
        // the `dyn Trait` return value's components
        ptr: *(), vtable: *(),
    ) -> (),
    // `self: &dyn Trait`'s components
    self_ptr: *(), self_vtable: *(),
) -> ()
