
type-parameter-invalid-lint.rs:14:8: 14:9 error: defaults for type parameters are only allowed on type definitions, like `struct` or `enum`
type-parameter-invalid-lint.rs:14 fn avg<T=i32>(_: T) {}
                                         ^
type-parameter-invalid-lint.rs:14:8: 14:9 warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
type-parameter-invalid-lint.rs:14:8: 14:9 note: for more information, see PR 30742 <https://github.com/rust-lang/rust/pull/30724>
type-parameter-invalid-lint.rs:11:9: 11:28 note: lint level defined here
type-parameter-invalid-lint.rs:11 #![deny(future_incompatible)]
                                          ^~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
