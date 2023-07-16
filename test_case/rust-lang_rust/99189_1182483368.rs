plain

---- [ui] src/test/ui/const-generics/issues/issue-90318.rs stdout ----
diff of stderr:

18 note: impl defined here, but it is not `const`
19   --> $SRC_DIR/core/src/any.rs:LL:COL
20    |
- LL | #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
-    |                       ^^^^^^^^^
+ LL | impl PartialEq for TypeId {
23    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
23    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
26 error: overly complex generic constant
27   --> $DIR/issue-90318.rs:22:8


43 note: impl defined here, but it is not `const`
44   --> $SRC_DIR/core/src/any.rs:LL:COL
45    |
- LL | #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
-    |                       ^^^^^^^^^
+ LL | impl PartialEq for TypeId {
48    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
48    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
51 error: aborting due to 4 previous errors
52 

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-90318.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
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
LL | impl PartialEq for TypeId {
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:22:8
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:22:8
   |
LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
   |        ^^-----------------^^^^^^^^^^^^^^^^^^^^^^^^
   |          borrowing is not supported in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error[E0015]: cannot call non-const operator in constants
  --> /checkout/src/test/ui/const-generics/issues/issue-90318.rs:22:10
   |
   |
LL |     If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
   |
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl PartialEq for TypeId {
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

