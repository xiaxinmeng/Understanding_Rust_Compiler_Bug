plain
[RUSTC-TIMING] gimli test:false 4.796
[RUSTC-TIMING] object test:false 4.798
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: calls to `std::mem::drop` with a reference instead of an owned value does nothing
   --> library/std/src/sys/sgx/waitqueue/mod.rs:210:17
210 |                 drop(entry);
    |                 ^^^^^-----^
    |                      |
    |                      |
    |                      argument has type `&SpinMutex<WaitEntry>`
    = note: use `let _ = ...` to ignore the expression or result
    = note: use `let _ = ...` to ignore the expression or result
    = note: `-D drop-ref` implied by `-D warnings`
[RUSTC-TIMING] std test:false 2.734
warning: `std` (lib) generated 1 warning
error: could not compile `std` (lib) due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:27:28
