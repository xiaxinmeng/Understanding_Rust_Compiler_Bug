plain

running 11402 tests
.................................................................................................... 100/11402
.......................................i........ii.................................................. 200/11402
......FF....F.......F............................................................................... 300/11402
.................................................................................................... 500/11402
...............................................................................................i.... 600/11402
....................................................................ii.............................. 700/11402
.................................................................................................... 800/11402
---
.................................................................................................... 1700/11402
.................................................................................................... 1800/11402
.....i.............................................................................................. 1900/11402
.................................................................................................... 2000/11402
.......................................F.......F.................................................... 2100/11402
..........FFF.....F....F..........................F..................F..F........................... 2200/11402
..............................................................................................FF.... 2300/11402
............................................................................................F....... 2400/11402
.................................F.....F............................................................ 2500/11402
.......FF......................................................................F...............i..i. 2600/11402
.................................................................................................... 2800/11402
iiiii............................................................................................... 2900/11402
.................................................................................................... 3000/11402
.................................................................................................... 3100/11402
---
.................................................................................................... 6100/11402
...............................................................................................ii.ii 6200/11402
.......i...i........................................................................................ 6300/11402
......................................................i..........................i.................. 6400/11402
.............................................FF....F................................................ 6500/11402
.....................................................................................ii............. 6700/11402
...............................i.................................................................... 6800/11402
.................................................................................................... 6900/11402
.................................................................................................... 7000/11402
.................................................................................................... 7000/11402
..........i.......................................F................................................. 7100/11402
.........ii................i.i..ii.................................................................. 7200/11402
.................................................................................................... 7300/11402
.................................................................................................... 7400/11402
.................................................................................................... 7500/11402
...........................................................i..ii.................................... 7600/11402
............FFFF.FF....F.F.ii.FFFFF................................................................. 7700/11402
................................i................................................................... 7900/11402
.........................i.......................................................................... 8000/11402
.................................................................................................... 8100/11402
.................................................................................................... 8200/11402
---

---- [ui] ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs#noopt stdout ----
diff of stderr:

7    = note: `#[deny(arithmetic_overflow)]` on by default
8 
9 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const NEG: i32 = -i32::MIN + T::NEG;
+    |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
11    |
11    |
12 LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);

13    |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
14 
15 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
+    |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
17    |
17    |
18 LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;

19    |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
20 
21 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
+    |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
+ 
+ error: this arithmetic operation will overflow
23    |
23    |
24 LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);

25    |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
26 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);
+    |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
27 error: this operation will panic at runtime
28   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:37:22
29    |


33    = note: `#[deny(unconditional_panic)]` on by default
34 
35 error: this operation will panic at runtime
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:37:22
+    |
+ LL |     const DIV: i32 = (1/0) + T::DIV;
+    |                      ^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
37    |
37    |
38 LL |     const DIV_REV: i32 = T::DIV + (1/0);

39    |                                   ^^^^^ attempt to divide `1_i32` by zero
41 error: this operation will panic at runtime
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:39:35
+    |
+    |
+ LL |     const DIV_REV: i32 = T::DIV + (1/0);
+    |                                   ^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
43    |
43    |
44 LL |     const OOB: i32 = [1][1] + T::OOB;
45    |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
46 
47 error: this operation will panic at runtime
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:42:22
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:42:22
+    |
+ LL |     const OOB: i32 = [1][1] + T::OOB;
+    |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
+ 
+ error: this operation will panic at runtime
49    |
49    |
50 LL |     const OOB_REV: i32 = T::OOB + [1][1];
51    |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
52 
- error: aborting due to 8 previous errors
- error: aborting due to 8 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     const OOB_REV: i32 = T::OOB + [1][1];
+    |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 16 previous errors
54 
55 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.noopt/issue-69020-assoc-const-arith-overflow.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-69020-assoc-const-arith-overflow.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     const NEG: i32 = -i32::MIN + T::NEG;
   |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     const NEG: i32 = -i32::MIN + T::NEG;
   |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
   |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
   |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
   |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
   |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);
   |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);
   |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:37:22
   |
   |
LL |     const DIV: i32 = (1/0) + T::DIV;
   |                      ^^^^^ attempt to divide `1_i32` by zero
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:37:22
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:37:22
   |
LL |     const DIV: i32 = (1/0) + T::DIV;
   |                      ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:39:35
   |
   |
LL |     const DIV_REV: i32 = T::DIV + (1/0);
   |                                   ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:39:35
   |
   |
LL |     const DIV_REV: i32 = T::DIV + (1/0);
   |                                   ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:42:22
   |
   |
LL |     const OOB: i32 = [1][1] + T::OOB;
   |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:42:22
   |
   |
LL |     const OOB: i32 = [1][1] + T::OOB;
   |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:44:35
   |
   |
LL |     const OOB_REV: i32 = T::OOB + [1][1];
   |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:44:35
   |
   |
LL |     const OOB_REV: i32 = T::OOB + [1][1];
   |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 16 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs#opt stdout ----
diff of stderr:

7    = note: `#[deny(arithmetic_overflow)]` on by default
8 
9 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const NEG: i32 = -i32::MIN + T::NEG;
+    |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
11    |
11    |
12 LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);

13    |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
14 
15 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
+    |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
17    |
17    |
18 LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;

