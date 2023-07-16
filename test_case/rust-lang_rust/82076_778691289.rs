plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0432]: unresolved import `crate::macros::builtin::derive`
  --> library/core/src/prelude/v1.rs:74:9
   |
74 | pub use crate::macros::builtin::derive;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `derive` in `macros::builtin`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:18
