plain

---- [ui] ui/consts/const-for.rs stdout ----
diff of stderr:

7 note: impl defined here, but it is not `const`
8   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
9    |
- LL | impl<I: Iterator> IntoIterator for I {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
12    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
13 
13 
14 error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for/const-for.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-for.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-for.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot convert `std::ops::Range<i32>` into an iterator in constants
   |
LL |     for _ in 0..5 {}
   |              ^^^^
   |
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl<I: ~const Iterator> const IntoIterator for I {
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants
   |
LL |     for _ in 0..5 {}
   |              ^^^^
   |
---

---- [ui] ui/never_type/issue-52443.rs stdout ----
diff of stderr:

47 note: impl defined here, but it is not `const`
48   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
49    |
- LL | impl<I: Iterator> IntoIterator for I {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
52    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
53 
54 error[E0658]: mutable references are not allowed in constants

---
To only update this specific test, also pass `--test-args never_type/issue-52443.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-52443.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     [(); {while true {break}; 0}];
   |           ^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default

error[E0658]: `for` is not allowed in a `const`
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/issue-52443.rs:2:10
   |
LL |     [(); & { loop { continue } } ]; //~ ERROR mismatched types
   |
   = note:   expected type `usize`
           found reference `&_`
help: consider removing the borrow
help: consider removing the borrow
   |
LL -     [(); & { loop { continue } } ]; //~ ERROR mismatched types
LL +     [(); { loop { continue } } ]; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/issue-52443.rs:4:17
   |
   |
LL |     [(); loop { break }]; //~ ERROR mismatched types
   |                 |
   |                 expected `usize`, found `()`
   |                 expected `usize`, found `()`
   |                 help: give it a value of the expected type: `break 42`

error[E0015]: cannot convert `RangeFrom<usize>` into an iterator in constants
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl<I: ~const Iterator> const IntoIterator for I {
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error[E0658]: mutable references are not allowed in constants
  --> /checkout/src/test/ui/never_type/issue-52443.rs:9:21
  --> /checkout/src/test/ui/never_type/issue-52443.rs:9:21
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0015]: cannot call non-const fn `<RangeFrom<usize> as Iterator>::next` in constants
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error: aborting due to 6 previous errors; 1 warning emitted
