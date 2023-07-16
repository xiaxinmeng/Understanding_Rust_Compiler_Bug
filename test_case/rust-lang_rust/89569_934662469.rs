plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0532]: expected unit struct, unit variant or constant, found enum `traits::OverflowError`
  --> src/librustdoc/clean/blanket_impl.rs:81:37
   |
81 | ...                   Err(traits::OverflowError) => {}
   |                           ^^^^^^^^^^^^^^^^^^^^^ not a unit struct, unit variant or constant
For more information about this error, try `rustc --explain E0532`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:24
