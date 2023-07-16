rust
error: implementation of `std::fmt::Debug` is not general enough
   --> src/lib.rs:3:9
    |
3   |       Bar(fn(&())),
    |           ^^^^^^^ implementation of `std::fmt::Debug` is not general enough
    |
    = note: `std::fmt::Debug` would have to be implemented for the type `for<'r> fn(&'r ())`
    = note: ...but `std::fmt::Debug` is actually implemented for the type `fn(&'0 ())`, for some specific lifetime `'0`
    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