19    |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
20 
21 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
+    |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
+ 
+ error: this arithmetic operation will overflow
23    |
23    |
24 LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);

25    |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
26 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);
+    |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
27 error: this operation will panic at runtime
28   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:37:22
29    |


33    = note: `#[deny(unconditional_panic)]` on by default
34 
35 error: this operation will panic at runtime
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:37:22
+    |
+ LL |     const DIV: i32 = (1/0) + T::DIV;
+    |                      ^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
37    |
37    |
38 LL |     const DIV_REV: i32 = T::DIV + (1/0);

39    |                                   ^^^^^ attempt to divide `1_i32` by zero
41 error: this operation will panic at runtime
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:39:35
+    |
+    |
+ LL |     const DIV_REV: i32 = T::DIV + (1/0);
+    |                                   ^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
43    |
43    |
44 LL |     const OOB: i32 = [1][1] + T::OOB;
45    |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
46 
47 error: this operation will panic at runtime
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:42:22
+   --> $DIR/issue-69020-assoc-const-arith-overflow.rs:42:22
+    |
+ LL |     const OOB: i32 = [1][1] + T::OOB;
+    |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
+ 
+ error: this operation will panic at runtime
49    |
49    |
50 LL |     const OOB_REV: i32 = T::OOB + [1][1];
51    |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
52 
- error: aborting due to 8 previous errors
- error: aborting due to 8 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     const OOB_REV: i32 = T::OOB + [1][1];
+    |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 16 previous errors
54 
55 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.opt/issue-69020-assoc-const-arith-overflow.opt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-69020-assoc-const-arith-overflow.rs`

error in revision `opt`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     const NEG: i32 = -i32::MIN + T::NEG;
   |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     const NEG: i32 = -i32::MIN + T::NEG;
   |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
   |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
   |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
   |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
   |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);
   |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);
   |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:37:22
   |
   |
LL |     const DIV: i32 = (1/0) + T::DIV;
   |                      ^^^^^ attempt to divide `1_i32` by zero
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:37:22
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:37:22
   |
LL |     const DIV: i32 = (1/0) + T::DIV;
   |                      ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:39:35
   |
   |
LL |     const DIV_REV: i32 = T::DIV + (1/0);
   |                                   ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:39:35
   |
   |
LL |     const DIV_REV: i32 = T::DIV + (1/0);
   |                                   ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:42:22
   |
   |
LL |     const OOB: i32 = [1][1] + T::OOB;
   |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:42:22
   |
   |
LL |     const OOB: i32 = [1][1] + T::OOB;
   |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:44:35
   |
   |
LL |     const OOB_REV: i32 = T::OOB + [1][1];
   |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs:44:35
   |
   |
LL |     const OOB_REV: i32 = T::OOB + [1][1];
   |                                   ^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 16 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/associated-consts/issue-69020-assoc-const-arith-overflow.rs#opt_with_overflow_checks stdout ----
diff of stderr:

7    = note: `#[deny(arithmetic_overflow)]` on by default
8 
9 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const NEG: i32 = -i32::MIN + T::NEG;
+    |                      ^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
11    |
11    |
12 LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);

