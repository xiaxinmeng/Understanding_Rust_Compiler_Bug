
error[E0080]: evaluation of constant value failed
   --> /home/lukas/code/rust/library/core/src/slice/raw.rs:100:9
    |
100 |         &*ptr::slice_from_raw_parts(data, len)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |         |
    |         accessing memory with alignment 1, but alignment 2 is required
    |         inside `std::slice::from_raw_parts::<'_, u16>` at /home/lukas/code/rust/library/core/src/slice/raw.rs:100:9
    |
   ::: src/main.rs:23:9
    |
23  |         slice::from_raw_parts(middle.as_ptr().cast::<u16>(), middle.len() / 2)
    |         ---------------------------------------------------------------------- inside `bad_align_to` at src/main.rs:23:9
...
38  |     const _FOO: &[u16] = bad_align_to(&[1, 2, 3]).1; //~ ERROR: it is undefined behavior to use this value
    |                          ------------------------ inside `_FOO` at src/main.rs:38:26

error[E0080]: evaluation of constant value failed
   --> /home/lukas/code/rust/library/core/src/slice/raw.rs:100:9
    |
100 |         &*ptr::slice_from_raw_parts(data, len)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |         |
    |         accessing memory with alignment 1, but alignment 2 is required
    |         inside `std::slice::from_raw_parts::<'_, u16>` at /home/lukas/code/rust/library/core/src/slice/raw.rs:100:9
    |
   ::: src/main.rs:23:9
    |
23  |         slice::from_raw_parts(middle.as_ptr().cast::<u16>(), middle.len() / 2)
    |         ---------------------------------------------------------------------- inside `bad_align_to` at src/main.rs:23:9
...
39  |     const _BAR: u16 = bad_align_to(&[1, 2, 3]).1[0];
    |                       ------------------------ inside `_BAR` at src/main.rs:39:23

For more information about this error, try `rustc --explain E0080`.
error: could not compile `const_align_ub` due to 2 previous errors
