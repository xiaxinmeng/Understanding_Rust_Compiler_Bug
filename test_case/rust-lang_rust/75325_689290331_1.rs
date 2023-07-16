rust
pub fn foo(vtable: &AnyVTable) -> (usize, usize, TypeId) {
    // `vtable.type_id_impl` implements virtual method `Any::type_id`
    (vtable.size, vtable.align, (vtable.type_id_impl)())
}
