
error[E0320]: overflow while adding drop-check rules for S<i32>
 --> test.rs:7:5
  |
7 |     S { a: None, b: 0i32 };
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: overflowed on i32

error[E0320]: overflow while adding drop-check rules for std::option::Option<std::boxed::Box<S<(i32, i32)>>>
 --> test.rs:7:12
  |
7 |     S { a: None, b: 0i32 };
  |            ^^^^
  |
  = note: overflowed on i32

error: aborting due to 2 previous errors
