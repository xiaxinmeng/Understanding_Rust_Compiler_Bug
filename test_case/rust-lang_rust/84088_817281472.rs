plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0541]: unknown meta item 'reason'
    |
    |
614 |     #[stable(feature = "option_insert", reason = "newly added", since = "1.53.0")]
    |                                         ^^^^^^^^^^^^^^^^^^^^^^ expected one of `since`, `note`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0541`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:16
