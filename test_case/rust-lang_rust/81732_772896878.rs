plain
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unused imports: `AddAssign`, `Add`
 --> library/core/src/iter/adapters/enumerate.rs:3:18
  |
3 | use crate::ops::{Add, AddAssign, Try};
  |                  ^^^  ^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `Add`, `Sub`
  |
  |
4 | use crate::ops::{self, Add, Sub, Try};
  |                        ^^^  ^^^

error: unused imports: `Add`, `Mul`
  |
  |
3 | use crate::ops::{Add, Mul};
  |                  ^^^  ^^^
error: unused import: `Add`
 --> library/core/src/iter/traits/iterator.rs:6:18
  |
  |
6 | use crate::ops::{Add, ControlFlow, Try};

error: aborting due to 4 previous errors

error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:27
