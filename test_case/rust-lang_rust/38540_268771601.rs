
error[E0308]: mismatched types
   --> tests/test.rs:129:42
    |
129 |                 knot.tie(knot.wrap0(knot.tie(Timeout::new(Duration::new(5, 0), &handle).unwrap())))
    |                                          ^^^ conflicting type parameter defaults `_` and `_`
    |
    = note: expected type `()`
    = note:    found type `i32`
