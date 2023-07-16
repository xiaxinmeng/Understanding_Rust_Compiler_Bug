
error[E0277]: `?` couldn't convert the error to `!`
   --> src/librustc_metadata/encoder.rs:106:33
    |
106 |         self.emit_usize(seq.len)?;
    |                                 ^
    |                                 |
    |                                 the trait `std::convert::From<()>` is not implemented for `!`
    |                                 in this expansion of `desugaring of `?``
    |                                 in this macro invocation
    |
    = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
    = note: required because of the requirements on the impl of `std::convert::Into<!>` for `()`
    = note: required by `std::convert::Into::into`
