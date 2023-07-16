plain
........................................................................................ 2464/14044
........................................................................................ 2552/14044
........................................................................................ 2640/14044
........................................................................................ 2728/14044
..................................................................F...............F..... 2816/14044
..........................................................................F............. 2992/14044
..........................................................................F............. 2992/14044
...F...................................F...................F............................ 3080/14044
..F..................................................................................... 3168/14044
........................................................................................ 3344/14044
........................................................................................ 3432/14044
....................................................iiiii............................... 3520/14044
........................................................................................ 3608/14044
---
diff of stderr:

22    |                                      ^^^^^^^^^^^
23    |
24    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
25    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
27 error[E0010]: allocations are not allowed in statics


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/check-static-values-constraints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-static-values-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary"
stdout: none
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
--- stderr -------------------------------
error[E0493]: destructor of `SafeStruct` cannot be evaluated at compile-time
   |
   |
LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
   |  ___________________________________________^
LL | | //~^ ERROR destructor of
LL | |                                                      field2: SafeEnum::Variant1}};
   | |                                                                                ^- value is dropped here
   |                                                                                  the destructor for this type cannot be evaluated in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;
   |                                 ^^^^^^^^^^^ allocation not allowed in statics

error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in statics
   |
   |
LL |     field2: SafeEnum::Variant4("str".to_string())
   |
   = note: calls in statics are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
   = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:94:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:99:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:100:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:106:5
   |
LL |     box 3;
LL |     box 3;
   |     ^^^^^ allocation not allowed in statics

error[E0507]: cannot move out of static item `x`
  --> /checkout/src/test/ui/check-static-values-constraints.rs:110:45
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                             ^ move occurs because `x` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider borrowing here
   |
   |
LL |     let y = { static x: Box<isize> = box 3; &x };

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:110:38
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                      ^^^^^ allocation not allowed in statics
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0010, E0015, E0493, E0507.
For more information about an error, try `rustc --explain E0010`.
For more information about an error, try `rustc --explain E0010`.
------------------------------------------


---- [ui] src/test/ui/const-generics/issue-93647.rs stdout ----
diff of stderr:

6    |
7    = note: closures need an RFC before allowed to be called in constants
8    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-93647/issue-93647.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issue-93647.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-93647.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-93647" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-93647/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const closure in constants
   |
   |
LL |     (||1usize)()
   |
   |
   = note: closures need an RFC before allowed to be called in constants
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-fn-error.rs stdout ----
diff of stderr:

22 note: impl defined here, but it is not `const`
23   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
24    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
26 error[E0658]: mutable references are not allowed in constant functions
27   --> $DIR/const-fn-error.rs:5:14

39    |              ^^^^
39    |              ^^^^
40    |
41    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
43 error: aborting due to 4 previous errors
44 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/const-fn-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-fn-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/auxiliary"
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
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/collect.rs:267:1
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
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
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0015, E0658.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.
------------------------------------------


---- [ui] src/test/ui/consts/const-for.rs stdout ----
diff of stderr:

7 note: impl defined here, but it is not `const`
8   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
9    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
10 
11 error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants

