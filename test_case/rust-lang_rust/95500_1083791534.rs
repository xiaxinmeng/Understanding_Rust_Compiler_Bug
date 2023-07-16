plain
[RUSTC-TIMING] gimli test:false 5.114
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error[E0658]: associated type bounds are unstable
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:574:24
    |
574 |     I: SliceIndex<[T], Output: UserSafe>,
    |
    = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
    = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
    = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:594:24
    |
594 |     I: SliceIndex<[T], Output: UserSafe>,
    |
    = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
    = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
    = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
[RUSTC-TIMING] std test:false 1.852
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 2 previous errors; 1 warning emitted
