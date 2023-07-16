plain

error[E0308]: mismatched types
   --> library/std/src/sys/wasi/fs.rs:123:25
    |
123 |         self.accessed = t.to_wasi_timestamp_or_panic();
    |         -------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Option`, found `u64`
    |         expected due to the type of this binding
    |
    = note: expected enum `Option<u64>`
               found type `u64`
               found type `u64`
help: try wrapping the expression in `core::prelude::v1::Some`
    |
123 |         self.accessed = core::prelude::v1::Some(t.to_wasi_timestamp_or_panic());

error[E0308]: mismatched types
   --> library/std/src/sys/wasi/fs.rs:127:25
    |
    |
127 |         self.modified = t.to_wasi_timestamp_or_panic();
    |         -------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Option`, found `u64`
    |         expected due to the type of this binding
    |
    = note: expected enum `Option<u64>`
               found type `u64`
               found type `u64`
help: try wrapping the expression in `core::prelude::v1::Some`
    |
127 |         self.modified = core::prelude::v1::Some(t.to_wasi_timestamp_or_panic());

error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
   --> library/std/src/sys/wasi/fs.rs:482:28
    |
    |
482 |             times.accessed.map_or(0, || wasi::FSTFLAGS_ATIM)
    |                            ^^^^^^    -- takes 0 arguments
    |                            expected closure that takes 1 argument
    |
help: consider changing the closure to take and ignore the expected argument
    |
    |
482 |             times.accessed.map_or(0, |_| wasi::FSTFLAGS_ATIM)

error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
   --> library/std/src/sys/wasi/fs.rs:483:34
    |
    |
483 |                 | times.modified.map_or(0, || wasi::FSTFLAGS_MTIM),
    |                                  ^^^^^^    -- takes 0 arguments
    |                                  expected closure that takes 1 argument
    |
help: consider changing the closure to take and ignore the expected argument
    |
    |
483 |                 | times.modified.map_or(0, |_| wasi::FSTFLAGS_MTIM),

Some errors have detailed explanations: E0308, E0593.
For more information about an error, try `rustc --explain E0308`.
[RUSTC-TIMING] std test:false 1.823
