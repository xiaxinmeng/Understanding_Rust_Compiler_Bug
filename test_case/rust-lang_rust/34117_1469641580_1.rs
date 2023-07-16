
error[[E0308]](https://doc.rust-lang.org/nightly/error_codes/E0308.html): mismatched types
  --> src/main.rs:31:14
   |
31 |     make_foo(&mut &mut *result); // mismatched types: expected `*mut *mut Foo`, found `&mut &mut _`
   |     -------- ^^^^^^^^^^^^^^^^^ expected `*mut *mut Foo`, found `&mut &mut _`
   |     |
   |     arguments to this function are incorrect
   |
   = note:    expected raw pointer `*mut *mut Foo`
           found mutable reference `&mut &mut _`
