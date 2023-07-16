
type-parameter-invalid-lint.rs:14:8: 14:9 error: defaults for type parameters were only intended to be allowed on `struct` or `enum` definitions
type-parameter-invalid-lint.rs:14 fn avg<T=i32>(_: T) {}
                                         ^
type-parameter-invalid-lint.rs:14:8: 14:9 warning: this was fixed in Rust 1.7; it will become a HARD ERROR in a future release!
