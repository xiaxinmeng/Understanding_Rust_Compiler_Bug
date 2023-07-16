rust
fn get_some_data() -> u128 { unsafe { 
  let x = MaybeUninit::uninitialized();
  ptr::freeze(x.as_mut_ptr());
  x.into_initialized()
} }
