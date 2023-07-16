text
error[E0080]: evaluation of constant value failed
    --> /home/lukas/code/rust/library/core/src/ptr/mod.rs:1165:5
     |
1165 | /     debug_assert_nounwind!(
1166 | |         is_aligned_and_not_null(src),
1167 | |         "ptr::read requires that the pointer argument is aligned and non-null",
1168 | |     );
     | |_____^ the evaluated program panicked at 'ptr::read requires that the pointer argument is aligned and non-null', /home/lukas/code/rust/library/core/src/ptr/mod.rs:1165:5
     |
note: inside `std::ptr::read::<i32>`
    --> /home/lukas/code/rust/library/core/src/ptr/mod.rs:1165:5
     |
1165 | /     debug_assert_nounwind!(
1166 | |         is_aligned_and_not_null(src),
1167 | |         "ptr::read requires that the pointer argument is aligned and non-null",
1168 | |     );
     | |_____^
note: inside `ptr::const_ptr::<impl *const i32>::read`
    --> /home/lukas/code/rust/library/core/src/ptr/const_ptr.rs:1198:18
     |
1198 |         unsafe { read(self) }
     |                  ^^^^^^^^^^
note: inside `_`
    --> src/lib.rs:7:14
     |
7    |     unsafe { a.as_ptr().byte_add(1).read() }
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: this error originates in the macro `debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
    --> /home/lukas/code/rust/library/core/src/ptr/mod.rs:1165:5
     |
1165 | /     debug_assert_nounwind!(
1166 | |         is_aligned_and_not_null(src),
1167 | |         "ptr::read requires that the pointer argument is aligned and non-null",
1168 | |     );
     | |_____^ the evaluated program panicked at 'ptr::read requires that the pointer argument is aligned and non-null', /home/lukas/code/rust/library/core/src/ptr/mod.rs:1165:5
     |
note: inside `std::ptr::read::<i32>`
    --> /home/lukas/code/rust/library/core/src/ptr/mod.rs:1165:5
     |
1165 | /     debug_assert_nounwind!(
1166 | |         is_aligned_and_not_null(src),
1167 | |         "ptr::read requires that the pointer argument is aligned and non-null",
1168 | |     );
     | |_____^
note: inside `ptr::const_ptr::<impl *const i32>::read`
    --> /home/lukas/code/rust/library/core/src/ptr/const_ptr.rs:1198:18
     |
1198 |         unsafe { read(self) }
     |                  ^^^^^^^^^^
note: inside `_`
    --> src/lib.rs:11:14
     |
11   |     unsafe { core::ptr::null::<i32>().read() }
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: this error originates in the macro `debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0080`.
