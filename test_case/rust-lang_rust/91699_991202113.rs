plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `NORMALIZE_CSS` in module `static_files`
   --> src/librustdoc/html/render/write_shared.rs:289:49
    |
289 |     write_minify("normalize.css", static_files::NORMALIZE_CSS, cx, options)?;

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:05
