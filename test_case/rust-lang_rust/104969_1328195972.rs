plain
normalized stderr:
warning: an associated function with this name may be added to the standard library in the future
  --> $DIR/example-calendar.rs:719:31
   |
LL |     let c = r.iter().cloned().chunks(3).collect::<Vec<_>>();
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `Chunks::chunks(...)` to keep using the current method
   = help: add `#![feature(iter_array_chunks)]` to the crate attributes to enable `chunks`

warning: an associated function with this name may be added to the standard library in the future
  --> $DIR/example-calendar.rs:734:10
   |
   |
LL |         .chunks(months_per_row)
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `Chunks::chunks(...)` to keep using the current method
   = help: add `#![feature(iter_array_chunks)]` to the crate attributes to enable `chunks`
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args impl-trait/example-calendar.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/example-calendar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/example-calendar/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/example-calendar.rs:719:31
   |
   |
LL |     let c = r.iter().cloned().chunks(3).collect::<Vec<_>>();
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `Chunks::chunks(...)` to keep using the current method
   = help: add `#![feature(iter_array_chunks)]` to the crate attributes to enable `chunks`

warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/impl-trait/example-calendar.rs:734:10
   |
   |
LL |         .chunks(months_per_row)
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `Chunks::chunks(...)` to keep using the current method
   = help: add `#![feature(iter_array_chunks)]` to the crate attributes to enable `chunks`
warning: 2 warnings emitted
------------------------------------------