13    |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
14 
15 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const NEG_REV: i32 = T::NEG + (-i32::MIN);
+    |                                   ^^^^^^^^^^^ attempt to negate `i32::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
17    |
17    |
18 LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;

19    |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
20 
21 error: this arithmetic operation will overflow
+    |
+    |
+ LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
+    |                      ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
+ 
+ error: this arithmetic operation will overflow
23    |
23    |
24 LL |     const ADD_REV: i32 =  T::ADD + (i32::MAX+1);

25    |                                    ^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
26 
+ error: this arithmetic operation will overflow
---
- error: aborting due to 3 previous errors
+ error: erroneous constant used
+   --> $DIR/defaults-not-assumed-fail.rs:34:5
+    |
+ LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
+ error: aborting due to 4 previous errors
30 
31 For more information about this error, try `rustc --explain E0080`.
32 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail/defaults-not-assumed-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/defaults-not-assumed-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |     const B: u8 = Self::A + 1;
   |                   |
   |                   |
   |                   attempt to compute `u8::MAX + 1_u8`, which would overflow
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:34:16
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:34:16
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |                ^^^^^^^^^^^^^ referenced constant has errors
error: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:34:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:34:5
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

---
10 LL | #![warn(unconditional_panic)]
11    |         ^^^^^^^^^^^^^^^^^^^
12 
- warning: 1 warning emitted
+ warning: this operation will panic at runtime
+   --> $DIR/ice-assert-fail-div-by-zero.rs:13:5
+    |
+ LL |     f.0 / 0;
+    |     ^^^^^^^ attempt to divide `_` by zero
+ warning: 2 warnings emitted
14 
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/ice-assert-fail-div-by-zero/ice-assert-fail-div-by-zero.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const_prop/ice-assert-fail-div-by-zero.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const_prop/ice-assert-fail-div-by-zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/ice-assert-fail-div-by-zero" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "--emit=mir,link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/ice-assert-fail-div-by-zero/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this operation will panic at runtime
  --> /checkout/src/test/ui/const_prop/ice-assert-fail-div-by-zero.rs:13:5
   |
LL |     f.0 / 0; //~ WARN will panic at runtime
   |     ^^^^^^^ attempt to divide `_` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/const_prop/ice-assert-fail-div-by-zero.rs:7:9
   |
LL | #![warn(unconditional_panic)]
LL | #![warn(unconditional_panic)]
   |         ^^^^^^^^^^^^^^^^^^^

warning: this operation will panic at runtime
  --> /checkout/src/test/ui/const_prop/ice-assert-fail-div-by-zero.rs:13:5
   |
LL |     f.0 / 0; //~ WARN will panic at runtime
   |     ^^^^^^^ attempt to divide `_` by zero
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/consts/array-literal-index-oob.rs stdout ----
diff of stderr:

10 LL | #![warn(const_err, unconditional_panic)]
12 
- warning: 1 warning emitted
- warning: 1 warning emitted
+ warning: this operation will panic at runtime
+    |
+    |
+ LL |     &{ [1, 2, 3][4] };
+    |        ^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 4
+ warning: 2 warnings emitted
14 
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/array-literal-index-oob.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/array-literal-index-oob.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/array-literal-index-oob.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:7:8
   |
LL |     &{ [1, 2, 3][4] };
   |        ^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 4
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:4:20
   |
   |
LL | #![warn(const_err, unconditional_panic)]

warning: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/array-literal-index-oob.rs:7:8
   |
   |
LL |     &{ [1, 2, 3][4] };
   |        ^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 4
warning: 2 warnings emitted


------------------------------------------
---
44    |
45    = note: `#[deny(unconditional_panic)]` on by default
46 
- error: aborting due to 7 previous errors
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let a = -i8::MIN;
+    |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    |
+ LL |     let a_i128 = -i128::MIN;
+    |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let b = 200u8 + 200u8 + 200u8;
+    |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let b_i128 = i128::MIN - i128::MAX;
+    |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let c = 200u8 * 4;
+    |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let d = 42u8 - (42u8 + 1);
+    |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
+ 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     let _e = [5u8][1];
+    |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 14 previous errors
48 
49 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt/const-err2.opt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-err2.rs`

error in revision `opt`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let a = -i8::MIN;
   |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     let a_i128 = -i128::MIN;
   |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b = 200u8 + 200u8 + 200u8;
   |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b_i128 = i128::MIN - i128::MAX;
   |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let c = 200u8 * 4;
   |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let d = 42u8 - (42u8 + 1);
   |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-err2.rs:31:14
   |
   |
LL |     let _e = [5u8][1];
   |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default


error: this arithmetic operation will overflow
   |
   |
LL |     let a = -i8::MIN;
   |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let a_i128 = -i128::MIN;
   |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b = 200u8 + 200u8 + 200u8;
   |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b_i128 = i128::MIN - i128::MAX;
   |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let c = 200u8 * 4;
   |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let d = 42u8 - (42u8 + 1);
   |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-err2.rs:31:14
   |
   |
LL |     let _e = [5u8][1];
   |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 14 previous errors


------------------------------------------
---
44    |
45    = note: `#[deny(unconditional_panic)]` on by default
46 
- error: aborting due to 7 previous errors
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let a = -i8::MIN;
+    |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let a_i128 = -i128::MIN;
+    |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let b = 200u8 + 200u8 + 200u8;
+    |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let b_i128 = i128::MIN - i128::MAX;
+    |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let c = 200u8 * 4;
+    |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let d = 42u8 - (42u8 + 1);
+    |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
+ 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     let _e = [5u8][1];
+    |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 14 previous errors
48 
49 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.noopt/const-err2.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-err2.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let a = -i8::MIN;
   |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     let a_i128 = -i128::MIN;
   |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b = 200u8 + 200u8 + 200u8;
   |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b_i128 = i128::MIN - i128::MAX;
   |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let c = 200u8 * 4;
   |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let d = 42u8 - (42u8 + 1);
   |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-err2.rs:31:14
   |
   |
LL |     let _e = [5u8][1];
   |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default


error: this arithmetic operation will overflow
   |
   |
LL |     let a = -i8::MIN;
   |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let a_i128 = -i128::MIN;
   |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b = 200u8 + 200u8 + 200u8;
   |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b_i128 = i128::MIN - i128::MAX;
   |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let c = 200u8 * 4;
   |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let d = 42u8 - (42u8 + 1);
   |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-err2.rs:31:14
   |
   |
LL |     let _e = [5u8][1];
   |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 14 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-err.rs stdout ----
diff of stderr:

26 LL |     black_box((FOO, FOO));
27    |                     ^^^ referenced constant has errors
- error: aborting due to 2 previous errors; 1 warning emitted
- error: aborting due to 2 previous errors; 1 warning emitted
+ error[E0080]: erroneous constant used
+    |
+    |
+ LL |     black_box((FOO, FOO));
+    |                ^^^ referenced constant has errors
+ 
+ error[E0080]: erroneous constant used
+    |
+    |
+ LL |     black_box((FOO, FOO));
+    |                     ^^^ referenced constant has errors
+ error: aborting due to 4 previous errors; 1 warning emitted
30 
31 For more information about this error, try `rustc --explain E0080`.
32 
32 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/const-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-err.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zforce-overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL | const FOO: u8 = [5u8][1];
   |                 |
   |                 index out of bounds: the length is 1 but the index is 1
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-err.rs:5:9
   |
LL | #![warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: erroneous constant used
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err.rs:16:16
   |
LL |     black_box((FOO, FOO));
   |                ^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err.rs:16:21
   |
   |
