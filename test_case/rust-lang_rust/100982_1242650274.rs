plain
........................................................................................ 2376/13507
........................................................................................ 2464/13507
........................................................................................ 2552/13507
........................................................................................ 2640/13507
.......................F...........F.F.................................................. 2728/13507
......................................................................................F. 2816/13507
........................................................................................ 2992/13507
.............................................i.......................................... 3080/13507
...........i............................................................................ 3168/13507
........................................................................................ 3256/13507
---
........................................................................................ 6160/13507
........................................................................................ 6248/13507
........................................................................................ 6336/13507
................................................................................i....... 6424/13507
..............................F.................F....................................... 6512/13507
........................................................................................ 6688/13507
.....................................................i.................................. 6776/13507
......................ii.ii........i....i............................................... 6864/13507
................i....................................................................... 6952/13507
---

-    |              ^^^^ `std::ops::Range<usize>` is not an iterator
+    |              ^^^^
19    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<usize>`
- note: the trait `Iterator` is implemented for `std::ops::Range<usize>`, but that implementation is not `const`
+ note: impl defined here, but it is not `const`
+   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+    |
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+ error[E0658]: mutable references are not allowed in constant functions
22   --> $DIR/const-fn-error.rs:5:14
23    |
24 LL |     for i in 0..x {
24 LL |     for i in 0..x {

25    |              ^^^^
-    = note: required for `std::ops::Range<usize>` to implement `~const IntoIterator`
- help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
28    |
- LL | const fn f(x: usize) -> usize where std::ops::Range<usize>: ~const Iterator {
+    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
+    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
31 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0015]: cannot call non-const fn `<std::ops::Range<usize> as Iterator>::next` in constant functions
+    |
+ LL |     for i in 0..x {
+    |              ^^^^
+    |
+    |
+    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
- Some errors have detailed explanations: E0277, E0658.
- For more information about an error, try `rustc --explain E0277`.
+ error: aborting due to 4 previous errors
+ 
---
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
LL | |         //~^ ERROR the trait bound
LL | |         //~| ERROR `for` is not allowed in a `const fn`
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
---

7    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
8    = help: add `#![feature(const_for)]` to the crate attributes to enable
9 
- error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
-   --> $DIR/const-for-feature-gate.rs:4:14
- LL |     for _ in 0..5 {}
-    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
-    |
-    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
- note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
-   --> $DIR/const-for-feature-gate.rs:4:14
- LL |     for _ in 0..5 {}
-    |              ^^^^
-    |              ^^^^
-    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0277, E0658.
- For more information about an error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args consts/const-for-feature-gate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-for-feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for-feature-gate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for-feature-gate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
LL |     for _ in 0..5 {}
   |     ^^^^^^^^^^^^^^^^
   |
---

---- [ui] src/test/ui/consts/const-for.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
+ error[E0015]: cannot convert `std::ops::Range<i32>` into an iterator in constants
3    |
4 LL |     for _ in 0..5 {}

-    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
-    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
+    |              ^^^^
6    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
- note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
+ note: impl defined here, but it is not `const`
+   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+    |
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
+    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+ 
+ 
+ error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants
10    |
11 LL |     for _ in 0..5 {}

12    |              ^^^^
12    |              ^^^^
-    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
+    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
14 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
---
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

