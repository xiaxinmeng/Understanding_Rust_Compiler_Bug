plain
    Checking addr2line v0.14.0
error[E0308]: mismatched types
    --> library/std/src/io/mod.rs:2467:36
     |
2467 |     default fn size_hint(&self) -> (usize, Option<usize>) {}
     |                ---------           ^^^^^^^^^^^^^^^^^^^^^^ expected tuple, found `()`
     |                |
     |                implicitly returns `()` as its body has no tail or `return` expression
     |
     = note:  expected tuple `(usize, core::option::Option<usize>)`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:31
