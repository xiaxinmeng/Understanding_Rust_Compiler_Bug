
error[E0152]: duplicate lang item found: `panic_fmt`.
 --> src/lib.rs:4:1
  |
4 | / fn panic_fmt() -> ! {
5 | |     loop {}
6 | | }
  | |_^
  |
  = note: first defined in crate `std`.

error: aborting due to previous error