22    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
23    = help: add `#![feature(const_for)]` to the crate attributes to enable
24 
- error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
-   --> $DIR/loop.rs:53:14
- LL |     for i in 0..4 {
-    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
-    |
-    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
- note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
-   --> $DIR/loop.rs:53:14
- LL |     for i in 0..4 {
-    |              ^^^^
-    |              ^^^^
-    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
+ error: aborting due to 2 previous errors
38 
- error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
-   --> $DIR/loop.rs:58:14
- LL |     for i in 0..4 {
-    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
-    |
-    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
- note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
-   --> $DIR/loop.rs:58:14
- LL |     for i in 0..4 {
-    |              ^^^^
-    |              ^^^^
-    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
- error: aborting due to 4 previous errors
- 
- Some errors have detailed explanations: E0277, E0658.
- For more information about an error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args consts/control-flow/loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/loop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
   |
LL | /     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
LL | |         //~^ ERROR the trait bound
LL | |         x += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0658]: `for` is not allowed in a `const`
   |
   |
LL | /     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
LL | |         //~^ ERROR the trait bound
LL | |         x += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable

---

7    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
8    = help: add `#![feature(const_for)]` to the crate attributes to enable
9 
- error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
-   --> $DIR/issue-50582.rs:2:29
-    |
- LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
-    |                             ^^^^ `std::ops::Range<{integer}>` is not an iterator
-    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
- note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
-   --> $DIR/issue-50582.rs:2:29
-    |
- LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
-    |                             ^^^^
-    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
- 
24 error[E0277]: cannot add `()` to `{integer}` in const contexts
26    |


39              <&'a isize as Add<isize>>
41 
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
43 
---
To only update this specific test, also pass `--test-args issues/issue-50582.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50582.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
   |
LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0277]: cannot add `()` to `{integer}` in const contexts
   |
   |
LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
   |                  ^ no implementation for `{integer} + ()`
   |
   = help: the trait `~const Add<()>` is not implemented for `{integer}`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a f32 as Add<f32>>
             <&'a f64 as Add<f64>>
             <&'a i128 as Add<i128>>
             <&'a i16 as Add<i16>>
             <&'a i32 as Add<i32>>
             <&'a i64 as Add<i64>>
             <&'a i8 as Add<i8>>
             <&'a isize as Add<isize>>

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
---

7    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
8    = help: add `#![feature(const_for)]` to the crate attributes to enable
9 
- error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
-   --> $DIR/issue-50585.rs:2:27
-    |
- LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
-    |                           ^^^^ `std::ops::Range<{integer}>` is not an iterator
-    |
-    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
- note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
-   --> $DIR/issue-50585.rs:2:27
-    |
- LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
-    |                           ^^^^
-    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
24 error[E0308]: mismatched types
25   --> $DIR/issue-50585.rs:2:18
26    |


27 LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
29 
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
31 
---
To only update this specific test, also pass `--test-args issues/issue-50585.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50585.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-50585.rs:2:18
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0658.
---
---- [ui] src/test/ui/never_type/issue-52443.rs stdout ----
diff of stderr:

38    |                 expected `usize`, found `()`
39    |                 help: give it a value of the expected type: `break 42`
40 
- error[E0277]: the trait bound `RangeFrom<usize>: Iterator` is not satisfied
+ error[E0015]: cannot convert `RangeFrom<usize>` into an iterator in constants
43    |
43    |
44 LL |     [(); { for _ in 0usize.. {}; 0}];
-    |                     ^^^^^^^^ `RangeFrom<usize>` is not an iterator
+    |                     ^^^^^^^^
46    |
46    |
-    = help: the trait `~const Iterator` is not implemented for `RangeFrom<usize>`
- note: the trait `Iterator` is implemented for `RangeFrom<usize>`, but that implementation is not `const`
+ note: impl defined here, but it is not `const`
+   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+    |
+ LL | impl<I: ~const Iterator> const IntoIterator for I {
+    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+ 
+ error[E0658]: mutable references are not allowed in constants
49   --> $DIR/issue-52443.rs:9:21
49   --> $DIR/issue-52443.rs:9:21
50    |
51 LL |     [(); { for _ in 0usize.. {}; 0}];
52    |                     ^^^^^^^^
52    |                     ^^^^^^^^
-    = note: required for `RangeFrom<usize>` to implement `~const IntoIterator`
- help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
55    |
- LL | fn main() where RangeFrom<usize>: ~const Iterator {
+    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
+    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
58 
- error: aborting due to 4 previous errors; 1 warning emitted
- error: aborting due to 4 previous errors; 1 warning emitted
+ error[E0015]: cannot call non-const fn `<RangeFrom<usize> as Iterator>::next` in constants
+    |
+    |
+ LL |     [(); { for _ in 0usize.. {}; 0}];
+    |
+    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
60 
- Some errors have detailed explanations: E0277, E0308, E0658.
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
---

---- [ui] src/test/ui/ufcs/ufcs-qpath-self-mismatch.rs stdout ----
diff of stderr:

18              <&'a isize as Add<isize>>
20 
20 
+ error[E0277]: cannot add `u32` to `i32`
+   --> $DIR/ufcs-qpath-self-mismatch.rs:4:5
+    |
+ LL |     <i32 as Add<u32>>::add(1, 2);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `i32 + u32`
+    = help: the trait `Add<u32>` is not implemented for `i32`
+    = help: the trait `Add<u32>` is not implemented for `i32`
+    = help: the following other types implement trait `Add<Rhs>`:
+              <&'a f32 as Add<f32>>
+              <&'a f64 as Add<f64>>
+              <&'a i128 as Add<i128>>
+              <&'a i16 as Add<i16>>
+              <&'a i32 as Add<i32>>
+              <&'a i64 as Add<i64>>
+              <&'a i8 as Add<i8>>
+              <&'a isize as Add<isize>>
+ 
21 error[E0308]: mismatched types
22   --> $DIR/ufcs-qpath-self-mismatch.rs:7:28
23    |
23    |

72              <&'a isize as Add<isize>>
74 
- error: aborting due to 4 previous errors
+ error: aborting due to 5 previous errors
76 
---
To only update this specific test, also pass `--test-args ufcs/ufcs-qpath-self-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/ufcs/ufcs-qpath-self-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-qpath-self-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-qpath-self-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: cannot add `u32` to `i32`
   |
   |
LL |     <i32 as Add<u32>>::add(1, 2);
   |     ----------------------    ^ no implementation for `i32 + u32`
   |     required by a bound introduced by this call
   |
   = help: the trait `Add<u32>` is not implemented for `i32`
   = help: the trait `Add<u32>` is not implemented for `i32`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a f32 as Add<f32>>
             <&'a f64 as Add<f64>>
             <&'a i128 as Add<i128>>
             <&'a i16 as Add<i16>>
             <&'a i32 as Add<i32>>
             <&'a i64 as Add<i64>>
             <&'a i8 as Add<i8>>
             <&'a isize as Add<isize>>


error[E0277]: cannot add `u32` to `i32`
   |
   |
LL |     <i32 as Add<u32>>::add(1, 2);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `i32 + u32`
   = help: the trait `Add<u32>` is not implemented for `i32`
   = help: the trait `Add<u32>` is not implemented for `i32`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a f32 as Add<f32>>
             <&'a f64 as Add<f64>>
             <&'a i128 as Add<i128>>
             <&'a i16 as Add<i16>>
             <&'a i32 as Add<i32>>
             <&'a i64 as Add<i64>>
             <&'a i8 as Add<i8>>
             <&'a isize as Add<isize>>

error[E0308]: mismatched types
  --> /checkout/src/test/ui/ufcs/ufcs-qpath-self-mismatch.rs:7:28
   |
   |
LL |     <i32 as Add<i32>>::add(1u32, 2);
   |     ---------------------- ^^^^ expected `i32`, found `u32`
   |     arguments to this function are incorrect
   |
note: associated function defined here
  --> /checkout/library/core/src/ops/arith.rs:91:8
  --> /checkout/library/core/src/ops/arith.rs:91:8
   |
LL |     fn add(self, rhs: Rhs) -> Self::Output;
   |        ^^^
help: change the type of the numeric literal from `u32` to `i32`
   |
LL |     <i32 as Add<i32>>::add(1i32, 2);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/ufcs/ufcs-qpath-self-mismatch.rs:9:31
   |
   |
LL |     <i32 as Add<i32>>::add(1, 2u32);
   |     ----------------------    ^^^^ expected `i32`, found `u32`
   |     arguments to this function are incorrect
   |
note: associated function defined here
  --> /checkout/library/core/src/ops/arith.rs:91:8
  --> /checkout/library/core/src/ops/arith.rs:91:8
   |
LL |     fn add(self, rhs: Rhs) -> Self::Output;
   |        ^^^
help: change the type of the numeric literal from `u32` to `i32`
   |
LL |     <i32 as Add<i32>>::add(1, 2i32);


error[E0277]: cannot add `u32` to `i32`
   |
   |
LL |     <i32 as Add<u32>>::add(1, 2);
   |     ^^^^^^^^^^^^^^^^^^^^^^ no implementation for `i32 + u32`
   = help: the trait `Add<u32>` is not implemented for `i32`
   = help: the trait `Add<u32>` is not implemented for `i32`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a f32 as Add<f32>>
             <&'a f64 as Add<f64>>
             <&'a i128 as Add<i128>>
             <&'a i16 as Add<i16>>
             <&'a i32 as Add<i32>>
             <&'a i64 as Add<i64>>
             <&'a i8 as Add<i8>>
             <&'a isize as Add<isize>>

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0308.
