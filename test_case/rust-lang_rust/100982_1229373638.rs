plain
..................................i..................................................... 2024/13441
.................F...................................................................... 2112/13441
........................................................................................ 2200/13441
........................................................................................ 2288/13441
........................F.............F................................................. 2376/13441
........................................................................................ 2464/13441
...............................................F.F...................................... 2552/13441
........................................................................................ 2640/13441
..F..............FF..................................................................... 2728/13441
.........................................................................F.............. 2816/13441
........................................................................................ 2904/13441
.......................F.......................................................F........ 2992/13441
........................................................................................ 3168/13441
........................................................................................ 3256/13441
....................................................F..iiiii............................ 3344/13441
........................................................................................ 3432/13441
---
........................................................................................ 10384/13441
........................................................................................ 10472/13441
........................................................................................ 10560/13441
........................................................................................ 10648/13441
................iiiii...i....i.i.......F...FFF....F.......FF.FFF..FF....F.....F.F..F...F 10736/13441
.FFFF..F....FF............................................................i............. 10824/13441
ii.i..iiiiii.i.......................................................................... 11000/13441
........................................................................................ 11088/13441
........................................................................................ 11176/13441
........................................................................................ 11264/13441
---
---- [ui] src/test/ui/const-generics/const_trait_fn-issue-88433.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_trait_fn-issue-88433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_trait_fn-issue-88433" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_trait_fn-issue-88433/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const Func<&usize> for Closure {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Func<T> {
   | ^^^^^^^^^^^^^

---
---- [ui] src/test/ui/const-generics/issues/issue-88119.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-88119.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-88119" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-88119/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const ConstName for u8 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait ConstName {
   | ^^^^^^^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | / impl<T: ?Sized + ConstName> const ConstName for &T
