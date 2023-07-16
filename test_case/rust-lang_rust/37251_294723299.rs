
error[E0093]: unrecognized intrinsic function: `prefetch`
   --> src/libcore/intrinsics.rs:569:5
    |
569 |     pub fn prefetch<T>(data: *const T, rw: i32, locality: i32, cache: i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
