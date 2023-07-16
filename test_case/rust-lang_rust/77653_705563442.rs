
error: couldn't read [...]/substrate/client/service/target-bisector-ci-5849a7eca90582ee59b67eb09548a2aa424d7f52-x86_64-unknown-linux-gnu/debug/build/node-runtime-b5ec2a55671b02f8/out/wasm_binary.rs: No such file or directory (os error 2)
  --> bin/node/runtime/src/lib.rs:95:1
   |
95 | include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