LL |     black_box((FOO, FOO));
   |                     ^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err.rs:16:16
   |
   |
LL |     black_box((FOO, FOO));
   |                ^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err.rs:16:21
   |
   |
LL |     black_box((FOO, FOO));
   |                     ^^^ referenced constant has errors
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.

---
44    |
45    = note: `#[deny(unconditional_panic)]` on by default
46 
- error: aborting due to 7 previous errors
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let a = -i8::MIN;
+    |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let a_i128 = -i128::MIN;
+    |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let b = 200u8 + 200u8 + 200u8;
+    |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let b_i128 = i128::MIN - i128::MAX;
+    |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let c = 200u8 * 4;
+    |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow
+ 
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let d = 42u8 - (42u8 + 1);
+    |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
+ 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     let _e = [5u8][1];
+    |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 14 previous errors
48 
49 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt_with_overflow_checks/const-err2.opt_with_overflow_checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-err2.rs`

error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt_with_overflow_checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let a = -i8::MIN;
   |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     let a_i128 = -i128::MIN;
   |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b = 200u8 + 200u8 + 200u8;
   |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b_i128 = i128::MIN - i128::MAX;
   |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let c = 200u8 * 4;
   |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let d = 42u8 - (42u8 + 1);
   |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-err2.rs:31:14
   |
   |
LL |     let _e = [5u8][1];
   |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default


error: this arithmetic operation will overflow
   |
   |
LL |     let a = -i8::MIN;
   |             ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let a_i128 = -i128::MIN;
   |                  ^^^^^^^^^^ attempt to negate `i128::MIN`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b = 200u8 + 200u8 + 200u8;
   |             ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let b_i128 = i128::MIN - i128::MAX;
   |                  ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i128::MIN - i128::MAX`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let c = 200u8 * 4;
   |             ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     let d = 42u8 - (42u8 + 1);
   |             ^^^^^^^^^^^^^^^^^ attempt to compute `42_u8 - 43_u8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-err2.rs:31:14
   |
   |
LL |     let _e = [5u8][1];
   |              ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 14 previous errors


------------------------------------------
---
- error: aborting due to previous error; 2 warnings emitted
+ warning: erroneous constant used
+   --> $DIR/conditional_array_execution.rs:12:20
+    |
+ LL |     println!("{}", FOO);
+    |                    ^^^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ error: aborting due to previous error; 3 warnings emitted
+ error: aborting due to previous error; 3 warnings emitted
33 
34 For more information about this error, try `rustc --explain E0080`.
35 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/conditional_array_execution.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/conditional_array_execution.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
   |                   |
   |                   |
   |                   attempt to compute `5_u32 - 6_u32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:3:9
   |
   |
LL | #![warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:12:20
   |
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:12:20
   |
   |
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: erroneous constant used
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:12:20
   |
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to previous error; 3 warnings emitted
---

---- [ui] ui/consts/const-eval/erroneous-const.rs stdout ----
diff of stderr:

10 LL | #![warn(const_err, unconditional_panic)]
12 
12 
+ warning: this operation will panic at runtime
+    |
+    |
+ LL |     const VOID: () = [()][2];
+    |                      ^^^^^^^ index out of bounds: the length is 1 but the index is 2
+ 
13 warning: any use of this value will cause an error
15    |


38 LL | pub static FOO: () = no_codegen::<i32>();
39    |                      ------------------- inside `FOO` at $DIR/erroneous-const.rs:17:22
- error: aborting due to previous error; 2 warnings emitted
+ error: aborting due to previous error; 3 warnings emitted
42 
43 For more information about this error, try `rustc --explain E0080`.
43 For more information about this error, try `rustc --explain E0080`.
44 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/erroneous-const/erroneous-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/erroneous-const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/erroneous-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/erroneous-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/erroneous-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:6:22
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |                      ^^^^^^^ index out of bounds: the length is 1 but the index is 2
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:2:20
   |
   |
LL | #![warn(const_err, unconditional_panic)]

warning: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:6:22
   |
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |                      ^^^^^^^ index out of bounds: the length is 1 but the index is 2

warning: any use of this value will cause an error
   |
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |                      |
   |                      index out of bounds: the length is 1 but the index is 2
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:2:9
   |
LL | #![warn(const_err, unconditional_panic)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: could not evaluate static initializer
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:13:17
   |
LL |         let _ = PrintName::<T>::VOID; //~ERROR could not evaluate static initializer
   |                 |
   |                 |
   |                 referenced constant has errors
   |                 inside `no_codegen::<i32>` at /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:13:17
...
LL | pub static FOO: () = no_codegen::<i32>();
   |                      ------------------- inside `FOO` at /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:17:22
error: aborting due to previous error; 3 warnings emitted

For more information about this error, try `rustc --explain E0080`.