15    |              ^^^^
16    |
17    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
---
To only update this specific test, also pass `--test-args consts/const-for.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-for.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot convert `std::ops::Range<i32>` into an iterator in constants
   |
LL |     for _ in 0..5 {}
   |              ^^^^
   |
   |
note: impl defined here, but it is not `const`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/collect.rs:267:1
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable

error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants
   |
LL |     for _ in 0..5 {}
   |              ^^^^
   |
---
---- [ui] src/test/ui/consts/invalid-inline-const-in-match-arm.rs stdout ----
diff of stderr:

6    |
7    = note: closures need an RFC before allowed to be called in constants
8    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-inline-const-in-match-arm/invalid-inline-const-in-match-arm.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/invalid-inline-const-in-match-arm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-inline-const-in-match-arm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-inline-const-in-match-arm" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-inline-const-in-match-arm/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const closure in constants
   |
   |
LL |         const { (|| {})() } => {}
   |
   |
   = note: closures need an RFC before allowed to be called in constants
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/issue-28113.rs stdout ----
diff of stderr:

6    |
7    = note: closures need an RFC before allowed to be called in constants
8    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-28113/issue-28113.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-28113.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-28113.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-28113" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-28113/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const closure in constants
   |
   |
LL |     || -> u8 { 5 }()
   |
   |
   = note: closures need an RFC before allowed to be called in constants
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/issue-56164.rs stdout ----
diff of stderr:

6    |
7    = note: closures need an RFC before allowed to be called in constant functions
8    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
10 error: function pointer calls are not allowed in constant functions
11   --> $DIR/issue-56164.rs:5:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164/issue-56164.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-56164.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-56164.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const closure in constant functions
   |
   |
LL | const fn foo() { (||{})() }
   |
   |
   = note: closures need an RFC before allowed to be called in constant functions
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error: function pointer calls are not allowed in constant functions
  --> /checkout/src/test/ui/consts/issue-56164.rs:5:5
   |
LL |     input()
LL |     input()
   |     ^^^^^^^

note: erroneous constant used
  --> /checkout/src/test/ui/consts/issue-56164.rs:1:18
   |
LL | const fn foo() { (||{})() }

note: erroneous constant used
  --> /checkout/src/test/ui/consts/issue-56164.rs:1:18
   |
   |
LL | const fn foo() { (||{})() }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0015`.
For more information about this error, try `rustc --explain E0015`.
------------------------------------------


---- [ui] src/test/ui/consts/issue-68542-closure-in-array-len.rs stdout ----
diff of stderr:

6    |
7    = note: closures need an RFC before allowed to be called in constants
8    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-68542-closure-in-array-len/issue-68542-closure-in-array-len.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-68542-closure-in-array-len.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-68542-closure-in-array-len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-68542-closure-in-array-len" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-68542-closure-in-array-len/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const closure in constants
   |
   |
LL |     a: [(); (|| { 0 })()] //~ ERROR cannot call non-const closure
   |
   |
   = note: closures need an RFC before allowed to be called in constants
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/issue-90870.rs stdout ----
diff of stderr:

5    |     ^^^^^^
6    |
7    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
9    |
9    |
10 LL |     *a == *b
17    |     ^^^^^^
18    |
18    |
19    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
21    |
21    |
22 LL |     ****a == ****b
29    |            ^^^^^^
30    |
30    |
31    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
33    |
33    |
34 LL |         if *l == *r {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-90870/issue-90870.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-90870.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-90870.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-90870" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-90870/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const operator in constant functions
   |
LL |     a == b
   |     ^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
   |
   |
LL |     *a == *b

error[E0015]: cannot call non-const operator in constant functions
  --> /checkout/src/test/ui/consts/issue-90870.rs:14:5
   |
   |
LL |     a == b
   |     ^^^^^^
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
   |
   |
LL |     ****a == ****b

error[E0015]: cannot call non-const operator in constant functions
  --> /checkout/src/test/ui/consts/issue-90870.rs:21:12
   |
   |
LL |         if l == r {
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
   |
   |
LL |         if *l == *r {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0015`.
---
6    |
7    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
8 
9 error[E0015]: cannot call non-const fn `<Dim3 as Dim>::dim` in constants

13    |               ^^^^^^^^^^^
14    |
15    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
---
To only update this specific test, also pass `--test-args issues/issue-39559-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39559-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39559-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39559-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const fn `<Dim3 as Dim>::dim` in constants
   |
   |
LL |     let array: [usize; Dim3::dim()]
   |
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable


error[E0015]: cannot call non-const fn `<Dim3 as Dim>::dim` in constants
   |
   |
LL |         = [0; Dim3::dim()];
   |
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable

---

---- [ui] src/test/ui/never_type/issue-52443.rs stdout ----
diff of stderr:

47 note: impl defined here, but it is not `const`
48   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
49    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
51 error[E0658]: mutable references are not allowed in constants
52   --> $DIR/issue-52443.rs:9:21

64    |                     ^^^^^^^^
---
To only update this specific test, also pass `--test-args never_type/issue-52443.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-52443.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443/auxiliary"
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
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/collect.rs:267:1
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
error[E0658]: mutable references are not allowed in constants
  --> /checkout/src/test/ui/never_type/issue-52443.rs:9:21
   |
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
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable

