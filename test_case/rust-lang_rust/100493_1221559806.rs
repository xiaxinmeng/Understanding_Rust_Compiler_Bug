rust
unsafe fn foo<'a>(ptr: ConstLtPtr<'a, c_char>) {
  let first = unsafe { CStr::from_lt_ptr(ptr) }.to_str().map(|x| x.to_owned());
  unsafe { mutate_ffi_string(); }
  let second = unsafe { CStr::from_lt_ptr(ptr) }.to_str().map(|x| x.to_owned());
}
