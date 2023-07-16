plain
..........................................................................i............. 1848/14042
.....i...........ii..................................................................... 1936/14042
........................................................................................ 2024/14042
..............................................................................i......... 2112/14042
..........................F.F........................................................... 2200/14042
........................................................................................ 2376/14042
........................................................................................ 2464/14042
........................................................................................ 2552/14042
............F........................................................................... 2640/14042
---
failures:

---- [ui] src/test/ui/const-generics/bool_cond.rs#unified stdout ----

error in revision `unified`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/bool_cond.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unified" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/bool_cond.unified/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/bool_cond.unified/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | pub struct ConstOption<T, const S: bool> where [T; bool_to_usize(S)]:, {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:13:13
   |
   |
LL |     _v: [T; bool_to_usize(S)]
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:47:58
   |
   |
LL | fn _test_func<const N: usize>() where ConstOption<usize, {N >= 5}>: Default,
   |                                                          ^------^
   |                                                           pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:48:13
   |
   |
LL |     [usize; bool_to_usize(N >= 5)]: {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:54:8
   |
   |
LL |   [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:54:38
   |
   |
LL |   [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:55:8
   |
   |
LL |   [(); bool_to_usize(N <= 5)]:, [(); bool_to_usize(N > 3)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:55:38
   |
   |
LL |   [(); bool_to_usize(N <= 5)]:, [(); bool_to_usize(N > 3)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:57:28
   |
   |
LL |     _a: ConstOption<usize, { N <= 0 }>,
   |                            ^^------^^
   |                              pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:59:28
   |
   |
LL |     _b: ConstOption<usize, { N <= 1 }>,
   |                            ^^------^^
   |                              pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:61:18
   |
   |
LL |     _c: ConstTwo<{ N <= 5 }, {N > 3}>,
   |                  ^^------^^
   |                    pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:61:30
   |
   |
LL |     _c: ConstTwo<{ N <= 5 }, {N > 3}>,
   |                              ^-----^
   |                               pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:54:8
   |
   |
LL |   [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:54:38
   |
   |
LL |   [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:55:8
   |
   |
LL |   [(); bool_to_usize(N <= 5)]:, [(); bool_to_usize(N > 3)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_cond.rs:55:38
   |
   |
LL |   [(); bool_to_usize(N <= 5)]:, [(); bool_to_usize(N > 3)]:,
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: aborting due to 16 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/bool_must_be_exhaustive.rs stdout ----

- error[E0308]: mismatched types
-   --> $DIR/bool_must_be_exhaustive.rs:28:5
+ error: overly complex generic constant
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:9:52
3    |
- LL | #[derive(Default)]
-    |          ------- in this derive macro expansion
+ LL | pub struct ConstOption<T, const S: bool> where [T; bool_to_usize(S)]:, {
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:10:13
+    |
+    |
+ LL |     _v: [T; bool_to_usize(S)]
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:21:58
+    |
+    |
+ LL | fn _test_func<const N: usize>() where ConstOption<usize, {N >= 5}>: Default,
+    |                                                          ^------^
+    |                                                           pointer casts are not allowed in generic constants
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:22:13
+    |
+    |
+ LL |     [usize; bool_to_usize(N >= 5)]: {
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:27:43
+    |
+    |
6 LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:27:73
+    |
+    |
+ LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:28:28
+    |
+    |
7 LL |     _a: ConstOption<usize, { N <= 0 }>,
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N <= 0 }`, found `true`
+    |                            ^^------^^
+    |                              pointer casts are not allowed in generic constants
9    |
9    |
-    = note: expected constant `{ N <= 0 }`
-               found constant `true`
-    = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = help: consider moving this anonymous constant into a `const` function
- error[E0308]: mismatched types
-   --> $DIR/bool_must_be_exhaustive.rs:30:5
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:30:28
+   --> $DIR/bool_must_be_exhaustive.rs:30:28
16    |
- LL | #[derive(Default)]
-    |          ------- in this derive macro expansion
- ...
20 LL |     _b: ConstOption<usize, { N <= 1 }>,
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N <= 1 }`, found `true`
+    |                            ^^------^^
+    |                              pointer casts are not allowed in generic constants
22    |
22    |
-    = note: expected constant `{ N <= 1 }`
-               found constant `true`
-    = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = help: consider moving this anonymous constant into a `const` function
- error: aborting due to 2 previous errors
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:27:43
+    |
+    |
+ LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
- For more information about this error, try `rustc --explain E0308`.
+ error: overly complex generic constant
+   --> $DIR/bool_must_be_exhaustive.rs:27:73
+    |
+    |
+ LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: aborting due to 10 previous errors
+ 
30 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/bool_must_be_exhaustive/bool_must_be_exhaustive.stderr
To only update this specific test, also pass `--test-args const-generics/bool_must_be_exhaustive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/bool_must_be_exhaustive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/bool_must_be_exhaustive/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:9:52
   |
LL | pub struct ConstOption<T, const S: bool> where [T; bool_to_usize(S)]:, {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:10:13
   |
   |
LL |     _v: [T; bool_to_usize(S)]
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:21:58
   |
   |
LL | fn _test_func<const N: usize>() where ConstOption<usize, {N >= 5}>: Default,
   |                                                          ^------^
   |                                                           pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:22:13
   |
   |
LL |     [usize; bool_to_usize(N >= 5)]: {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:27:43
   |
   |
LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:27:73
   |
   |
LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:28:28
   |
   |
LL |     _a: ConstOption<usize, { N <= 0 }>,
   |                            ^^------^^
   |                              pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:30:28
   |
   |
LL |     _b: ConstOption<usize, { N <= 1 }>,
   |                            ^^------^^
   |                              pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:27:43
   |
   |
LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/bool_must_be_exhaustive.rs:27:73
   |
   |
LL | pub struct Arg<const N: usize> where [(); bool_to_usize(N <= 0)]:, [(); bool_to_usize(N <= 1)]:, {
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: aborting due to 10 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/unified_with_assoc.rs stdout ----

+ error: overly complex generic constant
+   --> $DIR/unified_with_assoc.rs:20:43
+    |
+    |
+ LL | fn demo_func<const N: usize>() where Demo<{ N > 5 }>: WithAssoc,
+    |                                           ^^-----^^
+    |                                             pointer casts are not allowed in generic constants
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ error: overly complex generic constant
+   --> $DIR/unified_with_assoc.rs:21:14
+    |
+    |
+ LL |   [();Demo::<{N>5}>::VAL]:,
+    |              ^---^
+    |               pointer casts are not allowed in generic constants
+    |
+    |
+    = help: consider moving this anonymous constant into a `const` function
+ 
1 error[E0391]: cycle detected when building an abstract representation for `demo_func::{constant#1}`
2   --> $DIR/unified_with_assoc.rs:21:7


22 LL | |   [();Demo::<{N>5}>::VAL]:,
24 
- error: aborting due to previous error
+ error: aborting due to 3 previous errors
26 
26 
27 For more information about this error, try `rustc --explain E0391`.
28 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unified_with_assoc/unified_with_assoc.stderr
To only update this specific test, also pass `--test-args const-generics/unified_with_assoc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/unified_with_assoc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unified_with_assoc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unified_with_assoc/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/unified_with_assoc.rs:20:43
   |
LL | fn demo_func<const N: usize>() where Demo<{ N > 5 }>: WithAssoc,
   |                                           ^^-----^^
   |                                             pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/unified_with_assoc.rs:21:14
   |
   |
LL |   [();Demo::<{N>5}>::VAL]:,
   |              ^---^
   |               pointer casts are not allowed in generic constants
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error[E0391]: cycle detected when building an abstract representation for `demo_func::{constant#1}`
  --> /checkout/src/test/ui/const-generics/unified_with_assoc.rs:21:7
   |
LL |   [();Demo::<{N>5}>::VAL]:,
   |
   |
note: ...which requires building THIR for `demo_func::{constant#1}`...
  --> /checkout/src/test/ui/const-generics/unified_with_assoc.rs:21:7
   |
LL |   [();Demo::<{N>5}>::VAL]:,
   |       ^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `demo_func::{constant#1}`...
  --> /checkout/src/test/ui/const-generics/unified_with_assoc.rs:21:7
   |
LL |   [();Demo::<{N>5}>::VAL]:,
   |       ^^^^^^^^^^^^^^^^^^
   = note: ...which again requires building an abstract representation for `demo_func::{constant#1}`, completing the cycle
note: cycle used when checking that `demo_func` is well-formed
  --> /checkout/src/test/ui/const-generics/unified_with_assoc.rs:20:1
   |
LL | / fn demo_func<const N: usize>() where Demo<{ N > 5 }>: WithAssoc,
LL | |   [();Demo::<{N>5}>::VAL]:,

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0391`.
