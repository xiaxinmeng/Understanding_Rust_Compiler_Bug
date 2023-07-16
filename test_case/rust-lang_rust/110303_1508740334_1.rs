text
error[E0080]: evaluation of constant value failed
    --> /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1188:13
     |
1188 |             crate::intrinsics::read_via_copy(src)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ accessing memory with alignment 1, but alignment 4 is required
     |
note: inside `std::ptr::read::<i32>`
    --> /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1188:13
     |
1188 |             crate::intrinsics::read_via_copy(src)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `ptr::const_ptr::<impl *const i32>::read`
    --> /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1198:18
     |
1198 |         unsafe { read(self) }
     |                  ^^^^^^^^^^
note: inside `_`
    --> src/lib.rs:7:14
     |
7    |     unsafe { a.as_ptr().byte_add(1).read() }
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0080]: evaluation of constant value failed
    --> /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1188:13
     |
1188 |             crate::intrinsics::read_via_copy(src)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
     |
note: inside `std::ptr::read::<i32>`
    --> /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1188:13
     |
1188 |             crate::intrinsics::read_via_copy(src)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `ptr::const_ptr::<impl *const i32>::read`
    --> /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1198:18
     |
1198 |         unsafe { read(self) }
     |                  ^^^^^^^^^^
note: inside `_`
    --> src/lib.rs:11:14
     |
11   |     unsafe { core::ptr::null::<i32>().read() }
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0080`.
