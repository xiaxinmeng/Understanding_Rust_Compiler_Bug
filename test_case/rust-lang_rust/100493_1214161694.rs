rust
unsafe fn foo<'a>(ptr: *const c_char, bogus: PhantomData<&'a ()>) {
  let first = unsafe { CStr::from_ptr::<'a>(ptr) }.to_str();
  bar(first);
}

fn bar(s: &str) {
  // we stop using `s` here
  unsafe { mutate_ffi_string(); }
}
