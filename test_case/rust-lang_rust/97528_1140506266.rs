plain
........................................................................................ 2728/13289
........................................................................................ 2816/13289
........................................................................................ 2904/13289
..............................................i......................................... 2992/13289
............i.........................................................................F. 3080/13289
........FFFF......................................F..................................... 3168/13289
........................................................................................ 3344/13289
........................................................................................ 3432/13289
........................................................................................ 3520/13289
........................................................................................ 3608/13289
---
.............................................................i........i........i.....i.. 11792/13289
..........................i............................................................. 11880/13289
........................................................................................ 11968/13289
........................................................................................ 12056/13289
.............................................................F....F..................... 12144/13289
........................................................................................ 12320/13289
........................................................................................ 12408/13289
.................................i...................................................... 12496/13289
........................................................................................ 12584/13289
---

---- [ui] src/test/ui/derives/clone-debug-dead-code.rs stdout ----
diff of stderr:

16 LL | struct B { f: () }
18    |
- note: `B` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
- note: `B` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
+ note: `B` has derived impls for the traits `Clone` and `Clone`, but these are intentionally ignored during dead code analysis
21    |
22 LL | #[derive(Clone)]


42 LL | struct D { f: () }
44    |
- note: `D` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
+ note: `D` has derived impls for the traits `Clone` and `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
46   --> $DIR/clone-debug-dead-code.rs:17:10
46   --> $DIR/clone-debug-dead-code.rs:17:10
47    |
48 LL | #[derive(Debug,Clone)]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/clone-debug-dead-code/clone-debug-dead-code.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/clone-debug-dead-code.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/clone-debug-dead-code.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/clone-debug-dead-code" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/clone-debug-dead-code/auxiliary"
stdout: none
--- stderr -------------------------------
error: field is never read: `f`
   |
   |
LL | struct A { f: () }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:4:11
   |
   |
LL | #![forbid(dead_code)]

error: field is never read: `f`
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:10:12
   |
   |
LL | struct B { f: () }
   |
note: `B` has derived impls for the traits `Clone` and `Clone`, but these are intentionally ignored during dead code analysis
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:9:10
   |
   |
LL | #[derive(Clone)]
   |          ^^^^^
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: field is never read: `f`
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:14:12
   |
LL | struct C { f: () }
   |
note: `C` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:13:10
   |
   |
LL | #[derive(Debug)]
   |          ^^^^^
   = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

error: field is never read: `f`
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:18:12
   |
LL | struct D { f: () }
   |
note: `D` has derived impls for the traits `Clone` and `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:17:10
   |
   |
LL | #[derive(Debug,Clone)]
   |          ^^^^^ ^^^^^

error: field is never read: `f`
  --> /checkout/src/test/ui/derives/clone-debug-dead-code.rs:21:12
   |
   |
LL | struct E { f: () }

error: aborting due to 5 previous errors
------------------------------------------

---
+ LL |      x: Error
+    |      ^^^^^^^^ the trait `Clone` is not implemented for `Error`
+    |
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider annotating `Error` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
+ 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args derives/derives-span-Clone-enum-struct-variant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Clone-enum-struct-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-enum-struct-variant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-enum-struct-variant/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Error: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
...
...
LL |      x: Error //~ ERROR
   |      ^^^^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error[E0277]: the trait bound `Error: Clone` is not satisfied
error[E0277]: the trait bound `Error: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/derives-span-Clone-enum-struct-variant.rs:9:6
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
...
LL |      x: Error //~ ERROR
   |      ^^^^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 2 previous errors
---
+ LL |      Error
+    |      ^^^^^ the trait `Clone` is not implemented for `Error`
+    |
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider annotating `Error` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
+ 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args derives/derives-span-Clone-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Clone-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Error: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
...
...
LL |      Error //~ ERROR
   |      ^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error[E0277]: the trait bound `Error: Clone` is not satisfied
error[E0277]: the trait bound `Error: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/derives-span-Clone-enum.rs:9:6
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
...
LL |      Error //~ ERROR
   |      ^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 2 previous errors
