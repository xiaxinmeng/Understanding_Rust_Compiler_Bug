plain
diff of stderr:

31    | ^^^^^^^^^^^
32    |
33    = note: the lang item is first defined in crate `core`.
-    = note: first definition in `core` loaded from $BUILD_DIR/aarch64-apple-darwin/stage1/lib/rustlib/aarch64-apple-darwin/lib/libcore-500f4c12402b1108.rlib
+    = note: first definition in `core` loaded from $BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
35    = note: second definition in the local crate (`issue_102989`)
36 
37 error: `#[panic_handler]` function required, but not found

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989/issue-102989.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-102989.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-102989.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Cinstrument-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989/auxiliary"
stdout: none
--- stderr -------------------------------
error: `self` parameter is only allowed in associated functions
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |               ^^^^ not semantically valid as function parameter
   |
   = note: associated functions are those in `impl` or `trait` definitions
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0412]: cannot find type `Struct` in this scope
  --> /checkout/src/test/ui/traits/issue-102989.rs:9:22
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {

error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/traits/issue-102989.rs:13:13
   |
   |
LL |     let x = x << 1;
   |             ^ help: a local variable with a similar name exists: `f`

error: `profiler_builtins` crate (required by compiler options) is not compatible with crate attribute `#![no_core]`
error[E0463]: can't find crate for `profiler_builtins`
   |
   = note: the compiler may have been built without the profiler runtime


error[E0152]: found duplicate lang item `sized`
   |
   |
LL | trait Sized { } //~ ERROR found duplicate lang item `sized`
   |
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`issue_102989`)

error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
   |
   = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
   = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error[E0277]: the size for values of type `{integer}` cannot be known at compilation time
  --> /checkout/src/test/ui/traits/issue-102989.rs:13:15
   |
LL |     let x = x << 1;
LL |     let x = x << 1;
   |               ^^ doesn't have a size known at compile-time
   |
   = help: the trait `core::marker::Sized` is not implemented for `{integer}`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/traits/issue-102989.rs:9:42
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |    ----------                            ^^^^ expected `&u32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
note: consider returning one of these bindings
  --> /checkout/src/test/ui/traits/issue-102989.rs:9:30
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
...
LL |     let x = x << 1;
   |         ^

