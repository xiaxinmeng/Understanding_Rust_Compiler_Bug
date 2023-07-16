rust
   #[no_mangle]
   pub unsafe extern "C" fn foo(ptr: *const u8) {
       ptr.offset(10);
   }
   