plain
[RUSTC-TIMING] serde test:false 3.448
[RUSTC-TIMING] basic_toml test:false 0.638
   Compiling askama_derive v0.12.1
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
error[E0464]: multiple candidates for `rmeta` dependency `rustc_hash` found
  |
8 | extern crate rustc_hash;
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  |
  = note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librustc_hash-2127386464b12f48.rmeta
  = note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librustc_hash-6142df8766dda1c3.rmeta
[RUSTC-TIMING] serde test:false 3.710
[RUSTC-TIMING] basic_toml test:false 0.700
For more information about this error, try `rustc --explain E0464`.
[RUSTC-TIMING] rustdoc_json_types test:false 0.946
