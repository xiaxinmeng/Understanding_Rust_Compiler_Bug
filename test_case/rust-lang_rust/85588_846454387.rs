plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unused import: `InPlaceIterable`
 --> library/core/src/iter/adapters/peekable.rs:1:56
  |
1 | use crate::iter::{adapters::SourceIter, FusedIterator, InPlaceIterable, TrustedLen};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `core`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:09
