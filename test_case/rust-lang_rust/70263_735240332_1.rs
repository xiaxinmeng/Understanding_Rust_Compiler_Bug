
error: implementation of `FnOnce` is not general enough
   --> src/main.rs:8:5
    |
8   |       example(func);
    |       ^^^^^^^ implementation of `FnOnce` is not general enough
    |
    = note: `FnOnce<(&'0 i32,)>` would have to be implemented for the type `for<'r> fn(&'r i32) -> &'r i32`, for some specific lifetime `'0`...
    = note: ...but `FnOnce<(&i32,)>` is actually implemented for the type `for<'r> fn(&'r i32) -> &'r i32`
