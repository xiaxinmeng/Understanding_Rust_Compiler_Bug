plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `tikv_jemalloc_sys`
   |
   |
65 | use tikv_jemalloc_sys as jemalloc_sys;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no external crate `tikv_jemalloc_sys`

error[E0432]: unresolved import `tikv_jemallocator`
   |
   |
68 | use tikv_jemallocator as jemallocator;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no external crate `tikv_jemallocator`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc`
