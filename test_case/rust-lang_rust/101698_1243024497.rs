plain

---- [ui] src/test/ui/const-generics/issues/issue-90318.rs stdout ----
diff of stderr:

- error[E0277]: can't compare `TypeId` with `_` in const contexts
-   --> $DIR/issue-90318.rs:14:28
+ error: overly complex generic constant
3    |
3    |
4 LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
-    |                            ^^ no implementation for `TypeId == _`
+    |        ^^-----------------^^^^^^^^^^^^^^^^^^^^^^^^
+    |          borrowing is not supported in generic constants
6    |
6    |
-    = help: the trait `~const PartialEq<_>` is not implemented for `TypeId`
- note: the trait `PartialEq<_>` is implemented for `TypeId`, but that implementation is not `const`
-   --> $DIR/issue-90318.rs:14:28
+    = help: consider moving this anonymous constant into a `const` function
+ 
+ error[E0015]: cannot call non-const operator in constants
+   --> $DIR/issue-90318.rs:14:10
10    |
10    |
11 LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
+    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: impl defined here, but it is not `const`
+   --> $SRC_DIR/core/src/any.rs:LL:COL
+    |
+ LL | impl const PartialEq for TypeId {
+    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
13 
13 
- error[E0277]: can't compare `TypeId` with `_` in const contexts
-   --> $DIR/issue-90318.rs:21:28
+ error: overly complex generic constant
16    |
16    |
17 LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
-    |                            ^^ no implementation for `TypeId == _`
+    |        ^^-----------------^^^^^^^^^^^^^^^^^^^^^^^^
+    |          borrowing is not supported in generic constants
19    |
19    |
-    = help: the trait `~const PartialEq<_>` is not implemented for `TypeId`
- note: the trait `PartialEq<_>` is implemented for `TypeId`, but that implementation is not `const`
-   --> $DIR/issue-90318.rs:21:28
+    = help: consider moving this anonymous constant into a `const` function
+ 
+ error[E0015]: cannot call non-const operator in constants
+   --> $DIR/issue-90318.rs:21:10
23    |
23    |
24 LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
+    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: impl defined here, but it is not `const`
+   --> $SRC_DIR/core/src/any.rs:LL:COL
+    |
+ LL | impl const PartialEq for TypeId {
+    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
26 
- error: aborting due to 2 previous errors
+ error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-90318.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-90318.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-90318" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-90318/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
   |        ^^-----------------^^^^^^^^^^^^^^^^^^^^^^^^
   |          borrowing is not supported in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error[E0015]: cannot call non-const operator in constants
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:14:10
   |
   |
LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
   |
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl const PartialEq for TypeId {
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:21:8
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:21:8
   |
LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
   |        ^^-----------------^^^^^^^^^^^^^^^^^^^^^^^^
   |          borrowing is not supported in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error[E0015]: cannot call non-const operator in constants
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:21:10
   |
   |
LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
   |
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl const PartialEq for TypeId {
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error: aborting due to 4 previous errors

---
---- [ui] src/test/ui/consts/issue-73976-monomorphic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-73976-monomorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic/auxiliary"
stdout: none
--- stderr -------------------------------
error: to use a constant of type `TypeId` in a pattern, `TypeId` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<usize>::VALUE)

error: aborting due to previous error
------------------------------------------

