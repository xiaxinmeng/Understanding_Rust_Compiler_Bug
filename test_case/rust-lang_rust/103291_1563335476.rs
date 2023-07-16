plain
---- [ui] tests/ui/const-generics/generic_const_exprs/typeid-equality-by-subtyping.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/generic_const_exprs/typeid-equality-by-subtyping.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/typeid-equality-by-subtyping" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/typeid-equality-by-subtyping/auxiliary"
stdout: none
--- stderr -------------------------------
error: to use a constant of type `TypeId` in a pattern, `TypeId` must be annotated with `#[derive(PartialEq, Eq)]`
  --> fake-test-src-base/const-generics/generic_const_exprs/typeid-equality-by-subtyping.rs:18:9
   |
LL |         WHAT_A_TYPE => 0,
   |
   |
   = note: the traits must be derived, manual `impl`s are not sufficient
   = note: see https://doc.rust-lang.org/stable/std/marker/trait.StructuralEq.html for details
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/consts/const_cmp_type_id.rs stdout ----
diff of stderr:

20 note: impl defined here, but it is not `const`
21   --> $SRC_DIR/core/src/any.rs:LL:COL
22    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
25 error[E0277]: can't compare `TypeId` with `TypeId` in const contexts


44 note: impl defined here, but it is not `const`
45   --> $SRC_DIR/core/src/any.rs:LL:COL
46    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
48 
49 error[E0277]: can't compare `TypeId` with `TypeId` in const contexts


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_cmp_type_id/const_cmp_type_id.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_cmp_type_id/const_cmp_type_id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_cmp_type_id.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const_cmp_type_id.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_cmp_type_id" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_cmp_type_id/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: can't compare `TypeId` with `TypeId` in const contexts
  --> fake-test-src-base/consts/const_cmp_type_id.rs:8:13
   |
LL |     assert!(TypeId::of::<u8>() == TypeId::of::<u8>());
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `TypeId == TypeId`
   |
   = help: the trait `~const PartialEq` is not implemented for `TypeId`
note: the trait `PartialEq` is implemented for `TypeId`, but that implementation is not `const`
  --> fake-test-src-base/consts/const_cmp_type_id.rs:8:13
   |
LL |     assert!(TypeId::of::<u8>() == TypeId::of::<u8>());

error[E0015]: cannot call non-const operator in constant functions
  --> fake-test-src-base/consts/const_cmp_type_id.rs:8:13
   |
   |
LL |     assert!(TypeId::of::<u8>() == TypeId::of::<u8>());
   |
   |
note: impl defined here, but it is not `const`
  --> /rustc/FAKE_PREFIX/library/core/src/any.rs:672:1
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0277]: can't compare `TypeId` with `TypeId` in const contexts
  --> fake-test-src-base/consts/const_cmp_type_id.rs:9:13
   |
LL |     assert!(TypeId::of::<()>() != TypeId::of::<u8>());
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `TypeId == TypeId`
   |
   = help: the trait `~const PartialEq` is not implemented for `TypeId`
note: the trait `PartialEq` is implemented for `TypeId`, but that implementation is not `const`
  --> fake-test-src-base/consts/const_cmp_type_id.rs:9:13
   |
LL |     assert!(TypeId::of::<()>() != TypeId::of::<u8>());

error[E0015]: cannot call non-const operator in constant functions
  --> fake-test-src-base/consts/const_cmp_type_id.rs:9:13
   |
   |
LL |     assert!(TypeId::of::<()>() != TypeId::of::<u8>());
   |
   |
note: impl defined here, but it is not `const`
  --> /rustc/FAKE_PREFIX/library/core/src/any.rs:672:1
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0277]: can't compare `TypeId` with `TypeId` in const contexts
  --> fake-test-src-base/consts/const_cmp_type_id.rs:10:22
   |
LL |     const _A: bool = TypeId::of::<u8>() < TypeId::of::<u16>();
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `TypeId < TypeId` and `TypeId > TypeId`
   |
   = help: the trait `~const PartialOrd` is not implemented for `TypeId`
note: the trait `PartialOrd` is implemented for `TypeId`, but that implementation is not `const`
  --> fake-test-src-base/consts/const_cmp_type_id.rs:10:22
   |
LL |     const _A: bool = TypeId::of::<u8>() < TypeId::of::<u16>();

error[E0015]: cannot call non-const operator in constants
  --> fake-test-src-base/consts/const_cmp_type_id.rs:10:22
   |
   |
LL |     const _A: bool = TypeId::of::<u8>() < TypeId::of::<u16>();
   |
   |
note: impl defined here, but it is not `const`
  --> /rustc/FAKE_PREFIX/library/core/src/any.rs:665:40
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.
------------------------------------------


---- [ui] tests/ui/consts/issue-73976-monomorphic.rs stdout ----
diff of stderr:

20 note: impl defined here, but it is not `const`
21   --> $SRC_DIR/core/src/any.rs:LL:COL
22    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic/issue-73976-monomorphic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-73976-monomorphic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-73976-monomorphic.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-monomorphic/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: can't compare `TypeId` with `TypeId` in const contexts
  --> fake-test-src-base/consts/issue-73976-monomorphic.rs:21:5
   |
LL |     GetTypeId::<T>::VALUE == GetTypeId::<usize>::VALUE
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `TypeId == TypeId`
   |
   = help: the trait `~const PartialEq` is not implemented for `TypeId`
note: the trait `PartialEq` is implemented for `TypeId`, but that implementation is not `const`
  --> fake-test-src-base/consts/issue-73976-monomorphic.rs:21:5
   |
LL |     GetTypeId::<T>::VALUE == GetTypeId::<usize>::VALUE

error[E0015]: cannot call non-const operator in constant functions
  --> fake-test-src-base/consts/issue-73976-monomorphic.rs:21:5
   |
   |
LL |     GetTypeId::<T>::VALUE == GetTypeId::<usize>::VALUE
   |
   |
note: impl defined here, but it is not `const`
  --> /rustc/FAKE_PREFIX/library/core/src/any.rs:672:1
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