LL | | where
LL | |     [(); name_len::<T>()]:,
LL | | {
LL | |     const NAME_BYTES: &'static [u8] = b"&T";
LL | | }
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait ConstName {
   | ^^^^^^^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | / impl<T: ?Sized + ConstName> const ConstName for &mut T
LL | | where
LL | |     [(); name_len::<T>()]:,
LL | | {
LL | |     const NAME_BYTES: &'static [u8] = b"&mut T";
LL | | }
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait ConstName {
   | ^^^^^^^^^^^^^^^


error: aborting due to 3 previous errors
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-98629.rs stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+ LL | impl const Trait for i32 {}
+    | ^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+ LL | trait Trait {
+    | ^^^^^^^^^^^
+ 
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-98629.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-98629.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-98629" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-98629/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Trait for i32 {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Trait {
   | ^^^^^^^^^^^

---

---- [ui] src/test/ui/consts/const-eval/const-eval-overflow-3b.rs stdout ----
diff of stderr:

4 LL |     = [0; (i8::MAX + 1u8) as usize];
5    |                      ^^^ expected `i8`, found `u8`
6 
- error[E0277]: cannot add `u8` to `i8`
+ error[E0277]: cannot add `u8` to `i8` in const contexts
9    |
9    |
10 LL |     = [0; (i8::MAX + 1u8) as usize];

11    |                    ^ no implementation for `i8 + u8`
-    = help: the trait `Add<u8>` is not implemented for `i8`
-    = help: the trait `Add<u8>` is not implemented for `i8`
+    = help: the trait `~const Add<u8>` is not implemented for `i8`
14    = help: the following other types implement trait `Add<Rhs>`:
15              <&'a f32 as Add<f32>>
16              <&'a f64 as Add<f64>>

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3b/const-eval-overflow-3b.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-3b.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-3b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3b" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3b/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-3b.rs:16:22
   |
   |
LL |     = [0; (i8::MAX + 1u8) as usize];
   |                      ^^^ expected `i8`, found `u8`

error[E0277]: cannot add `u8` to `i8` in const contexts
   |
   |
LL |     = [0; (i8::MAX + 1u8) as usize];
   |                    ^ no implementation for `i8 + u8`
   |
   = help: the trait `~const Add<u8>` is not implemented for `i8`
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

Some errors have detailed explanations: E0277, E0308.
---

---- [ui] src/test/ui/consts/const-eval/const-eval-overflow-4b.rs stdout ----
diff of stderr:

4 LL |     : [u32; (i8::MAX as i8 + 1u8) as usize]
5    |                              ^^^ expected `i8`, found `u8`
6 
- error[E0277]: cannot add `u8` to `i8`
+ error[E0277]: cannot add `u8` to `i8` in const contexts
9    |
9    |
10 LL |     : [u32; (i8::MAX as i8 + 1u8) as usize]

11    |                            ^ no implementation for `i8 + u8`
-    = help: the trait `Add<u8>` is not implemented for `i8`
-    = help: the trait `Add<u8>` is not implemented for `i8`
+    = help: the trait `~const Add<u8>` is not implemented for `i8`
14    = help: the following other types implement trait `Add<Rhs>`:
15              <&'a f32 as Add<f32>>
16              <&'a f64 as Add<f64>>

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4b/const-eval-overflow-4b.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-4b.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-4b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4b" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4b/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-4b.rs:9:30
   |
   |
LL |     : [u32; (i8::MAX as i8 + 1u8) as usize]
   |                              ^^^ expected `i8`, found `u8`

error[E0277]: cannot add `u8` to `i8` in const contexts
   |
   |
LL |     : [u32; (i8::MAX as i8 + 1u8) as usize]
   |                            ^ no implementation for `i8 + u8`
   |
   = help: the trait `~const Add<u8>` is not implemented for `i8`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a f32 as Add<f32>>
             <&'a f64 as Add<f64>>
             <&'a i128 as Add<i128>>
             <&'a i16 as Add<i16>>
             <&'a i32 as Add<i32>>
             <&'a i64 as Add<i64>>
             <&'a i8 as Add<i8>>
             <&'a isize as Add<isize>>


error[E0604]: only `u8` can be cast as `char`, not `i8`
   |
   |
LL |     : [u32; 5i8 as char as usize]
   |             ^^^^^^^^^^^ invalid cast
help: try casting from `u8` instead
  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-4b.rs:22:13
   |
   |
LL |     : [u32; 5i8 as char as usize]

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0604.
---

-    |              ^^^^
+    |              ^^^^ `std::ops::Range<usize>` is not an iterator
21    |
- note: impl defined here, but it is not `const`
-   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
-    |
- LL | impl<I: ~const Iterator> const IntoIterator for I {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
- error[E0658]: mutable references are not allowed in constant functions
- error[E0658]: mutable references are not allowed in constant functions
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<usize>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<usize>`, but that implementation is not `const`
31    |
32 LL |     for i in 0..x {

33    |              ^^^^
33    |              ^^^^
+    = note: required for `std::ops::Range<usize>` to implement `~const IntoIterator`
+ help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
-    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
-    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
-    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
+ LL | const fn f(x: usize) -> usize where std::ops::Range<usize>: ~const Iterator {
37 
37 
- error[E0015]: cannot call non-const fn `<std::ops::Range<usize> as Iterator>::next` in constant functions
-   --> $DIR/const-fn-error.rs:5:14
- LL |     for i in 0..x {
-    |              ^^^^
-    |
-    |
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+ error: aborting due to 2 previous errors
- error: aborting due to 4 previous errors
- 
- Some errors have detailed explanations: E0015, E0658.
- For more information about an error, try `rustc --explain E0015`.
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
LL | |         //~^ ERROR mutable references
LL | |         //~| ERROR cannot convert
LL | |         //~| ERROR cannot call non-const fn
LL | |         //~| ERROR `for` is not allowed in a `const fn`
LL | |         sum += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0277]: the trait bound `std::ops::Range<usize>: Iterator` is not satisfied
  --> /checkout/src/test/ui/consts/const-fn-error.rs:5:14
   |
LL |     for i in 0..x {
   |              ^^^^ `std::ops::Range<usize>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<usize>`
note: the trait `Iterator` is implemented for `std::ops::Range<usize>`, but that implementation is not `const`
   |
LL |     for i in 0..x {
   |              ^^^^
   |              ^^^^
   = note: required for `std::ops::Range<usize>` to implement `~const IntoIterator`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | const fn f(x: usize) -> usize where std::ops::Range<usize>: ~const Iterator {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
---
7    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
8    = help: add `#![feature(const_for)]` to the crate attributes to enable
9 
- error: aborting due to previous error
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
+    |
+    |
+ LL |     for _ in 0..5 {}
+    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
+    |
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
+    |
+    |
+ LL |     for _ in 0..5 {}
+    |              ^^^^
+    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
- For more information about this error, try `rustc --explain E0658`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0277, E0658.
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
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable

error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
LL |     for _ in 0..5 {}
   |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
LL |     for _ in 0..5 {}
   |              ^^^^
   |              ^^^^
   = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/consts/const-for.rs stdout ----
diff of stderr:

- error[E0015]: cannot convert `std::ops::Range<i32>` into an iterator in constants
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
3    |
4 LL |     for _ in 0..5 {}

-    |              ^^^^
-    |              ^^^^
+    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
6    |
- note: impl defined here, but it is not `const`
-   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
-    |
- LL | impl<I: ~const Iterator> const IntoIterator for I {
-    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
- 
- 
- error[E0015]: cannot call non-const fn `<std::ops::Range<i32> as Iterator>::next` in constants
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
16    |
17 LL |     for _ in 0..5 {}

18    |              ^^^^
18    |              ^^^^
-    |
-    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
23 
- For more information about this error, try `rustc --explain E0015`.
---
To only update this specific test, also pass `--test-args consts/const-for.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-for.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
LL |     for _ in 0..5 {}
   |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
LL |     for _ in 0..5 {}
   |              ^^^^
   |              ^^^^
   = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
20    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
21    = help: add `#![feature(const_for)]` to the crate attributes to enable
22 
- error: aborting due to 2 previous errors
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
+    |
+ LL |     for i in 0..4 {
+    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
+    |
+    |
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
+    |
+ LL |     for i in 0..4 {
+    |              ^^^^
+    |              ^^^^
+    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
- For more information about this error, try `rustc --explain E0658`.
- For more information about this error, try `rustc --explain E0658`.
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
+    |
+ LL |     for i in 0..4 {
+    |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
+    |
+    |
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
+    |
+ LL |     for i in 0..4 {
+    |              ^^^^
+    |              ^^^^
+    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
+ error: aborting due to 4 previous errors
+ 
+ Some errors have detailed explanations: E0277, E0658.
+ For more information about an error, try `rustc --explain E0277`.
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
LL | |         x += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0658]: `for` is not allowed in a `const`
   |
   |
LL | /     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
LL | |         x += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
   |
LL |     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
   |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
   |
LL |     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
   |              ^^^^
   = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`

error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
   |
LL |     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
   |              ^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
   |
LL |     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
   |              ^^^^
   = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0658.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/consts/issue-94675.rs stdout ----
diff of stderr:

- error[E0015]: cannot call non-const fn `Vec::<u32>::len` in constant functions
-   --> $DIR/issue-94675.rs:9:27
+ error[E0277]: the trait bound `Vec<usize>: ~const Index<_>` is not satisfied
3    |
3    |
4 LL |         self.bar[0] = baz.len();
-    |                           ^^^^^
+    |         ^^^^^^^^^^^ vector indices are of type `usize` or ranges of `usize`
6    |
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    = help: the trait `~const Index<_>` is not implemented for `Vec<usize>`
+ note: the trait `Index<_>` is implemented for `Vec<usize>`, but that implementation is not `const`
+    |
+    |
+ LL |         self.bar[0] = baz.len();
8 
8 
9 error[E0277]: the trait bound `Vec<usize>: ~const IndexMut<usize>` is not satisfied

18    |
18    |
19 LL |         self.bar[0] = baz.len();
- 
- error[E0015]: cannot call non-const operator in constant functions
-   --> $DIR/issue-94675.rs:9:9
-   --> $DIR/issue-94675.rs:9:9
+ help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
24    |
- LL |         self.bar[0] = baz.len();
-    |
-    |
- note: impl defined here, but it is not `const`
-   --> $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-    |
- LL | impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A> {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+ LL | impl<'a> Foo<'a> where Vec<usize>: ~const IndexMut<usize> {
34 
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
36 
---
To only update this specific test, also pass `--test-args consts/issue-94675.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-94675.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-94675" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-94675/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Vec<usize>: ~const Index<_>` is not satisfied
   |
   |
LL |         self.bar[0] = baz.len();
   |         ^^^^^^^^^^^ vector indices are of type `usize` or ranges of `usize`
   |
   = help: the trait `~const Index<_>` is not implemented for `Vec<usize>`
note: the trait `Index<_>` is implemented for `Vec<usize>`, but that implementation is not `const`
   |
   |
LL |         self.bar[0] = baz.len();


error[E0277]: the trait bound `Vec<usize>: ~const IndexMut<usize>` is not satisfied
   |
   |
LL |         self.bar[0] = baz.len();
   |         ^^^^^^^^^^^ vector indices are of type `usize` or ranges of `usize`
   |
   = help: the trait `~const IndexMut<usize>` is not implemented for `Vec<usize>`
note: the trait `IndexMut<usize>` is implemented for `Vec<usize>`, but that implementation is not `const`
   |
   |
LL |         self.bar[0] = baz.len();
   |         ^^^^^^^^^^^
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl<'a> Foo<'a> where Vec<usize>: ~const IndexMut<usize> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/consts/promoted-const-drop.rs stdout ----
diff of stderr:

- error[E0716]: temporary value dropped while borrowed
-   --> $DIR/promoted-const-drop.rs:13:26
+ error: const `impl`s must be for traits marked with `#[const_trait]`
3    |
3    |
- LL |     let _: &'static A = &A();
-    |            ----------    ^^^ creates a temporary which is freed while still in use
-    |            |
-    |            type annotation requires that borrow lasts for `'static`
- LL |     let _: &'static [A] = &[C];
- LL | }
-    | - temporary value is freed at the end of this statement
- error[E0716]: temporary value dropped while borrowed
-   --> $DIR/promoted-const-drop.rs:14:28
-   --> $DIR/promoted-const-drop.rs:14:28
+ LL | impl const Drop for A {
14    |
14    |
- LL |     let _: &'static [A] = &[C];
-    |            ------------    ^^^ creates a temporary which is freed while still in use
-    |            |
-    |            type annotation requires that borrow lasts for `'static`
- LL | }
-    | - temporary value is freed at the end of this statement
+ note: this trait must be annotated with `#[const_trait]`
+   --> $SRC_DIR/core/src/ops/drop.rs:LL:COL
+    |
+ LL | pub trait Drop {
21 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
23 
---
To only update this specific test, also pass `--test-args consts/promoted-const-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promoted-const-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Drop for A {
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs stdout ----
diff of stderr:

15 LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³⋅kg⁻¹⋅s⁻²
17 
17 
- error[E0277]: cannot subtract `{integer}` from `{float}`
+ error[E0277]: the trait bound `{float}: Sub<{integer}>` is not satisfied
20    |
20    |
21 LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²

22    |                                                     ^ no implementation for `{float} - {integer}`
23    |
-    = help: the trait `Sub<{integer}>` is not implemented for `{float}`
+    = help: the trait `~const Sub<{integer}>` is not implemented for `{float}`
25    = help: the following other types implement trait `Sub<Rhs>`:
26              <&'a f32 as Sub<f32>>
27              <&'a f64 as Sub<f64>>

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/issue-49746-unicode-confusable-in-float-literal-expt.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected at least one digit in exponent
   |
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²

error: unknown start of token: \u{2212}
  --> /checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:53
   |
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
   |
   |
help: Unicode character '−' (Minus Sign) looks like '-' (Minus/Hyphen), but it is not
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³⋅kg⁻¹⋅s⁻²


error[E0277]: the trait bound `{float}: Sub<{integer}>` is not satisfied
   |
   |
LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
   |                                                     ^ no implementation for `{float} - {integer}`
   |
   = help: the trait `~const Sub<{integer}>` is not implemented for `{float}`
   = help: the following other types implement trait `Sub<Rhs>`:
             <&'a f32 as Sub<f32>>
             <&'a f64 as Sub<f64>>
             <&'a i128 as Sub<i128>>
             <&'a i16 as Sub<i16>>
             <&'a i32 as Sub<i32>>
             <&'a i64 as Sub<i64>>
             <&'a i8 as Sub<i8>>
             <&'a isize as Sub<isize>>

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

2   --> $DIR/issue-20605.rs:2:17
3    |
4 LL |     for item in *things { *item = 0 }
-    |                 ^^^^^^^ the trait `IntoIterator` is not implemented for `dyn Iterator<Item = &'a mut u8>`
+    |                 ^^^^^^^ the trait `~const IntoIterator` is not implemented for `dyn Iterator<Item = &'a mut u8>`
6    |
7    = note: the trait bound `dyn Iterator<Item = &'a mut u8>: IntoIterator` is not satisfied
8    = note: required for `dyn Iterator<Item = &'a mut u8>` to implement `IntoIterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605/issue-20605.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-20605.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20605.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `dyn Iterator<Item = &'a mut u8>` cannot be known at compilation time
   |
   |
LL |     for item in *things { *item = 0 }
   |                 ^^^^^^^ the trait `~const IntoIterator` is not implemented for `dyn Iterator<Item = &'a mut u8>`
   |
   = note: the trait bound `dyn Iterator<Item = &'a mut u8>: IntoIterator` is not satisfied
   = note: required for `dyn Iterator<Item = &'a mut u8>` to implement `IntoIterator`
help: consider mutably borrowing here
   |
LL |     for item in &mut *things { *item = 0 }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-25901.rs stdout ----
diff of stderr:

- error[E0015]: cannot perform deref coercion on `A` in statics
+ error[E0277]: the trait bound `A: Deref` is not satisfied
3    |
3    |
4 LL | static S: &'static B = &A;
-    |                        ^^
-    |                        ^^
+    |                        ^^ the trait `~const Deref` is not implemented for `A`
-    = note: attempting to deref into `B`
- note: deref defined here
-   --> $DIR/issue-25901.rs:10:5
-   --> $DIR/issue-25901.rs:10:5
+ note: the trait `Deref` is implemented for `A`, but that implementation is not `const`
10    |
- LL |     type Target = B;
-    |     ^^^^^^^^^^^
-    |     ^^^^^^^^^^^
- note: impl defined here, but it is not `const`
-   --> $DIR/issue-25901.rs:9:1
-    |
- LL | impl Deref for A {
-    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
-    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
-    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
+ LL | static S: &'static B = &A;
20 
21 error: aborting due to previous error
22 

---
To only update this specific test, also pass `--test-args issues/issue-25901.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25901.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25901" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25901/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `A: Deref` is not satisfied
   |
   |
LL | static S: &'static B = &A;
   |                        ^^ the trait `~const Deref` is not implemented for `A`
   |
note: the trait `Deref` is implemented for `A`, but that implementation is not `const`
   |
   |
LL | static S: &'static B = &A;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-32709.rs stdout ----
diff of stderr:

7    |           ^ the trait `From<{integer}>` is not implemented for `()`
8    |
9    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = help: the following other types implement trait `FromResidual<R>`:
-              <Result<T, F> as FromResidual<Result<Infallible, E>>>
-              <Result<T, F> as FromResidual<Yeet<E>>>
+    = help: the trait `FromResidual<Result<Infallible, E>>` is implemented for `Result<T, F>`
13    = note: required for `Result<i32, ()>` to implement `FromResidual<Result<Infallible, {integer}>>`
15 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/issue-32709.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-32709.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `?` couldn't convert the error to `()`
   |
   |
LL | fn a() -> Result<i32, ()> {
   |           --------------- expected `()` because of this
LL |     Err(5)?; //~ ERROR
   |           ^ the trait `From<{integer}>` is not implemented for `()`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the trait `FromResidual<Result<Infallible, E>>` is implemented for `Result<T, F>`
   = note: required for `Result<i32, ()>` to implement `FromResidual<Result<Infallible, {integer}>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---

7    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
8    = help: add `#![feature(const_for)]` to the crate attributes to enable
9 
- error[E0277]: cannot add `()` to `{integer}`
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
+    |
+    |
+ LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
+    |                             ^^^^ `std::ops::Range<{integer}>` is not an iterator
+    |
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
+    |
+    |
+ LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
+    |                             ^^^^
+    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
+ 
+ error[E0277]: cannot add `()` to `{integer}` in const contexts
12    |
12    |
13 LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();

14    |                  ^ no implementation for `{integer} + ()`
15    |
-    = help: the trait `Add<()>` is not implemented for `{integer}`
+    = help: the trait `~const Add<()>` is not implemented for `{integer}`
17    = help: the following other types implement trait `Add<Rhs>`:
18              <&'a f32 as Add<f32>>
19              <&'a f64 as Add<f64>>

25              <&'a isize as Add<isize>>
27 
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
29 
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


error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
   |
LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
   |                             ^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
   |
LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
   |                             ^^^^
   = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`

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

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0658.
---

7    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
8    = help: add `#![feature(const_for)]` to the crate attributes to enable
9 
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
+    |
+    |
+ LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
+    |                           ^^^^ `std::ops::Range<{integer}>` is not an iterator
+    |
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
+    |
+    |
+ LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
+    |                           ^^^^
+    = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
10 error[E0308]: mismatched types
11   --> $DIR/issue-50585.rs:2:18
12    |


13 LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
15 
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
17 
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


error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
   |                           ^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
   |                           ^^^^
   = note: required for `std::ops::Range<{integer}>` to implement `~const IntoIterator`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-50585.rs:2:18
   |
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0658.
---
---- [ui] src/test/ui/never_type/issue-52443.rs stdout ----
diff of stderr:

38    |                 expected `usize`, found `()`
39    |                 help: give it a value of the expected type: `break 42`
40 
- error[E0015]: cannot convert `RangeFrom<usize>` into an iterator in constants
+ error[E0277]: the trait bound `RangeFrom<usize>: Iterator` is not satisfied
43    |
43    |
44 LL |     [(); { for _ in 0usize.. {}; 0}];
-    |                     ^^^^^^^^
+    |                     ^^^^^^^^ `RangeFrom<usize>` is not an iterator
46    |
46    |
- note: impl defined here, but it is not `const`
-   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
-    |
- LL | impl<I: ~const Iterator> const IntoIterator for I {
-    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
- 
- error[E0658]: mutable references are not allowed in constants
- error[E0658]: mutable references are not allowed in constants
+    = help: the trait `~const Iterator` is not implemented for `RangeFrom<usize>`
+ note: the trait `Iterator` is implemented for `RangeFrom<usize>`, but that implementation is not `const`
56    |
56    |
57 LL |     [(); { for _ in 0usize.. {}; 0}];
58    |                     ^^^^^^^^
58    |                     ^^^^^^^^
+    = note: required for `RangeFrom<usize>` to implement `~const IntoIterator`
+ help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
-    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
-    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
-    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
+ LL | fn main() where RangeFrom<usize>: ~const Iterator {
62 
62 
- error[E0015]: cannot call non-const fn `<RangeFrom<usize> as Iterator>::next` in constants
-   --> $DIR/issue-52443.rs:9:21
-    |
- LL |     [(); { for _ in 0usize.. {}; 0}];
-    |
-    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
+ error: aborting due to 4 previous errors; 1 warning emitted
70 
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

error[E0277]: the trait bound `RangeFrom<usize>: Iterator` is not satisfied
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |                     ^^^^^^^^ `RangeFrom<usize>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `RangeFrom<usize>`
note: the trait `Iterator` is implemented for `RangeFrom<usize>`, but that implementation is not `const`
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |                     ^^^^^^^^
   = note: required for `RangeFrom<usize>` to implement `~const IntoIterator`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn main() where RangeFrom<usize>: ~const Iterator {

error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0277, E0308, E0658.
Some errors have detailed explanations: E0277, E0308, E0658.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `u32: ~const Plus` is not satisfied
-   --> $DIR/call-const-trait-method-fail.rs:24:7
+ error: const `impl`s must be for traits marked with `#[const_trait]`
3    |
3    |
- LL |     a.plus(b)
-    |       ^^^^^^^ the trait `~const Plus` is not implemented for `u32`
+ LL | impl const Plus for i32 {
6    |
6    |
- note: the trait `Plus` is implemented for `u32`, but that implementation is not `const`
-   --> $DIR/call-const-trait-method-fail.rs:24:7
+ note: this trait must be annotated with `#[const_trait]`
9    |
9    |
- LL |     a.plus(b)
-    |       ^^^^^^^
+ LL | pub trait Plus {
12 
12 
- error[E0015]: cannot call non-const fn `<u32 as Plus>::plus` in constant functions
-   --> $DIR/call-const-trait-method-fail.rs:24:7
-    |
- LL |     a.plus(b)
-    |
-    |
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0015, E0277.
- For more information about an error, try `rustc --explain E0015`.
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/call-const-trait-method-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Plus for i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | pub trait Plus {
   | ^^^^^^^^^^^^^^

---
---- [ui] src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-pass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-pass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-pass/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Plus for i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | pub trait Plus {
   | ^^^^^^^^^^^^^^

---
---- [ui] src/test/ui/rfc-2632-const-trait-impl/call-generic-in-impl.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-generic-in-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-generic-in-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-generic-in-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<T: ~const PartialEq> const MyPartialEq for T {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait MyPartialEq {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const Foo for NonConstAdd {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+ LL | trait Foo {
+    | ^^^^^^^^^
+ 
+ 
+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const Baz for NonConstAdd {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+ LL | trait Baz {
+    | ^^^^^^^^^
+ 
+ 
1 error[E0277]: cannot add `NonConstAdd` to `NonConstAdd` in const contexts
3    |


20 LL | impl const Foo for NonConstAdd where NonConstAdd: ~const Add {
22 
- error: aborting due to previous error
+ error: aborting due to 3 previous errors
24 
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/assoc-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Foo for NonConstAdd {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Foo {
   | ^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const Baz for NonConstAdd {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Baz {
   | ^^^^^^^^^


error[E0277]: cannot add `NonConstAdd` to `NonConstAdd` in const contexts
   |
LL |     type Bar = NonConstAdd;
LL |     type Bar = NonConstAdd;
   |                ^^^^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
   |
   = help: the trait `~const Add` is not implemented for `NonConstAdd`
note: the trait `Add` is implemented for `NonConstAdd`, but that implementation is not `const`
   |
LL |     type Bar = NonConstAdd;
   |                ^^^^^^^^^^^
note: required by a bound in `Foo::Bar`
note: required by a bound in `Foo::Bar`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs:14:15
   |
LL |     type Bar: ~const std::ops::Add;
   |               ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Foo::Bar`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl const Foo for NonConstAdd where NonConstAdd: ~const Add {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl.rs stdout ----
diff of stderr:

- error[E0015]: cannot call non-const fn `non_const` in constant functions
-   --> $DIR/const-check-fns-in-const-impl.rs:11:16
+ error: const `impl`s must be for traits marked with `#[const_trait]`
3    |
3    |
- LL |     fn foo() { non_const() }
-    |                ^^^^^^^^^^^
+ LL | impl const T for S {
6    |
6    |
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+ note: this trait must be annotated with `#[const_trait]`
+    |
+    |
+ LL | trait T {
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-check-fns-in-const-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const T for S {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait T {
   | ^^^^^^^


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-drop.rs#precise stdout ----

error in revision `precise`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "precise" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop.precise/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop.precise/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<'a> const Drop for S<'a> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL |     impl const Drop for ConstDrop {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL |     impl const SomeTrait for () {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL |     pub trait SomeTrait {
   |     ^^^^^^^^^^^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL |     impl<T: ~const SomeTrait> const Drop for ConstDropWithBound<T> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL |     impl<T: SomeTrait> const Drop for ConstDropWithNonconstBound<T> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {

error: aborting due to 5 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs#precise stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const Drop for ConstImplWithDropGlue {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+   --> $SRC_DIR/core/src/ops/drop.rs:LL:COL
+    |
+ LL | pub trait Drop {
+ 
+ 
+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl<T: ~const A> const Drop for ConstDropImplWithBounds<T> {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+   --> $SRC_DIR/core/src/ops/drop.rs:LL:COL
+    |
+ LL | pub trait Drop {
+ 
+ 
1 error[E0277]: can't drop `NonTrivialDrop` in const contexts
3    |


73 LL |     &mut ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
75 
- error: aborting due to 3 previous errors
+ error: aborting due to 5 previous errors
77 
77 
78 For more information about this error, try `rustc --explain E0277`.
79 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise/const-drop-fail.precise.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-drop-fail.rs`


error in revision `precise`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "precise" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const Drop for ConstImplWithDropGlue {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<T: ~const A> const Drop for ConstDropImplWithBounds<T> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error[E0277]: can't drop `NonTrivialDrop` in const contexts
   |
   |
LL |         const _: () = check($exp);
...
...
LL |     NonTrivialDrop,
   |     ^^^^^^^^^^^^^^ the trait `~const Destruct` is not implemented for `NonTrivialDrop`
   |
   = note: the trait bound `NonTrivialDrop: ~const Destruct` is not satisfied
note: required by a bound in `check`
   |
   |
LL | const fn check<T: ~const Destruct>(_: T) {}
   |                   ^^^^^^^^^^^^^^^ required by this bound in `check`
   |
   |
LL |     &NonTrivialDrop,
   |     +
LL |     &mut NonTrivialDrop,


error[E0277]: can't drop `NonTrivialDrop` in const contexts
   |
   |
LL |         const _: () = check($exp);
...
...
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ within `ConstImplWithDropGlue`, the trait `~const Destruct` is not implemented for `NonTrivialDrop`
   |
note: the trait `Destruct` is implemented for `NonTrivialDrop`, but that implementation is not `const`
   |
   |
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `ConstImplWithDropGlue`
   |
   |
LL | struct ConstImplWithDropGlue(NonTrivialDrop);
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:34:19
   |
   |
LL | const fn check<T: ~const Destruct>(_: T) {}
   |                   ^^^^^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `ConstDropImplWithBounds<NonTrivialDrop>: ~const Destruct` is not satisfied
   |
   |
LL |         const _: () = check($exp);
...
...
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `~const Destruct` is not implemented for `ConstDropImplWithBounds<NonTrivialDrop>`
   |
note: required for `ConstDropImplWithBounds<NonTrivialDrop>` to implement `~const Destruct`
   |
   |
LL | impl<T: ~const A> const Drop for ConstDropImplWithBounds<T> {
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required for `ConstDropImplWithBounds<NonTrivialDrop>` to implement `~const Destruct`
note: required by a bound in `check`
   |
   |
LL | const fn check<T: ~const Destruct>(_: T) {}
   |                   ^^^^^^^^^^^^^^^ required by this bound in `check`
   |
   |
LL |     &ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     +
LL |     &mut ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-drop.rs#stock stdout ----

error in revision `stock`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop.stock/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop.stock/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<'a> const Drop for S<'a> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL |     impl const Drop for ConstDrop {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL |     impl const SomeTrait for () {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL |     pub trait SomeTrait {
   |     ^^^^^^^^^^^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL |     impl<T: ~const SomeTrait> const Drop for ConstDropWithBound<T> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL |     impl<T: SomeTrait> const Drop for ConstDropWithNonconstBound<T> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {

error: aborting due to 5 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-impl-recovery.rs stdout ----
diff of stderr:

22 LL + impl<T: Foo> const Bar for T {}
24 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+ LL | const impl Foo for i32 {}
+    | ^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+ LL | trait Foo {}
+    | ^^^^^^^^^
+ 
+ 
+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | const impl<T: Foo> Bar for T {}
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+    |
+ LL | trait Bar {}
+ 
+ error: aborting due to 4 previous errors
26 
27 
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-impl-recovery.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-impl-recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-impl-recovery" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-impl-recovery/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `impl`
   |
   |
LL | const impl Foo for i32 {} //~ ERROR: expected identifier, found keyword
   |
help: you might have meant to write a const trait impl
   |
   |
LL - const impl Foo for i32 {} //~ ERROR: expected identifier, found keyword
LL + impl const Foo for i32 {} //~ ERROR: expected identifier, found keyword

error: expected identifier, found keyword `impl`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-impl-recovery.rs:9:7
   |
   |
LL | const impl<T: Foo> Bar for T {} //~ ERROR: expected identifier, found keyword
   |
help: you might have meant to write a const trait impl
   |
   |
LL - const impl<T: Foo> Bar for T {} //~ ERROR: expected identifier, found keyword
LL + impl<T: Foo> const Bar for T {} //~ ERROR: expected identifier, found keyword


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | const impl Foo for i32 {} //~ ERROR: expected identifier, found keyword
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Foo {}
   | ^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | const impl<T: Foo> Bar for T {} //~ ERROR: expected identifier, found keyword
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Bar {}
   | ^^^^^^^^^


error: aborting due to 4 previous errors
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs#stock stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const Drop for ConstImplWithDropGlue {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+   --> $SRC_DIR/core/src/ops/drop.rs:LL:COL
+    |
+ LL | pub trait Drop {
+ 
+ 
+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl<T: ~const A> const Drop for ConstDropImplWithBounds<T> {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+   --> $SRC_DIR/core/src/ops/drop.rs:LL:COL
+    |
+ LL | pub trait Drop {
+ 
+ 
1 error[E0277]: can't drop `NonTrivialDrop` in const contexts
3    |


73 LL |     &mut ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
75 
- error: aborting due to 3 previous errors
+ error: aborting due to 5 previous errors
77 
77 
78 For more information about this error, try `rustc --explain E0277`.
79 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock/const-drop-fail.stock.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-drop-fail.rs`


error in revision `stock`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const Drop for ConstImplWithDropGlue {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<T: ~const A> const Drop for ConstDropImplWithBounds<T> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Drop {


error[E0277]: can't drop `NonTrivialDrop` in const contexts
   |
   |
LL |         const _: () = check($exp);
...
...
LL |     NonTrivialDrop,
   |     ^^^^^^^^^^^^^^ the trait `~const Destruct` is not implemented for `NonTrivialDrop`
   |
   = note: the trait bound `NonTrivialDrop: ~const Destruct` is not satisfied
note: required by a bound in `check`
   |
   |
LL | const fn check<T: ~const Destruct>(_: T) {}
   |                   ^^^^^^^^^^^^^^^ required by this bound in `check`
   |
   |
LL |     &NonTrivialDrop,
   |     +
LL |     &mut NonTrivialDrop,


error[E0277]: can't drop `NonTrivialDrop` in const contexts
   |
   |
LL |         const _: () = check($exp);
...
...
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ within `ConstImplWithDropGlue`, the trait `~const Destruct` is not implemented for `NonTrivialDrop`
   |
note: the trait `Destruct` is implemented for `NonTrivialDrop`, but that implementation is not `const`
   |
   |
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `ConstImplWithDropGlue`
   |
   |
LL | struct ConstImplWithDropGlue(NonTrivialDrop);
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:34:19
   |
   |
LL | const fn check<T: ~const Destruct>(_: T) {}
   |                   ^^^^^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `ConstDropImplWithBounds<NonTrivialDrop>: ~const Destruct` is not satisfied
   |
   |
LL |         const _: () = check($exp);
...
...
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `~const Destruct` is not implemented for `ConstDropImplWithBounds<NonTrivialDrop>`
   |
note: required for `ConstDropImplWithBounds<NonTrivialDrop>` to implement `~const Destruct`
   |
   |
LL | impl<T: ~const A> const Drop for ConstDropImplWithBounds<T> {
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required for `ConstDropImplWithBounds<NonTrivialDrop>` to implement `~const Destruct`
note: required by a bound in `check`
   |
   |
LL | const fn check<T: ~const Destruct>(_: T) {}
   |                   ^^^^^^^^^^^^^^^ required by this bound in `check`
   |
   |
LL |     &ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     +
LL |     &mut ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs#gated stdout ----
diff of stderr:

- error: fatal error triggered by #[rustc_error]
-   --> $DIR/feature-gate.rs:13:1
+ error: const `impl`s must be for traits marked with `#[const_trait]`
3    |
- LL | fn main() {}
-    | ^^^^^^^^^
-    | ^^^^^^^^^
+ LL | impl const T for S {}
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+    |
+ LL | trait T {}
6 
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated/feature-gate.gated.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/feature-gate.rs`

error in revision `gated`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gated" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.gated/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const T for S {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait T {}

error: aborting due to previous error
------------------------------------------

---
7    = note: see issue #67792 <https://github.com/rust-lang/rust/issues/67792> for more information
8    = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable
9 
- error: aborting due to previous error
+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const T for S {}
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+    |
+ LL | trait T {}
+ 
+ error: aborting due to 2 previous errors
11 
12 For more information about this error, try `rustc --explain E0658`.
12 For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock/feature-gate.stock.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/feature-gate.rs`


error in revision `stock`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/feature-gate.stock/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: const trait impls are experimental
   |
   |
LL | impl const T for S {}
   |
   = note: see issue #67792 <https://github.com/rust-lang/rust/issues/67792> for more information
   = help: add `#![feature(const_trait_impl)]` to the crate attributes to enable


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const T for S {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait T {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/hir-const-check.rs stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+   --> $DIR/hir-const-check.rs:9:1
+    |
+ LL | impl const MyTrait for () {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+   --> $DIR/hir-const-check.rs:5:1
+    |
+ LL | pub trait MyTrait {
+ 
+ 
1 error[E0658]: `?` is not allowed in a `const fn`
2   --> $DIR/hir-const-check.rs:11:9

7    = note: see issue #74935 <https://github.com/rust-lang/rust/issues/74935> for more information
8    = help: add `#![feature(const_try)]` to the crate attributes to enable
9 
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/hir-const-check.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/hir-const-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/hir-const-check" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/hir-const-check/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const MyTrait for () {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait MyTrait {


error[E0658]: `?` is not allowed in a `const fn`
   |
   |
LL |         Some(())?; //~ ERROR `?` is not allowed in a `const fn`
   |
   = note: see issue #74935 <https://github.com/rust-lang/rust/issues/74935> for more information
   = help: add `#![feature(const_try)]` to the crate attributes to enable

---
---- [ui] src/test/ui/rfc-2632-const-trait-impl/inherent-impl-const-bounds.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/inherent-impl-const-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/inherent-impl-const-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/inherent-impl-const-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const A for S {}
   | ^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait A {}


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const B for S {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait B {}

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/issue-100222.rs#yn stdout ----

error in revision `yn`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/issue-100222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "yn" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-100222.yn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-100222.yn/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const IndexMut for <() as Index>::Output {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait IndexMut where Self: Index {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/issue-100222.rs#nn stdout ----

error in revision `nn`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/issue-100222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nn" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-100222.nn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-100222.nn/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const IndexMut for <() as Index>::Output {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait IndexMut where Self: Index {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/issue-92230-wf-super-trait-env.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/issue-92230-wf-super-trait-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-92230-wf-super-trait-env" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-92230-wf-super-trait-env/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<A> const Super for &A where A: ~const Super {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | pub trait Super {}
   | ^^^^^^^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<A> const Sub for &A where A: ~const Sub {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | pub trait Sub: Super {}

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/non-const-op-in-closure-in-const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/non-const-op-in-closure-in-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/non-const-op-in-closure-in-const" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/non-const-op-in-closure-in-const/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl<A, B> const Convert<B> for A where B: ~const From<A> {
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait Convert<T> {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/staged-api-user-crate.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api-user-crate/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api-user-crate/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs:16:1
   |
LL | impl const MyTrait for Unstable {
   |
   |
note: this trait must be annotated with `#[const_trait]`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs:6:1
   |
LL | pub trait MyTrait {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/staged-api.rs#stable stdout ----

error in revision `stable`: auxiliary build of "/checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stable" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.stable/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.stable/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs:16:1
   |
LL | impl const MyTrait for Unstable {
   |
   |
note: this trait must be annotated with `#[const_trait]`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs:6:1
   |
LL | pub trait MyTrait {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/staged-api.rs#unstable stdout ----

error in revision `unstable`: auxiliary build of "/checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unstable" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs:16:1
   |
LL | impl const MyTrait for Unstable {
   |
   |
note: this trait must be annotated with `#[const_trait]`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/auxiliary/staged-api.rs:6:1
   |
LL | pub trait MyTrait {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/super-traits.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/super-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/super-traits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/super-traits/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Foo for S {
   | ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Foo {
   | ^^^^^^^^^


error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const Bar for S {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait Bar: ~const Foo {}

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/super-traits-fail.rs stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const Bar for S {}
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+    |
+ LL | trait Bar: ~const Foo {}
+ 
+ 
1 error[E0277]: the trait bound `S: ~const Foo` is not satisfied
3    |


19 LL | impl const Bar for S where S: ~const Foo {}
21 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
23 
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/super-traits-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/super-traits-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/super-traits-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/super-traits-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
   |
LL | impl const Bar for S {}
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
   |
LL | trait Bar: ~const Foo {}


error[E0277]: the trait bound `S: ~const Foo` is not satisfied
   |
   |
LL | impl const Bar for S {}
   |            ^^^ the trait `~const Foo` is not implemented for `S`
   |
note: the trait `Foo` is implemented for `S`, but that implementation is not `const`
   |
   |
LL | impl const Bar for S {}
note: required by a bound in `Bar`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/super-traits-fail.rs:6:12
   |
   |
LL | trait Bar: ~const Foo {}
   |            ^^^^^^^^^^ required by this bound in `Bar`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl const Bar for S where S: ~const Foo {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/rfc-2632-const-trait-impl/trait-where-clause-run.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/trait-where-clause-run.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/trait-where-clause-run/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/trait-where-clause-run/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Bar for Const {
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | trait Bar {
   | ^^^^^^^^^


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/stability-attribute/missing-const-stability.rs stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+    |
+ LL | impl const Bar for Foo {
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+ LL | pub trait Bar {
+    | ^^^^^^^^^^^^^
+ 
---
To only update this specific test, also pass `--test-args stability-attribute/missing-const-stability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/missing-const-stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/missing-const-stability" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/missing-const-stability/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const Bar for Foo {
   | ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | pub trait Bar {
   | ^^^^^^^^^^^^^


error: function has missing const stability attribute
  --> /checkout/src/test/ui/stability-attribute/missing-const-stability.rs:6:1
   |
LL | pub const fn foo() {} //~ ERROR function has missing const stability attribute

error: implementation has missing const stability attribute
  --> /checkout/src/test/ui/stability-attribute/missing-const-stability.rs:27:1
   |
   |
LL | / impl const Bar for Foo {
LL | |     //~^ ERROR implementation has missing const stability attribute
LL | |     fn fun() {}
LL | | }

error: associated function has missing const stability attribute
  --> /checkout/src/test/ui/stability-attribute/missing-const-stability.rs:15:5
   |
   |
LL |     pub const fn foo() {} //~ ERROR associated function has missing const stability attribute

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/suggestions/slice-issue-87994.rs stdout ----
diff of stderr:

2   --> $DIR/slice-issue-87994.rs:3:12
3    |
4 LL |   for _ in v[1..] {
-    |            ^^^^^^ the trait `IntoIterator` is not implemented for `[i32]`
+    |            ^^^^^^ the trait `~const IntoIterator` is not implemented for `[i32]`
6    |
7    = note: the trait bound `[i32]: IntoIterator` is not satisfied
8    = note: required for `[i32]` to implement `IntoIterator`
17   --> $DIR/slice-issue-87994.rs:3:12
18    |
18    |
19 LL |   for _ in v[1..] {
-    |            ^^^^^^ the trait `IntoIterator` is not implemented for `[i32]`
+    |            ^^^^^^ the trait `~const IntoIterator` is not implemented for `[i32]`
21    |
22    = note: the trait bound `[i32]: IntoIterator` is not satisfied
23    = note: required for `[i32]` to implement `IntoIterator`
32   --> $DIR/slice-issue-87994.rs:11:13
33    |
33    |
34 LL |   for i2 in v2[1..] {
-    |             ^^^^^^^ the trait `IntoIterator` is not implemented for `[K]`
+    |             ^^^^^^^ the trait `~const IntoIterator` is not implemented for `[K]`
36    |
37    = note: the trait bound `[K]: IntoIterator` is not satisfied
38    = note: required for `[K]` to implement `IntoIterator`
47   --> $DIR/slice-issue-87994.rs:11:13
48    |
48    |
49 LL |   for i2 in v2[1..] {
-    |             ^^^^^^^ the trait `IntoIterator` is not implemented for `[K]`
+    |             ^^^^^^^ the trait `~const IntoIterator` is not implemented for `[K]`
51    |
52    = note: the trait bound `[K]: IntoIterator` is not satisfied
53    = note: required for `[K]` to implement `IntoIterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/slice-issue-87994/slice-issue-87994.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/slice-issue-87994.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/slice-issue-87994.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/slice-issue-87994" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/slice-issue-87994/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
   |
   |
LL |   for _ in v[1..] {
   |            ^^^^^^ the trait `~const IntoIterator` is not implemented for `[i32]`
   |
   = note: the trait bound `[i32]: IntoIterator` is not satisfied
   = note: required for `[i32]` to implement `IntoIterator`
   |
   |
LL |   for _ in &v[1..] {
   |            +
LL |   for _ in &mut v[1..] {


error[E0277]: `[i32]` is not an iterator
   |
   |
LL |   for _ in v[1..] {
   |            ^^^^^^ the trait `~const IntoIterator` is not implemented for `[i32]`
   |
   = note: the trait bound `[i32]: IntoIterator` is not satisfied
   = note: required for `[i32]` to implement `IntoIterator`
   |
   |
LL |   for _ in &v[1..] {
   |            +
LL |   for _ in &mut v[1..] {


error[E0277]: the size for values of type `[K]` cannot be known at compilation time
   |
   |
LL |   for i2 in v2[1..] {
   |             ^^^^^^^ the trait `~const IntoIterator` is not implemented for `[K]`
   |
   = note: the trait bound `[K]: IntoIterator` is not satisfied
   = note: required for `[K]` to implement `IntoIterator`
   |
   |
LL |   for i2 in &v2[1..] {
   |             +
LL |   for i2 in &mut v2[1..] {


error[E0277]: `[K]` is not an iterator
   |
   |
LL |   for i2 in v2[1..] {
   |             ^^^^^^^ the trait `~const IntoIterator` is not implemented for `[K]`
   |
   = note: the trait bound `[K]: IntoIterator` is not satisfied
   = note: required for `[K]` to implement `IntoIterator`
   |
   |
LL |   for i2 in &v2[1..] {
   |             +
LL |   for i2 in &mut v2[1..] {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