---
+ LL |     Error
+    |     ^^^^^ the trait `Clone` is not implemented for `Error`
+    |
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider annotating `Error` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
+ 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args derives/derives-span-Clone-tuple-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Clone-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-tuple-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-tuple-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Error: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct Struct(
LL | struct Struct(
LL |     Error //~ ERROR
   |     ^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error[E0277]: the trait bound `Error: Clone` is not satisfied
error[E0277]: the trait bound `Error: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/derives-span-Clone-tuple-struct.rs:8:5
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct Struct(
LL |     Error //~ ERROR
   |     ^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 2 previous errors
---
+ LL |     x: Error
+    |     ^^^^^^^^ the trait `Clone` is not implemented for `Error`
+    |
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider annotating `Error` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
+ 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args derives/derives-span-Clone-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Clone-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Clone-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Error: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct Struct {
LL | struct Struct {
LL |     x: Error //~ ERROR
   |     ^^^^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error[E0277]: the trait bound `Error: Clone` is not satisfied
error[E0277]: the trait bound `Error: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/derives-span-Clone-struct.rs:8:5
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct Struct {
LL |     x: Error //~ ERROR
   |     ^^^^^^^^ the trait `Clone` is not implemented for `Error`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Error` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 2 previous errors
---
53 LL | #[derive(Clone)]
54    |
55 
- error: aborting due to 3 previous errors
+ error[E0277]: the trait bound `NoCloneOrEq: Clone` is not satisfied
+    |
+ LL | #[derive(Clone)]
+    |          ----- in this derive macro expansion
+ LL | struct C {
+ LL | struct C {
+ LL |     x: NoCloneOrEq
+    |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `NoCloneOrEq`
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider annotating `NoCloneOrEq` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
+ 
+ error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args derives/deriving-no-inner-impl-error-message.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `NoCloneOrEq`
   |
   |
LL | #[derive(PartialEq)]
LL | struct E {
LL | struct E {
LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `NoCloneOrEq`
   |
   |
LL | struct NoCloneOrEq;
   | ^^^^^^^^^^^^^^^^^^^ must implement `PartialEq<_>`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `NoCloneOrEq` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]


error[E0369]: binary operation `!=` cannot be applied to type `NoCloneOrEq`
   |
   |
LL | #[derive(PartialEq)]
LL | struct E {
LL | struct E {
LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `NoCloneOrEq`
   |
   |
LL | struct NoCloneOrEq;
   | ^^^^^^^^^^^^^^^^^^^ must implement `PartialEq<_>`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `NoCloneOrEq` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]


error[E0277]: the trait bound `NoCloneOrEq: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct C {
LL | struct C {
LL |     x: NoCloneOrEq
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `NoCloneOrEq`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `NoCloneOrEq` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0277]: the trait bound `NoCloneOrEq: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct C {
LL | struct C {
LL |     x: NoCloneOrEq
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `NoCloneOrEq`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `NoCloneOrEq` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args lint/dead-code/unused-variant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/dead-code/unused-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/unused-variant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/unused-variant/auxiliary"
stdout: none
--- stderr -------------------------------
error: variant is never constructed: `Variant1`
   |
   |
LL |     Variant1, //~ ERROR: variant is never constructed
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/dead-code/unused-variant.rs:1:9
   |
---
+   --> $DIR/issue-71136.rs:5:5
+    |
+ LL | #[derive(Clone)]
+    |          ----- in this derive macro expansion
+ LL | struct FooHolster {
+ LL |     the_foos: Vec<Foo>,
+    |     ^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `Foo`
+    = note: required because of the requirements on the impl of `Clone` for `Vec<Foo>`
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider annotating `Foo` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
+ 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args traits/issue-71136.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-71136.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-71136" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-71136/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Foo: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
   |          ----- in this derive macro expansion
LL | struct FooHolster {
LL |     the_foos: Vec<Foo>, //~ERROR Clone
   |     ^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `Foo`
   = note: required because of the requirements on the impl of `Clone` for `Vec<Foo>`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Foo` with `#[derive(Clone)]`
   |
---
  --> /checkout/src/test/ui/traits/issue-71136.rs:5:5
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct FooHolster {
LL |     the_foos: Vec<Foo>, //~ERROR Clone
   |     ^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `Foo`
   = note: required because of the requirements on the impl of `Clone` for `Vec<Foo>`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Foo` with `#[derive(Clone)]`
   |
---

---- [ui] src/test/ui/traits/issue-79458.rs stdout ----
diff of stderr:

14    = note: `Clone` is implemented for `&T`, but not for `&mut T`
16 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0277]: the trait bound `&'a mut T: Clone` is not satisfied
+    |
+ LL | #[derive(Clone)]
+    |          ----- in this derive macro expansion
+    |          ----- in this derive macro expansion
+ LL | struct Foo<'a, T> {
+ LL |     bar: &'a mut T
+    |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&'a mut T`
+    = help: the following other types implement trait `Clone`:
+              &T
+              *const T
+              *mut T
+              *mut T
+    = note: `Clone` is implemented for `&'a T`, but not for `&'a mut T`
+ 
+ error: aborting due to 2 previous errors
18 
19 For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args traits/issue-79458.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-79458.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-79458" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-79458/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&mut T: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
   |          ----- in this derive macro expansion
LL | struct Foo<'a, T> {
LL |     bar: &'a mut T
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&mut T`
   = help: the following other types implement trait `Clone`:
             &T
             *const T
             *mut T
             *mut T
   = note: `Clone` is implemented for `&T`, but not for `&mut T`


error[E0277]: the trait bound `&'a mut T: Clone` is not satisfied
   |
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
   |          ----- in this derive macro expansion
LL | struct Foo<'a, T> {
LL |     bar: &'a mut T
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&'a mut T`
   = help: the following other types implement trait `Clone`:
             &T
             *const T
             *mut T
             *mut T
   = note: `Clone` is implemented for `&'a T`, but not for `&'a mut T`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
