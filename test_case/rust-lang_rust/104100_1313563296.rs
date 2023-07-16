plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
---
........................................................................................ 2464/13840
........................................................................................ 2552/13840
........................................................................................ 2640/13840
........................................................................................ 2728/13840
............F...............F........................................................... 2816/13840
........................................................................................ 2992/13840
........................................................................................ 3080/13840
..........................................i............................................. 3168/13840
.........i.............................................................................. 3256/13840
---

---- [ui] src/test/ui/consts/const-fn-error.rs stdout ----
diff of stderr:

22 note: impl defined here, but it is not `const`
23   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
24    |
- LL | impl<I: Iterator> const IntoIterator for I {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
27    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
29 error[E0658]: mutable references are not allowed in constant functions


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/const-fn-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-fn-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `for` is not allowed in a `const fn`
   |
LL | /     for i in 0..x {
LL | /     for i in 0..x {
LL | |         //~^ ERROR cannot convert
LL | |         //~| ERROR `for` is not allowed in a `const fn`
LL | |         //~| ERROR mutable references are not allowed in constant functions
LL | |         //~| ERROR cannot call non-const fn
LL | |         sum += i;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0015]: cannot convert `std::ops::Range<usize>` into an iterator in constant functions
  --> /checkout/src/test/ui/consts/const-fn-error.rs:5:14
   |
LL |     for i in 0..x {
   |              ^^^^
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl<I: ~const Iterator> const IntoIterator for I {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/const-fn-error.rs:5:14
   |
LL |     for i in 0..x {
LL |     for i in 0..x {
   |              ^^^^
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0015]: cannot call non-const fn `<std::ops::Range<usize> as Iterator>::next` in constant functions
   |
LL |     for i in 0..x {
   |              ^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0015, E0658.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.
------------------------------------------


---- [ui] src/test/ui/consts/const-for.rs stdout ----
diff of stderr:

7 note: impl defined here, but it is not `const`
8   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
9    |
- LL | impl<I: Iterator> const IntoIterator for I {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
12    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
13 
13 
14 error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for/const-for.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-for.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-for.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for/auxiliary"
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

---- [ui] src/test/ui/never_type/issue-52443.rs stdout ----
diff of stderr:

47 note: impl defined here, but it is not `const`
48   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
49    |
- LL | impl<I: Iterator> const IntoIterator for I {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
52    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
53 
54 error[E0658]: mutable references are not allowed in constants

---
To only update this specific test, also pass `--test-args never_type/issue-52443.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-52443.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     [(); {while true {break}; 0}];
   |           ^^^^^^^^^^ help: use `loop`
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
