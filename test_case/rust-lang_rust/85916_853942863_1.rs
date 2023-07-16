rust
// this works
let b = Box::new_in(MaybeUninit::uninit(), alloc);
// ... initialize b
let (ptr, alloc) = Box::into_raw_with_allocator(b);
return Box::from_raw_in(ptr as _, alloc);
