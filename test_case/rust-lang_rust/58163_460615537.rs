
error[E0152]: duplicate lang item found: `panic_impl`.
  --> src/main.rs:16:1
   |
16 | / fn panic_handler(_: &core::panic::PanicInfo) -> ! {
17 | |     unsafe {
18 | |         exit(1)
19 | |     }
20 | | }
   | |_^
   |
   = note: first defined in crate `std`.
