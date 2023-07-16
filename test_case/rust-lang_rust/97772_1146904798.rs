plain
Testing rustdoc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `as_bytes` found for struct `minifier::css::Minified` in the current scope
   --> src/librustdoc/theme/tests.rs:112:41
    |
112 |     let other = load_css_paths(minified.as_bytes());
    |                                         ^^^^^^^^ method not found in `minifier::css::Minified<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:26:21
