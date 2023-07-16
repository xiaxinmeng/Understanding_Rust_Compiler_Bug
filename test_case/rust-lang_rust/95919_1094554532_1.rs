
error[E0606]: cannot cast `usize` to a pointer that is wide
 --> src/main.rs:6:23
  |
2 |     () => { 0 }
  |             - consider casting this expression to `*const ()`, then using `core::ptr::from_raw_parts`
...
6 |     let x = foo!() as *const [u8];
  |                       ^^^^^^^^^^^ creating a `*const [u8]` requires both an address and a length

For more information about this error, try `rustc --explain E0606`.
