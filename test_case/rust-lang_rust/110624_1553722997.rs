
warning: `#[no_mangle]` has no effect on a foreign function
 --> src/lib.rs:3:5
  |
3 |     #[no_mangle]
  |     ^^^^^^^^^^^^ help: remove this attribute
4 |     pub fn linking_c_static();
  |     -------------------------- foreign function
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: symbol names in extern blocks are not mangled
  = note: `#[warn(unused_attributes)]` on by default
