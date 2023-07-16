
type-parameter-invalid-lint.rs:14:8: 14:9 error: defaults for type parameters were only intended to be allowed on `struct` or `enum` definitions
type-parameter-invalid-lint.rs:14 fn avg<T=i32>(_: T) {}
                                         ^
type-parameter-invalid-lint.rs:14:8: 14:9 warning: this was fixed in Rust 1.7; it will become a HARD ERROR in a future release!
type-parameter-invalid-lint.rs:14 fn avg<T=i32>(_: T) {}
                                         ^
type-parameter-invalid-lint.rs:14:8: 14:9 note: for more information, see https://github.com/rust-lang/rust/pull/30724
type-parameter-invalid-lint.rs:14 fn avg<T=i32>(_: T) {}
                                         ^
type-parameter-invalid-lint.rs:11:9: 11:28 note: lint level defined here
type-parameter-invalid-lint.rs:11 #![deny(future_incompatible)]
                                          ^~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
