
warning: trivial cast: `&mut T` as `*mut T`
   --> src\lib.rs:200:22
    |
200 |         OpParam::new(op as *mut T as usize)
    |                      ^^^^^^^^^^^^
    |
note: lint level defined here
   --> src\lib.rs:101:9
    |
101 | #![warn(trivial_casts)]
    |         ^^^^^^^^^^^^^
    = help: cast can be replaced by coercion; this might require a temporary variable
