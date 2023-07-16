plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0382]: use of moved value: `options`
    |
947 |                     options,
    |                     ------- value moved here
948 |                     config.should_panic,
948 |                     config.should_panic,
949 |                     config.no_run || options.no_run,
    |                                      ^^^^^^^^^^^^^^ value used here after move
    |
    = note: move occurs because `options` has type `config::Options`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustdoc`
