
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: `extern` fn uses type `(u64, u64)`, which is not FFI-safe
  --> src/libstd/sys/sgx/abi/mod.rs:59:86
   |
59 | extern "C" fn entry(p1: u64, p2: u64, p3: u64, secondary: bool, p4: u64, p5: u64) -> (u64, u64) {
   |                                                                                      ^^^^^^^^^^ not FFI-safe
   |
   = note: `-D improper-ctypes-definitions` implied by `-D warnings`
   = help: consider using a struct instead
   = note: tuples have unspecified layout

error: aborting due to previous error; 1 warning emitted
