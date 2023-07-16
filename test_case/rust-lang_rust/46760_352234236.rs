
[00:34:48] error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
[00:34:48]    --> /checkout/src/rustc/libc_shim/../../liblibc/src/unix/bsd/freebsdlike/freebsd/mod.rs:453:1
[00:34:48]     |
[00:34:48] 453 | pub const IFF_DRV_RUNNING: ::c_int = 0x40;
[00:34:48]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:34:48]
[00:34:48] error: `#[deprecated]` cannot be used in staged api, use `#[rustc_deprecated]` instead
[00:34:48]    --> /checkout/src/rustc/libc_shim/../../liblibc/src/unix/bsd/freebsdlike/freebsd/mod.rs:460:1
[00:34:48]     |
[00:34:48] 460 | pub const IFF_DRV_OACTIVE: ::c_int = 0x400;
[00:34:48]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:34:48]
