rust
error[E0764]: mutable references are not allowed in constants
   --> src/librustc_interface/passes.rs:724:25
    |
724 |     let mut providers = &mut Providers::EMPTY;
    |                         ^^^^^^^^^^^^^^^^^^^^^ `&mut` is only allowed in `const fn`

error[E0764]: mutable references are not allowed in constants
   --> src/librustc_interface/passes.rs:749:21
    |
749 |     let providers = &mut DEFAULT_QUERY_PROVIDERS;
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&mut` is only allowed in `const fn`
