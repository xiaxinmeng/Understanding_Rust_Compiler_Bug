
error[E0433]: failed to resolve: use of undeclared crate or module `alloc`
   --> src\range_type.rs:246:14
    |
246 |             &alloc::format!("{}", invalid_deserialized_error.unwrap_err()),
    |              ^^^^^ use of undeclared crate or module `alloc`
