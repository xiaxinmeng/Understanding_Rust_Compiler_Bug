plain

---- [ui] ui/cast/casts-issue-46365.rs stdout ----
diff of stderr:

4 LL |     ipsum: Ipsum
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0271]: type mismatch resolving `<Lorem as Pointee>::Metadata == ()`
+    |
+    |
+ LL |     let _foo: *mut Lorem = core::ptr::null_mut(); // no error here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    = note:    expected unit type `()`
+    = note:    expected unit type `()`
+            found associated type `<Lorem as Pointee>::Metadata`
+    = help: consider constraining the associated type `<Lorem as Pointee>::Metadata` to `()`
+    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
+ note: required by a bound in `null_mut`
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+    |
+ LL | pub const fn null_mut<T: ?Sized + Thin>() -> *mut T {
+    |                                   ^^^^ required by this bound in `null_mut`
- For more information about this error, try `rustc --explain E0412`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0271, E0412.
---
To only update this specific test, also pass `--test-args cast/casts-issue-46365.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cast/casts-issue-46365.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/casts-issue-46365" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/casts-issue-46365/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `Ipsum` in this scope
   |
   |
LL |     ipsum: Ipsum //~ ERROR cannot find type `Ipsum`


error[E0271]: type mismatch resolving `<Lorem as Pointee>::Metadata == ()`
   |
   |
LL |     let _foo: *mut Lorem = core::ptr::null_mut(); // no error here
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<Lorem as Pointee>::Metadata`
   = help: consider constraining the associated type `<Lorem as Pointee>::Metadata` to `()`
note: required by a bound in `null_mut`
  --> /checkout/library/core/src/ptr/mod.rs:278:35
   |
   |
LL | pub const fn null_mut<T: ?Sized + Thin>() -> *mut T {
   |                                   ^^^^ required by this bound in `null_mut`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0412.
For more information about an error, try `rustc --explain E0271`.
