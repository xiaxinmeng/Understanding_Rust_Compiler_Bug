rust
error: implementation of `std::ops::FnOnce` is not general enough
   --> src/main.rs:36:5
    |
36  |       check(_handler3); // FIXME: Why doesn't this work? What lifetimes does the generated `Future` have?
    |       ^^^^^ implementation of `std::ops::FnOnce` is not general enough
    |
    = note: `std::ops::FnOnce<(&Request<'0>,)>` would have to be implemented for the type `for<'r, '_> fn(&'r Request<'_>) -> impl std::future::Future {main::_handler3}`, for some specific lifetime `'0`...
    = note: ...but `std::ops::FnOnce<(&Request<'_>,)>` is actually implemented for the type `for<'r, '_> fn(&'r Request<'_>) -> impl std::future::Future {main::_handler3}`
