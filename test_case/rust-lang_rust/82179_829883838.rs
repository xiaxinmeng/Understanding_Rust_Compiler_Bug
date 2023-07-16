plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0433]: failed to resolve: use of undeclared crate or module `core`
   --> library/core/src/time.rs:694:23
    |
694 |             Err(e) => core::panicking::panic(e.description())
    |                       ^^^^ use of undeclared crate or module `core`
error[E0433]: failed to resolve: use of undeclared crate or module `core`
   --> library/core/src/time.rs:755:23
    |
    |
755 |             Err(e) => core::panicking::panic(e.description())
    |                       ^^^^ use of undeclared crate or module `core`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:41
