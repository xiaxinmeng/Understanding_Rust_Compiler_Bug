plain
[RUSTC-TIMING] object test:false 5.523
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error[E0282]: type annotations needed
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:118:27
    |
118 |         let is_aligned = |p| -> bool { 0 == p.addr() & (Self::align_of() - 1) };
    |                           ^                 - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
118 |         let is_aligned = |p: _| -> bool { 0 == p.addr() & (Self::align_of() - 1) };

For more information about this error, try `rustc --explain E0282`.
[RUSTC-TIMING] std test:false 2.517
warning: `std` (lib) generated 1 warning
