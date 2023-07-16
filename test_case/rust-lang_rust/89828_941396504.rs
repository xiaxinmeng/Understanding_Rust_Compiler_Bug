plain
   Compiling toml v0.5.7
error[E0425]: cannot find value `rust_overflow_checks` in this scope
   --> src/bootstrap/config.rs:985:73
    |
985 |         config.rust_overflow_checks_std = overflow_checks_std.unwrap_or(rust_overflow_checks);
    |                                                                         ^^^^^^^^^^^^^^^^^^^^ a field by this name exists in `Self`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:00:58
