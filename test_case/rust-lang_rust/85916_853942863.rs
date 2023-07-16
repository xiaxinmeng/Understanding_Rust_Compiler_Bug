rust
// doesn't work
let b = Box::new_in(MaybeUninit::uninit(), &alloc);
// ... initialize b
return Box::from_raw_in(Box::into_raw(b) as _, alloc);
