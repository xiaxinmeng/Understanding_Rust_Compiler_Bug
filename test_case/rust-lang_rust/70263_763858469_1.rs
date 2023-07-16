
error: implementation of `FnOnce` is not general enough
   --> src/main.rs:16:5
    |
16  |       f(&A {}, async_func);
    |       ^ implementation of `FnOnce` is not general enough
    |
    = note: `FnOnce<(&A, &'0 B)>` would have to be implemented for the type `for<'_, '_> fn(&A, &B) -> impl Future {async_func}`, for some specific lifetime `'0`...
    = note: ...but `FnOnce<(&A, &'b B)>` is actually implemented for the type `for<'_, '_> fn(&A, &B) -> impl Future {async_func}`

error: aborting due to previous error

error: could not compile `playground`