---
- error: aborting due to 2 previous errors; 4 warnings emitted
+ warning: erroneous constant used
+   --> $DIR/issue-43197.rs:16:23
+    |
+ LL |     println!("{} {}", X, Y);
+    |                       ^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ warning: erroneous constant used
+ warning: erroneous constant used
+   --> $DIR/issue-43197.rs:16:26
+    |
+ LL |     println!("{} {}", X, Y);
+    |                          ^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ error: aborting due to 2 previous errors; 6 warnings emitted
+ error: aborting due to 2 previous errors; 6 warnings emitted
59 
60 For more information about this error, try `rustc --explain E0080`.
61 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/issue-43197.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/issue-43197.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-43197.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL |     const X: u32 = 0 - 1;
   |                    |
   |                    |
   |                    attempt to compute `0_u32 - 1_u32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:3:9
   |
   |
LL | #![warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


warning: any use of this value will cause an error
   |
   |
LL |     const Y: u32 = foo(0 - 1);
   |                        |
   |                        |
   |                        attempt to compute `0_u32 - 1_u32`, which would overflow
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:23
   |
LL |     println!("{} {}", X, Y);
   |                       ^ referenced constant has errors
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:23
   |
   |
LL |     println!("{} {}", X, Y);
   |                       ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:26
   |
LL |     println!("{} {}", X, Y);
   |                          ^ referenced constant has errors
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:26
   |
   |
LL |     println!("{} {}", X, Y);
   |                          ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: erroneous constant used
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:23
   |
LL |     println!("{} {}", X, Y);
   |                       ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: erroneous constant used
warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:26
   |
LL |     println!("{} {}", X, Y);
   |                          ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors; 6 warnings emitted
---
6    |
7    = note: `#[deny(unconditional_panic)]` on by default
8 
- error: aborting due to previous error
+ error: this operation will panic at runtime
+    |
+ LL |     array[1];
+    |     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ 
+ 
+ error: aborting due to 2 previous errors
10 
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index_out_of_bounds_propagated/index_out_of_bounds_propagated.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/index_out_of_bounds_propagated.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/index_out_of_bounds_propagated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index_out_of_bounds_propagated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index_out_of_bounds_propagated/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-eval/index_out_of_bounds_propagated.rs:5:5
   |
LL |     array[1]; //~ ERROR operation will panic
   |     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-eval/index_out_of_bounds_propagated.rs:5:5
  --> /checkout/src/test/ui/consts/const-eval/index_out_of_bounds_propagated.rs:5:5
   |
LL |     array[1]; //~ ERROR operation will panic
   |     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 2 previous errors


------------------------------------------
---
6    |
7    = note: `#[deny(unconditional_panic)]` on by default
8 
- error: aborting due to previous error
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     [0; 3][3u64 as usize];
+    |     ^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 3
+ error: aborting due to 2 previous errors
10 
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice/const-prop-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-prop-ice.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-prop-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-prop-ice.rs:4:5
   |
LL |     [0; 3][3u64 as usize]; //~ ERROR this operation will panic at runtime
   |     ^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 3
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-prop-ice.rs:4:5
  --> /checkout/src/test/ui/consts/const-prop-ice.rs:4:5
   |
LL |     [0; 3][3u64 as usize]; //~ ERROR this operation will panic at runtime
   |     ^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 3
error: aborting due to 2 previous errors


------------------------------------------
---
6    |
7    = note: `#[deny(unconditional_panic)]` on by default
8 
- error: aborting due to previous error
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     println!("{}", xs[Enum::One as usize]);
+    |                    ^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 2 previous errors
10 
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2/const-prop-ice2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-prop-ice2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-prop-ice2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-prop-ice2.rs:6:20
   |
LL |     println!("{}", xs[Enum::One as usize]); //~ ERROR this operation will panic at runtime
   |                    ^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/const-prop-ice2.rs:6:20
  --> /checkout/src/test/ui/consts/const-prop-ice2.rs:6:20
   |
LL |     println!("{}", xs[Enum::One as usize]); //~ ERROR this operation will panic at runtime
   |                    ^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 2 previous errors


------------------------------------------
---
- error: aborting due to 2 previous errors; 1 warning emitted
+ error: erroneous constant used
+   --> $DIR/const_unsafe_unreachable_ub.rs:17:3
+    |
+ LL |   assert_eq!(BAR, true);
+    |   ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
+ error: aborting due to 3 previous errors; 1 warning emitted
43 
44 For more information about this error, try `rustc --explain E0080`.
45 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/const_unsafe_unreachable_ub.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_unsafe_unreachable_ub.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL |     unsafe { intrinsics::unreachable() }
   |              |
   |              entering unreachable code
   |              entering unreachable code
   |              inside `unreachable_unchecked` at /checkout/library/core/src/hint.rs:51:14
   |              inside `foo` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:9:18
   |              inside `BAR` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:14:28
  ::: /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:14:1
   |
   |
LL | const BAR: bool = unsafe { foo(false) };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:13:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:17:14
   |
LL |   assert_eq!(BAR, true);
   |              ^^^ referenced constant has errors
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:17:3
   |
   |
LL |   assert_eq!(BAR, true);
   |   ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: erroneous constant used
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:17:3
   |
LL |   assert_eq!(BAR, true);
   |   ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

---

---- [ui] ui/consts/issue-54348.rs stdout ----
diff of stderr:

12 LL |     [1][1u64 as usize];
13    |     ^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     [1][1.5 as usize];
+    |     ^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     [1][1u64 as usize];
+    |     ^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+ error: aborting due to 4 previous errors
16 
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54348/issue-54348.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-54348.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-54348.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54348" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54348/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/issue-54348.rs:5:5
   |
LL |     [1][1.5 as usize]; //~ ERROR operation will panic
   |     ^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/issue-54348.rs:6:5
  --> /checkout/src/test/ui/consts/issue-54348.rs:6:5
   |
