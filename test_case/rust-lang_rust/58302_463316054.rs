rust
error[E0277]: the trait bound `!: std::convert::From<()>` is not satisfied
   --> src/librustc_metadata/encoder.rs:105:9
    |
105 |         self.emit_usize(seq.len)?;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<()>` is not implemented for `!`
    |
    = help: the following implementations were found:
              <! as std::convert::From<std::convert::Infallible>>
    = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
    = note: required by `std::convert::From::from`
