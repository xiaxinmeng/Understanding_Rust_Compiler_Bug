plain
    Checking addr2line v0.14.0
error[E0282]: type annotations needed
   --> library/std/src/io/mod.rs:381:9
    |
381 |     let ret;
    |         ^^^ consider giving `ret` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:46
