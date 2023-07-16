rust
unsafe // Safety: keep it cool :)
fn bikeshed (x: *mut SomeStruct)
{
    use ::core::ptr;

    ptr::raw_const!((*x).some_field);
    ptr::raw_mut!((*x).some_field);

    ptr::const_addr_of!((*x).some_field);
    ptr::mut_addr_of!((*x).some_field);

    // Personal modification of the `{const,mut}_raw_ptr!` suggestion
    ptr::const_ptr_to!((*x).some_field);
    ptr::mut_ptr_to!((*x).some_field);

    // Another personal suggestion
    ptr::raw_ptr!(&raw const (*x).some_field);
    ptr::raw_ptr!(&raw mut (*x).some_field);

    use ::core::ptr::*;

    raw_const!((*x).some_field);
    raw_mut!((*x).some_field);

    const_addr_of!((*x).some_field);
    mut_addr_of!((*x).some_field);

    const_ptr_to!((*x).some_field);
    mut_ptr_to!((*x).some_field);

    raw_ptr!(&raw const (*x).some_field);
    raw_ptr!(&raw mut (*x).some_field);
}
