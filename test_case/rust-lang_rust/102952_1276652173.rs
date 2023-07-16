
error: couldn't read crates/wasmi/benches/wasm/wasm_kernel/res/revcomp-input.txt: No such file or directory (os error 2)
  --> crates/wasmi/benches/benches.rs:18:30
   |
18 | const REVCOMP_INPUT: &[u8] = include_bytes!("wasm/wasm_kernel/res/revcomp-input.txt");
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)