LL |     [1][1u64 as usize]; //~ ERROR operation will panic
   |     ^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/issue-54348.rs:5:5
   |
   |
LL |     [1][1.5 as usize]; //~ ERROR operation will panic
   |     ^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: this operation will panic at runtime
  --> /checkout/src/test/ui/consts/issue-54348.rs:6:5
   |
   |
LL |     [1][1u64 as usize]; //~ ERROR operation will panic
   |     ^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
error: aborting due to 4 previous errors


------------------------------------------
---
- error: aborting due to 2 previous errors
+ error: erroneous constant used
+   --> $DIR/issue-55878.rs:7:26
+    |
+ LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
+    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ error: aborting due to 3 previous errors
+ error: aborting due to 3 previous errors
23 
24 For more information about this error, try `rustc --explain E0080`.
25 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-55878/issue-55878.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-55878.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-55878" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-55878/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
   |
LL |     intrinsics::size_of::<T>()
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ inside `std::mem::size_of::<[u8; 18446744073709551615]>` at /checkout/library/core/src/mem/mod.rs:310:5
  ::: /checkout/src/test/ui/consts/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());

error: erroneous constant used
  --> /checkout/src/test/ui/consts/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: erroneous constant used
  --> /checkout/src/test/ui/consts/issue-55878.rs:7:26
  --> /checkout/src/test/ui/consts/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 3 previous errors
---

---- [ui] ui/consts/miri_unleashed/assoc_const.rs stdout ----
diff of stderr:

4 LL |     let y = <String as Bar<Vec<u32>, String>>::F;
5    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
6 
+ error[E0080]: erroneous constant used
+    |
+    |
+ LL |     let y = <String as Bar<Vec<u32>, String>>::F;
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
7 warning: skipping const checks
8    |
8    |
9 help: skipping check that does not even have a feature gate

12 LL |     const F: u32 = (U::X, 42).1;
14 
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to 2 previous errors; 1 warning emitted
16 
16 
17 For more information about this error, try `rustc --explain E0080`.
18 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/assoc_const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:31:13
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ ERROR erroneous constant
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:31:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ ERROR erroneous constant
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL |     const F: u32 = (U::X, 42).1;

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/consts/miri_unleashed/assoc_const_2.rs stdout ----
diff of stderr:

4 LL |     let y = <String as Bar<String>>::F;
5    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0080]: erroneous constant used
+    |
+    |
+ LL |     let y = <String as Bar<String>>::F;
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+ error: aborting due to 2 previous errors
8 
9 For more information about this error, try `rustc --explain E0080`.
10 
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/assoc_const_2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const_2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:29:13
   |
LL |     let y = <String as Bar<String>>::F; //~ ERROR erroneous constant
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:29:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ ERROR erroneous constant
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/consts/uninhabited-const-issue-61744.rs stdout ----
diff of stderr:

147 LL |     dbg!(i32::CONSTANT);
148    |          ^^^^^^^^^^^^^ referenced constant has errors
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0080]: erroneous constant used
+    |
+    |
+ LL |     dbg!(i32::CONSTANT);
+    |          ^^^^^^^^^^^^^ referenced constant has errors
+ error: aborting due to 3 previous errors
151 
152 For more information about this error, try `rustc --explain E0080`.
153 
153 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/uninhabited-const-issue-61744.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |     hint_unreachable() //~ ERROR any use of this value will cause an error [const_err]
   |     |
   |     |
   |     reached the configured maximum number of stack frames
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<!>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `hint_unreachable` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:9:5
   |     inside `fake_type::<i32>` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:4:5
   |     inside `<i32 as Const>::CONSTANT` at /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:13:36
...
LL |     const CONSTANT: i32 = unsafe { fake_type() };
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:19:10
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:19:10
   |
LL |     dbg!(i32::CONSTANT); //~ ERROR erroneous constant used
   |          ^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:19:10
   |
   |
LL |     dbg!(i32::CONSTANT); //~ ERROR erroneous constant used
   |          ^^^^^^^^^^^^^ referenced constant has errors
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.

---
diff of stderr:

11    |         ^^^^^^^^^^^^^^^^^^^
12 
13 warning: this arithmetic operation will overflow
+    |
+    |
+ LL |     const N: i32 = T::N << 42;
+    |                    ^^^^^^^^^^ attempt to shift left by `42_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
15    |
15    |
16 LL |     let _ = x << 42;

17    |             ^^^^^^^ attempt to shift left by `42_i32`, which would overflow
18 
19 warning: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _ = x << 42;
+    |             ^^^^^^^ attempt to shift left by `42_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
21    |
21    |
22 LL |       let n = 1u8 << 8;

