plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0063]: missing field `is_generic` in initializer of `types::Type`
   --> src/librustdoc/json/conversions.rs:513:13
    |
513 |             clean::ResolvedPath { path, did }.into_tcx(tcx)
    |             ^^^^^^^^^^^^^^^^^^^ missing `is_generic`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:22
