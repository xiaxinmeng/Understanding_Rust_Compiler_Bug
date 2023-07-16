
   --> src/liballoc/boxed_test.rs:154:52
    |
154 |     let ptr: std::ptr::NonNull<i32> = Box::from(0).into();
    |                                                    ^^^^ the trait `core::convert::From<std::boxed::Box<{integer}>>` is not implemented for `core::ptr::NonNull<i32>`
    |
    = help: the following implementations were found:
              <core::ptr::NonNull<T> as core::convert::From<&'a T>>
              <core::ptr::NonNull<T> as core::convert::From<&'a mut T>>
              <core::ptr::NonNull<T> as core::convert::From<core::ptr::Unique<T>>>
    = note: required because of the requirements on the impl of `core::convert::Into<core::ptr::NonNull<i32>>` for `std::boxed::Box<{integer}>`
