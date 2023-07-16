plain
normalized stderr:
warning: an associated function with this name may be added to the standard library in the future
  --> $DIR/env-funky-keys.rs:33:41
   |
LL | ...                   .as_bytes()).unwrap();
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `as_bytes(...)` to keep using the current method
   = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable `OsStr::as_bytes`
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args env-funky-keys.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/env-funky-keys.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/env-funky-keys/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/env-funky-keys/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/env-funky-keys.rs:33:41
   |
   |
LL | ...                   .as_bytes()).unwrap();
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `as_bytes(...)` to keep using the current method
   = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable `OsStr::as_bytes`
warning: 1 warning emitted
------------------------------------------


