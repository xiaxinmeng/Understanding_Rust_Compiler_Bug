rust
fn foo<T: ?Sized>(ptr: *const T) {
    let mut ptr_parts: [usize; std::mem::size_of::<*const T>() / std::mem::size_of::<usize>()]
        = unsafe { std::mem::transmute(ptr) };
}
