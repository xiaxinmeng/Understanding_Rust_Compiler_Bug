plain
[RUSTC-TIMING] gimli test:false 5.320
[RUSTC-TIMING] object test:false 5.545
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error: couldn't read library/std/src/sys/wasm/../unsupported/locks.rs: No such file or directory (os error 2)
  --> library/std/src/sys/wasm/mod.rs:69:9
69 |         pub mod locks;
   |         ^^^^^^^^^^^^^^

[RUSTC-TIMING] std test:false 0.270
