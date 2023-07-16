rust
error[E0658]: references in constant functions may only refer to immutable values
   --> src/libcore/mem/maybe_uninit.rs:360:13
    |
360 |             u.as_mut_ptr().write_bytes(0u8, 1);
    |             ^ constant functions require immutable values
    |
    = note: for more information, see https://github.com/rust-lang/rust/issues/57349
    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   --> src/libcore/mem/maybe_uninit.rs:360:13
    |
360 |             u.as_mut_ptr().write_bytes(0u8, 1);
    |             ^^^^^^^^^^^^^^

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   --> src/libcore/mem/maybe_uninit.rs:360:13
    |
360 |             u.as_mut_ptr().write_bytes(0u8, 1);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 3 previous errors
