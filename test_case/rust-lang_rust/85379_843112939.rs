
[RUSTC-TIMING] object test:false 10.149
error: unused import: `ptr`
 --> library/std/src/os/./unix/net/addr.rs:5:40
  |
5 | use crate::{ascii, fmt, io, iter, mem, ptr};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to previous error

[RUSTC-TIMING] std test:false 2.398
error: could not compile `std`