148 LL |       let n = 1_usize << BITS;
149    |               ^^^^^^^^^^^^^^^ attempt to shift left by `%BITS%`, which would overflow
- warning: 24 warnings emitted
- warning: 24 warnings emitted
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u8 << 8;
+    |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u16 << 16;
+    |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u32 << 32;
+    |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u64 << 64;
+    |               ^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i8 << 8;
+    |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i16 << 16;
+    |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i32 << 32;
+    |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i64 << 64;
+    |               ^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u8 >> 8;
+    |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u16 >> 16;
+    |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u32 >> 32;
+    |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u64 >> 64;
+    |               ^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i8 >> 8;
+    |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i16 >> 16;
+    |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i32 >> 32;
+    |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i64 >> 64;
+    |               ^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = n << 8;
+    |               ^^^^^^ attempt to shift left by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u8 << -8;
+    |               ^^^^^^^^^ attempt to shift left by `-8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1u8 << (4+4);
+    |               ^^^^^^^^^^^^ attempt to shift left by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1i64 >> [64][0];
+    |               ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1_isize << BITS;
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by `%BITS%`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+    |
+ LL |       let n = 1_usize << BITS;
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by `%BITS%`, which would overflow
+ warning: 48 warnings emitted
152 
153 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/lint-exceeding-bitshifts.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this arithmetic operation will overflow
   |
   |
LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
   |                    ^^^^^^^^^^ attempt to shift left by `42_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:10:9
   |
   |
LL | #![warn(arithmetic_overflow, const_err)]


warning: this arithmetic operation will overflow
   |
   |
LL |     const N: i32 = T::N << 42; //~ WARN: arithmetic operation will overflow
   |                    ^^^^^^^^^^ attempt to shift left by `42_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
   |             ^^^^^^^ attempt to shift left by `42_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |     let _ = x << 42; //~ WARN: arithmetic operation will overflow
   |             ^^^^^^^ attempt to shift left by `42_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^ attempt to shift left by `-8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by `64_usize`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by `64_usize`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i8 << 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i16 << 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i32 << 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i64 << 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i8 >> 8;   //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i16 >> 16; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i32 >> 32; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i64 >> 64; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = n << 8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 << -8; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^ attempt to shift left by `-8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1u8 << (4+4); //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^ attempt to shift left by `8_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1i64 >> [64][0]; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1_isize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by `64_usize`, which would overflow

warning: this arithmetic operation will overflow
   |
   |
LL |       let n = 1_usize << BITS; //~ WARN: arithmetic operation will overflow
   |               ^^^^^^^^^^^^^^^ attempt to shift left by `64_usize`, which would overflow
warning: 48 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt stdout ----
diff of stderr:

11    |         ^^^^^^^^^^^^^^^^^^^
12 
13 warning: this arithmetic operation will overflow
+    |
+    |
+ LL |     const N: i32 = T::N << 42;
+    |                    ^^^^^^^^^^ attempt to shift left by `42_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
15    |
15    |
16 LL |     let _ = x << 42;

148 LL |       let n = 1_usize << BITS;
---
8 
9 error: this operation will panic at runtime
+   --> $DIR/mir_detects_invalid_ops.rs:11:14
+    |
+ LL |     let _z = 1 / y;
+    |              ^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
11    |
11    |
12 LL |     let _z = 1 % y;

13    |              ^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     let _z = 1 % y;
+    |              ^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ error: aborting due to 4 previous errors
16 
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_detects_invalid_ops/mir_detects_invalid_ops.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir/mir_detects_invalid_ops.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir_detects_invalid_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_detects_invalid_ops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_detects_invalid_ops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this operation will panic at runtime
  --> /checkout/src/test/ui/mir/mir_detects_invalid_ops.rs:11:14
   |
LL |     let _z = 1 / y; //~ ERROR this operation will panic at runtime [unconditional_panic]
   |              ^^^^^ attempt to divide `1_i32` by zero
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/mir/mir_detects_invalid_ops.rs:11:14
  --> /checkout/src/test/ui/mir/mir_detects_invalid_ops.rs:11:14
   |
LL |     let _z = 1 / y; //~ ERROR this operation will panic at runtime [unconditional_panic]
   |              ^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/mir/mir_detects_invalid_ops.rs:16:14
   |
   |
LL |     let _z = 1 % y; //~ ERROR this operation will panic at runtime [unconditional_panic]
   |              ^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/mir/mir_detects_invalid_ops.rs:16:14
   |
   |
LL |     let _z = 1 % y; //~ ERROR this operation will panic at runtime [unconditional_panic]
   |              ^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: aborting due to 4 previous errors


------------------------------------------
---
46 
47 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:25:36
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
+ 
+ error: this operation will panic at runtime
49    |
49    |
50 LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());

51    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
53 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:27:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
+ 
+ error: this operation will panic at runtime
55    |
55    |
56 LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());

57    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
59 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:29:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
+ 
+ error: this operation will panic at runtime
61    |
61    |
62 LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());

63    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
65 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:31:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
67    |
67    |
68 LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());

69    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
71 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:33:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
+ 
+ error: this operation will panic at runtime
73    |
73    |
74 LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());

75    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
76 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
+ 
77 error: this arithmetic operation will overflow
79    |


117    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
119 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:49:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
+ 
+ error: this operation will panic at runtime
121    |
121    |
122 LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());

123    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
125 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:51:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
+ 
+ error: this operation will panic at runtime
127    |
127    |
128 LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());

129    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
131 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:53:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
+ 
+ error: this operation will panic at runtime
133    |
133    |
134 LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());

135    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
137 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:55:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ 
+ error: this operation will panic at runtime
139    |
139    |
140 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());

141    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
143 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:57:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
+ 
+ error: this operation will panic at runtime
145    |
145    |
146 LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());

147    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
- error: aborting due to 24 previous errors
- error: aborting due to 24 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
+ error: aborting due to 36 previous errors
150 
151 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.noopt/issue-8460-const.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/issue-8460-const.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:27:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to divide `1_i8` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:27:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to divide `1_i8` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:29:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:29:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:31:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:31:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:33:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:33:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:35:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:35:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:49:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:49:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:51:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:51:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:53:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:53:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:55:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:55:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:57:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:57:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:59:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:59:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
error: aborting due to 36 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-lsh-2.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _x = 1 << -1;
+    |              ^^^^^^^ attempt to shift left by `-1_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-2/overflowing-lsh-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _x = 1 << -1;
   |              ^^^^^^^ attempt to shift left by `-1_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-2.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _x = 1 << -1;
   |              ^^^^^^^ attempt to shift left by `-1_i32`, which would overflow
error: aborting due to 2 previous errors


------------------------------------------
---
46 
47 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:25:36
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
+ 
+ error: this operation will panic at runtime
49    |
49    |
50 LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());

51    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
53 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:27:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
+ 
+ error: this operation will panic at runtime
55    |
55    |
56 LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());

57    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
59 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:29:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
+ 
+ error: this operation will panic at runtime
61    |
61    |
62 LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());

63    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
65 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:31:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
67    |
67    |
68 LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());

69    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
71 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:33:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
+ 
+ error: this operation will panic at runtime
73    |
73    |
74 LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());

75    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
76 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
+ 
77 error: this arithmetic operation will overflow
79    |


117    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
119 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:49:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
+ 
+ error: this operation will panic at runtime
121    |
121    |
122 LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());

123    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
125 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:51:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
+ 
+ error: this operation will panic at runtime
127    |
127    |
128 LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());

129    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
131 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:53:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
+ 
+ error: this operation will panic at runtime
133    |
133    |
134 LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());

135    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
137 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:55:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ 
+ error: this operation will panic at runtime
139    |
139    |
140 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());

141    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
143 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:57:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
+ 
+ error: this operation will panic at runtime
145    |
145    |
146 LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());

147    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
- error: aborting due to 24 previous errors
- error: aborting due to 24 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
+ error: aborting due to 36 previous errors
150 
151 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt/issue-8460-const.opt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/issue-8460-const.rs`

