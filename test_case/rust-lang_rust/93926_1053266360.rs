plain

---- [ui] ui/lint/unused/unused_attributes-must_use.rs stdout ----
diff of stderr:

+ error[E0658]: inline assembly is not stable yet on this architecture
+   --> $DIR/unused_attributes-must_use.rs:59:1
+    |
+ LL | global_asm!("");
+    |
+    = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
+    = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+    = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
1 error: unused attribute `must_use`
2   --> $DIR/unused_attributes-must_use.rs:58:1
3    |


123 LL |     #[must_use]
124    |     ^^^^^^^^^^^
125 
- error: unused `X` that must be used
-   --> $DIR/unused_attributes-must_use.rs:103:5
- LL |     X;
-    |     ^^
-    |
- note: the lint level is defined here
- note: the lint level is defined here
-   --> $DIR/unused_attributes-must_use.rs:2:28
-    |
- LL | #![deny(unused_attributes, unused_must_use)]
+ error: aborting due to 20 previous errors
137 
137 
- error: unused `Y` that must be used
-   --> $DIR/unused_attributes-must_use.rs:104:5
-    |
- LL |     Y::Z;
- 
- error: unused `U` that must be used
-   --> $DIR/unused_attributes-must_use.rs:105:5
-    |
-    |
- LL |     U { unit: () };
- 
- 
- error: unused return value of `U::method` that must be used
-   --> $DIR/unused_attributes-must_use.rs:106:5
-    |
- LL |     U::method();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
- 
- error: unused return value of `foo` that must be used
-   --> $DIR/unused_attributes-must_use.rs:107:5
-   --> $DIR/unused_attributes-must_use.rs:107:5
-    |
- LL |     foo();
-    |     ^^^^^^
- 
- error: unused return value of `foreign_foo` that must be used
-   --> $DIR/unused_attributes-must_use.rs:110:9
-    |
- LL |         foreign_foo();
- 
- 
- error: unused return value of `Use::get_four` that must be used
-   --> $DIR/unused_attributes-must_use.rs:118:5
-    |
- LL |     ().get_four();
- 
- error: aborting due to 26 previous errors
- 
+ For more information about this error, try `rustc --explain E0658`.
---
To only update this specific test, also pass `--test-args lint/unused/unused_attributes-must_use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused_attributes-must_use.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused_attributes-must_use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused_attributes-must_use/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL | global_asm!("");
   |
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
   = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unused attribute `must_use`
  --> /checkout/src/test/ui/lint/unused/unused_attributes-must_use.rs:58:1
   |
   |
LL | #[must_use] //~ ERROR unused attribute
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused_attributes-must_use.rs:2:9
   |
   |
LL | #![deny(unused_attributes, unused_must_use)]
   |         ^^^^^^^^^^^^^^^^^
note: the built-in attribute `must_use` will be ignored, since it's applied to the macro invocation `global_asm`
   |
   |
LL | global_asm!("");


error: `#[must_use]` has no effect when applied to an extern crate
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a module
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a use
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a constant item
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a static item
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to an item
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a foreign module
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a type alias
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a type parameter
   |
   |
LL | fn qux<#[must_use] T>(_: T) {} //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to an item
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a trait alias
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a macro def
   |
   |
LL | #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a statement
   |
   |
LL |     #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a closure
   |
   |
LL |     let x = #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to an match arm
   |
   |
LL |         #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to an associated const
   |
   |
LL |     #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to an associated type
   |
   |
LL |     #[must_use] //~ ERROR `#[must_use]` has no effect


error: `#[must_use]` has no effect when applied to a foreign static item
   |
   |
LL |     #[must_use] //~ ERROR `#[must_use]` has no effect

error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0658`.
