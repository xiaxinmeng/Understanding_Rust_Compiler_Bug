plain
    Checking addr2line v0.14.0
error[E0432]: unresolved import `core::panic_2021`
  --> library/std/src/panic.rs:39:9
   |
39 | pub use core::panic_2021;
   |         ^^^^^^^^^^^^^^^^ no `panic_2021` in the root
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:28