error in revision `opt`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:27:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to divide `1_i8` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:27:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to divide `1_i8` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:29:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:29:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:31:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:31:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:33:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:33:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:35:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:35:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:49:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:49:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:51:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:51:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:53:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:53:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:55:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:55:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:57:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:57:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:59:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:59:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
error: aborting due to 36 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-lsh-1.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _x = 1_i32 << 32;
+    |              ^^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-1/overflowing-lsh-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _x = 1_i32 << 32;
   |              ^^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-1.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _x = 1_i32 << 32;
   |              ^^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow
error: aborting due to 2 previous errors


------------------------------------------
---
46 
47 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:25:36
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
+ 
+ error: this operation will panic at runtime
49    |
49    |
50 LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());

51    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
53 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:27:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
+ 
+ error: this operation will panic at runtime
55    |
55    |
56 LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());

57    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
59 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:29:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
+ 
+ error: this operation will panic at runtime
61    |
61    |
62 LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());

63    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
65 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:31:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
67    |
67    |
68 LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());

69    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
71 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:33:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
+ 
+ error: this operation will panic at runtime
73    |
73    |
74 LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());

75    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
76 
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
+ 
77 error: this arithmetic operation will overflow
79    |


117    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
119 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:49:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
+ 
+ error: this operation will panic at runtime
121    |
121    |
122 LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());

123    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
125 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:51:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
+ 
+ error: this operation will panic at runtime
127    |
127    |
128 LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());

129    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
131 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:53:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
+ 
+ error: this operation will panic at runtime
133    |
133    |
134 LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());

135    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
137 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:55:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ 
+ error: this operation will panic at runtime
139    |
139    |
140 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());

141    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
143 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:57:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
+ 
+ error: this operation will panic at runtime
145    |
145    |
146 LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());

147    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
- error: aborting due to 24 previous errors
- error: aborting due to 24 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
+ error: aborting due to 36 previous errors
150 
151 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt_with_overflow_checks/issue-8460-const.opt_with_overflow_checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/issue-8460-const.rs`

error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt_with_overflow_checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:27:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to divide `1_i8` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:27:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to divide `1_i8` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:29:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:29:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:31:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:31:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:33:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:33:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:35:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:35:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow

error: this arithmetic operation will overflow
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:49:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:49:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
   |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:51:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:51:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
   |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:53:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:53:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:55:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:55:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:57:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:57:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
   |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:59:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:59:36
   |
   |
LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
   |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
error: aborting due to 36 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-lsh-3.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _x = 1_u64 << 64;
+    |              ^^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-3/overflowing-lsh-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _x = 1_u64 << 64;
   |              ^^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-3.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _x = 1_u64 << 64;
   |              ^^^^^^^^^^^ attempt to shift left by `64_i32`, which would overflow
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-lsh-4.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let x = 1_i8 << 17;
+    |             ^^^^^^^^^^ attempt to shift left by `17_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-4/overflowing-lsh-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let x = 1_i8 << 17;
   |             ^^^^^^^^^^ attempt to shift left by `17_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-4.rs:7:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let x = 1_i8 << 17;
   |             ^^^^^^^^^^ attempt to shift left by `17_i32`, which would overflow
---
test result: FAILED. 11271 passed; 39 failed; 92 ignored; 0 measured; 0 filtered out; finished in 137.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:34
