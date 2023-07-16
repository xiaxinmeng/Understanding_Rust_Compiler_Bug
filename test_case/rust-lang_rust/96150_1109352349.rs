plain
normalized stderr:
warning: 1st rule of macro `const_assert` is never used
  --> $DIR/const-float-bits-conv.rs:11:5
   |
LL |     ($a:expr) => {
   |
   = note: `#[warn(unused_macro_rules)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args consts/const-float-bits-conv.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-float-bits-conv.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-conv/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-conv/auxiliary"
stdout: none
--- stderr -------------------------------
warning: 1st rule of macro `const_assert` is never used
   |
   |
LL |     ($a:expr) => {
   |
   = note: `#[warn(unused_macro_rules)]` on by default

warning: 1 warning emitted
