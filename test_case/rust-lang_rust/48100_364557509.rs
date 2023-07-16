
[01:20:34]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:20:34] error[E0658]: use of unstable library feature 'atomic_nand' (see issue #13226)
[01:20:34]   --> libcore/../libcore/tests/atomic.rs:54:18
[01:20:34]    |
[01:20:34] 54 |     assert_eq!(x.fetch_nand(0x137f, SeqCst), 0xf731);
[01:20:34]    |                  ^^^^^^^^^^
[01:20:34]    |
[01:20:34]    = help: add #![feature(atomic_nand)] to the crate attributes to enable
[01:20:34] 
[01:20:34] error[E0658]: use of unstable library feature 'atomic_nand' (see issue #13226)
[01:20:34]   --> libcore/../libcore/tests/atomic.rs:82:18
[01:20:34]    |
[01:20:34] 82 |     assert_eq!(x.fetch_nand(0x137f, SeqCst), 0xf731);
[01:20:34]    |                  ^^^^^^^^^^
[01:20:34]    |
[01:20:34]    = help: add #![feature(atomic_nand)] to the crate attributes to enable
[01:20:34] 
[01:20:53] error: aborting due to 2 previous errors
[01:20:53] 
[01:20:53] error: Could not compile `core`.
