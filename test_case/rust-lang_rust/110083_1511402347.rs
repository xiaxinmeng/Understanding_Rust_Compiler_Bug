plain
    Checking chalk-engine v0.87.0
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/svh.rs:27:9
   |
26 |     pub fn as_u64(&self) -> u64 {
   |                             --- expected `u64` because of return type
27 |         self.hash.to_smaller_hash()
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `Hash64`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_data_structures` due to previous error
