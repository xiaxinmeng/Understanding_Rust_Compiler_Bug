plain
......................................................................i............................. 1900/12739
.................................................................................................... 2000/12739
.................................................................................................... 2100/12739
.................................................................................................... 2200/12739
....................................................................................F....F.F........ 2300/12739
...............................................................F..F.....FF.F........................ 2400/12739
.........................F......F................................................................... 2500/12739
..........................................................F.......F................................. 2600/12739
.................................................................................F.................. 2700/12739
.........................F.......................................................................... 2800/12739
.......F.FF...................................................................................F..... 2900/12739
.................................................................................................... 3100/12739
.............................iiiii.................................................................. 3200/12739
.................................................................................................... 3300/12739
.................................................................................................... 3400/12739
---
............................................ii.ii........i...i...................................... 6600/12739
.................................................................................................... 6700/12739
....................................i.....i............F..........................i................i 6800/12739
.............i...................................................i.................................. 6900/12739
......F.....F...F...................................................i............................... 7000/12739
.................................................................................................... 7200/12739
.....ii................................ii.........................................................i. 7300/12739
.................................................................................................... 7400/12739
.........................................................................i.......................... 7500/12739
.........................................................................i.......................... 7500/12739
.................................................................................................... 7600/12739
.................................................................F.................................. 7700/12739
...............................i.i...............i....i..ii......................................... 7800/12739
.................................................................................................... 7900/12739
.................................................................................................... 8000/12739
.................................................................................................... 8100/12739
.................................................................................i..ii.............. 8200/12739
.....................................F.FFF.....FiiFFF.F.F.......F................................... 8300/12739
..................................iiii.............................................................. 8400/12739
...................F....F..............................i.......................................i.... 8500/12739
.................................................................................................... 8700/12739
..................i................................................................................. 8800/12739
.................................................................................................i.. 8900/12739
.................................................................................................... 9000/12739
---
---- [ui] ui/associated-consts/defaults-not-assumed-fail.rs stdout ----
diff of stderr:

26    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
27    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 3 previous errors
+ error: erroneous constant used
+   --> $DIR/defaults-not-assumed-fail.rs:34:5
+    |
+    |
+ LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to 4 previous errors
30 
31 For more information about this error, try `rustc --explain E0080`.
32 
---
To only update this specific test, also pass `--test-args associated-consts/defaults-not-assumed-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail/auxiliary"
stdout: none
--- stderr -------------------------------
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
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:34:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
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

---
To only update this specific test, also pass `--test-args const_prop/ice-assert-fail-div-by-zero.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const_prop/ice-assert-fail-div-by-zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/ice-assert-fail-div-by-zero" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "--emit=mir,link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/ice-assert-fail-div-by-zero/auxiliary"
stdout: none
--- stderr -------------------------------
warning: this operation will panic at runtime
   |
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



---- [ui] ui/const_prop/inline_spans.rs stdout ----
diff of stderr:

9    |
10    = note: `#[deny(arithmetic_overflow)]` on by default
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _ = add(u8::MAX, 1);
+    |             --------------- in this inlined function call
+ LL |     x + y
+ LL |     x + y
+    |     ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
+ error: aborting due to 2 previous errors
13 
14 

---
To only update this specific test, also pass `--test-args const_prop/inline_spans.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const_prop/inline_spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/inline_spans" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/inline_spans/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _ = add(u8::MAX, 1);
   |             --------------- in this inlined function call
LL |     x + y
LL |     x + y
   |     ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
   |
   |
LL |     let _ = add(u8::MAX, 1);
   |             --------------- in this inlined function call
LL |     x + y
LL |     x + y
   |     ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
error: aborting due to 2 previous errors
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
To only update this specific test, also pass `--test-args consts/array-literal-index-oob.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/array-literal-index-oob.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/array-literal-index-oob/auxiliary"
stdout: none
--- stderr -------------------------------
warning: this operation will panic at runtime
   |
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



---- [ui] ui/consts/const-err.rs stdout ----
diff of stderr:

26 LL |     black_box((FOO, FOO));
27    |                     ^^^ referenced constant has errors
- error: aborting due to 2 previous errors; 1 warning emitted
+ error[E0080]: erroneous constant used
+   --> $DIR/const-err.rs:16:16
+    |
+    |
+ LL |     black_box((FOO, FOO));
+    |                ^^^ referenced constant has errors
+ error[E0080]: erroneous constant used
+   --> $DIR/const-err.rs:16:21
+    |
+    |
+ LL |     black_box((FOO, FOO));
+    |                     ^^^ referenced constant has errors
+ error: aborting due to 4 previous errors; 1 warning emitted
30 
31 For more information about this error, try `rustc --explain E0080`.
32 
---
To only update this specific test, also pass `--test-args consts/const-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-err.rs:11:17
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
To only update this specific test, also pass `--test-args consts/const-err2.rs`


error in revision `noopt`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.noopt/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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



---- [ui] ui/consts/const-err2.rs#opt_with_overflow_checks stdout ----
diff of stderr:

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
To only update this specific test, also pass `--test-args consts/const-err2.rs`


error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt_with_overflow_checks/auxiliary"
stdout: none
--- stderr -------------------------------
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



---- [ui] ui/consts/const-err2.rs#opt stdout ----
diff of stderr:

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
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt/const-err2.opt.stderr
To only update this specific test, also pass `--test-args consts/const-err2.rs`


error in revision `opt`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2.opt/auxiliary"
stdout: none
--- stderr -------------------------------
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
+    = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
---
To only update this specific test, also pass `--test-args consts/const-eval/conditional_array_execution.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:7:19
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
LL |     println!("{}", FOO);
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:12:20
   |
LL |     println!("{}", FOO);
   |                    ^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

---
- error: aborting due to 2 previous errors; 4 warnings emitted
+ warning: erroneous constant used
+   --> $DIR/issue-43197.rs:16:23
+    |
+ LL |     println!("{} {}", X, Y);
+    |                       ^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
+ warning: erroneous constant used
+   --> $DIR/issue-43197.rs:16:26
+    |
+ LL |     println!("{} {}", X, Y);
+    |                          ^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-43197.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-43197.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:10:20
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
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:13:24
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
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


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
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:23
   |
LL |     println!("{} {}", X, Y);
   |                       ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)


warning: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-43197.rs:16:26
   |
LL |     println!("{} {}", X, Y);
   |                          ^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this warning originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

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
---
To only update this specific test, also pass `--test-args consts/const-eval/index_out_of_bounds_propagated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/index_out_of_bounds_propagated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index_out_of_bounds_propagated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index_out_of_bounds_propagated/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
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



---- [ui] ui/consts/const-prop-ice.rs stdout ----
diff of stderr:

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

---
To only update this specific test, also pass `--test-args consts/const-prop-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-prop-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
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



---- [ui] ui/consts/const-prop-ice2.rs stdout ----
diff of stderr:

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

---
To only update this specific test, also pass `--test-args consts/const-prop-ice2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-prop-ice2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
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
- error: aborting due to 2 previous errors
+ error: erroneous constant used
+   --> $DIR/invalid-union.rs:42:25
+    |
+ LL |     let _: &'static _ = &C;
+    |                         ^^ referenced constant has errors
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ error: aborting due to 3 previous errors
+ error: aborting due to 3 previous errors
23 
24 For more information about this error, try `rustc --explain E0080`.
25 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.64bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ type validation failed at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 3 previous errors
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

---
To only update this specific test, also pass `--test-args consts/issue-54348.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-54348.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54348" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54348/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
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



---- [ui] ui/consts/miri_unleashed/assoc_const_2.rs stdout ----
diff of stderr:

4 LL |     let y = <String as Bar<String>>::F;
5    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
- error: aborting due to previous error
+ error[E0080]: erroneous constant used
+   --> $DIR/assoc_const_2.rs:29:13
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
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: erroneous constant used
   |
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
------------------------------------------


---- [ui] ui/consts/miri_unleashed/assoc_const.rs stdout ----
diff of stderr:

4 LL |     let y = <String as Bar<Vec<u32>, String>>::F;
5    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+ error[E0080]: erroneous constant used
+   --> $DIR/assoc_const.rs:31:13
+    |
+    |
+ LL |     let y = <String as Bar<Vec<u32>, String>>::F;
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
7 warning: skipping const checks
8    |
9 help: skipping check that does not even have a feature gate


12 LL |     const F: u32 = (U::X, 42).1;
14 
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to 2 previous errors; 1 warning emitted
16 
---
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: erroneous constant used
   |
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
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:14:20
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:14:20
   |
LL |     const F: u32 = (U::X, 42).1;

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] ui/consts/miri_unleashed/const_refers_to_static.rs stdout ----
diff of stderr:

16 LL |     READ_MUT;
17    |     ^^^^^^^^ referenced constant has errors
+ error[E0080]: erroneous constant used
+   --> $DIR/const_refers_to_static.rs:25:5
+    |
+ LL |     MUTATE_INTERIOR_MUT;
+ LL |     MUTATE_INTERIOR_MUT;
+    |     ^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+ error[E0080]: erroneous constant used
+   --> $DIR/const_refers_to_static.rs:27:5
+    |
+ LL |     READ_INTERIOR_MUT;
+ LL |     READ_INTERIOR_MUT;
+    |     ^^^^^^^^^^^^^^^^^ referenced constant has errors
+ error[E0080]: erroneous constant used
+   --> $DIR/const_refers_to_static.rs:29:5
+    |
+ LL |     READ_MUT;
+ LL |     READ_MUT;
+    |     ^^^^^^^^ referenced constant has errors
19 warning: skipping const checks
20    |
21 help: skipping check that does not even have a feature gate


44 LL | const READ_MUT: u32 = unsafe { MUTABLE };
46 
- error: aborting due to 3 previous errors; 1 warning emitted
+ error: aborting due to 6 previous errors; 1 warning emitted
48 
---
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: erroneous constant used
   |
LL |     MUTATE_INTERIOR_MUT;
LL |     MUTATE_INTERIOR_MUT;
   |     ^^^^^^^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:27:5
   |
LL |     READ_INTERIOR_MUT;
LL |     READ_INTERIOR_MUT;
   |     ^^^^^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:29:5
   |
LL |     READ_MUT;
LL |     READ_MUT;
   |     ^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:25:5
   |
LL |     MUTATE_INTERIOR_MUT;
LL |     MUTATE_INTERIOR_MUT;
   |     ^^^^^^^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:27:5
   |
LL |     READ_INTERIOR_MUT;
LL |     READ_INTERIOR_MUT;
   |     ^^^^^^^^^^^^^^^^^ referenced constant has errors
error[E0080]: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:29:5
   |
LL |     READ_MUT;
LL |     READ_MUT;
   |     ^^^^^^^^ referenced constant has errors
warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:13:5
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:13:5
   |
LL |     FOO.fetch_add(1, Ordering::Relaxed)
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:13:5
   |
   |
LL |     FOO.fetch_add(1, Ordering::Relaxed)
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:18:17
   |
   |
LL |     unsafe { *(&FOO as *const _ as *const usize) }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:22:32
   |
   |
LL | const READ_MUT: u32 = unsafe { MUTABLE };
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:22:32
   |
   |
LL | const READ_MUT: u32 = unsafe { MUTABLE };

error: aborting due to 6 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] ui/consts/uninhabited-const-issue-61744.rs stdout ----
diff of stderr:

147 LL |     dbg!(i32::CONSTANT);
148    |          ^^^^^^^^^^^^^ referenced constant has errors
- error: aborting due to 2 previous errors
+ error[E0080]: erroneous constant used
+   --> $DIR/uninhabited-const-issue-61744.rs:19:10
+    |
+    |
+ LL |     dbg!(i32::CONSTANT);
+    |          ^^^^^^^^^^^^^ referenced constant has errors
+ error: aborting due to 3 previous errors
151 
152 For more information about this error, try `rustc --explain E0080`.
153 
---
To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary"
stdout: none
--- stderr -------------------------------
error: any use of this value will cause an error
   |
   |
LL |     hint_unreachable() //~ ERROR any use of this value will cause an error [const_err]
   |     |
   |     reached the configured maximum number of stack frames
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
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
---
To only update this specific test, also pass `--test-args limits/issue-55878.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/limits/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ inside `std::mem::size_of::<[u8; 18446744073709551615]>` at /checkout/library/core/src/mem/mod.rs:305:5
   |
   |
  ::: /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());

error: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: erroneous constant used
error: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

---

---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt stdout ----
diff of stderr:

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
22 LL |       let n = 1u8 << 8;

148 LL |       let n = 1_usize << BITS;
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
+ LL |       let n = 1i8 << 8;
+ LL |       let n = 1i8 << 8;
+    |               ^^^^^^^^ attempt to shift left by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1i16 << 16;
+ LL |       let n = 1i16 << 16;
+    |               ^^^^^^^^^^ attempt to shift left by `16_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1i32 << 32;
+ LL |       let n = 1i32 << 32;
+    |               ^^^^^^^^^^ attempt to shift left by `32_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1i64 << 64;
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
+ LL |       let n = 1i8 >> 8;
+ LL |       let n = 1i8 >> 8;
+    |               ^^^^^^^^ attempt to shift right by `8_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1i16 >> 16;
+ LL |       let n = 1i16 >> 16;
+    |               ^^^^^^^^^^ attempt to shift right by `16_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1i32 >> 32;
+ LL |       let n = 1i32 >> 32;
+    |               ^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1i64 >> 64;
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
+ LL |       let n = 1_isize << BITS;
+ LL |       let n = 1_isize << BITS;
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by `%BITS%`, which would overflow
+ 
+ warning: this arithmetic operation will overflow
+    |
+ LL |       let n = 1_usize << BITS;
+ LL |       let n = 1_usize << BITS;
+    |               ^^^^^^^^^^^^^^^ attempt to shift left by `%BITS%`, which would overflow
+ warning: 47 warnings emitted
152 
153 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/lint-exceeding-bitshifts.opt.stderr
To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts.rs`


error in revision `opt`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/auxiliary"
stdout: none
--- stderr -------------------------------
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
warning: 47 warnings emitted
------------------------------------------



---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt_with_overflow_checks stdout ----
diff of stderr:

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
22 LL |       let n = 1u8 << 8;

148 LL |       let n = 1_usize << BITS;
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

---
To only update this specific test, also pass `--test-args mir/mir_detects_invalid_ops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir_detects_invalid_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_detects_invalid_ops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_detects_invalid_ops/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
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

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-1/auxiliary"
stdout: none
--- stderr -------------------------------
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

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-2/auxiliary"
stdout: none
--- stderr -------------------------------
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

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-3/auxiliary"
stdout: none
--- stderr -------------------------------
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

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-lsh-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-lsh-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-lsh-4/auxiliary"
stdout: none
--- stderr -------------------------------
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
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/numbers-arithmetic/overflowing-rsh-2.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _x = -1_i32 >> -1;
+    |              ^^^^^^^^^^^^ attempt to shift right by `-1_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-rsh-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _x = -1_i32 >> -1;
   |              ^^^^^^^^^^^^ attempt to shift right by `-1_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-2.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _x = -1_i32 >> -1;
   |              ^^^^^^^^^^^^ attempt to shift right by `-1_i32`, which would overflow
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/numbers-arithmetic/overflowing-rsh-1.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _x = -1_i32 >> 32;
+    |              ^^^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-rsh-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _x = -1_i32 >> 32;
   |              ^^^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-1.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _x = -1_i32 >> 32;
   |              ^^^^^^^^^^^^ attempt to shift right by `32_i32`, which would overflow
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/numbers-arithmetic/overflowing-rsh-3.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _x = -1_i64 >> 64;
+    |              ^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-rsh-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _x = -1_i64 >> 64;
   |              ^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-3.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _x = -1_i64 >> 64;
   |              ^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/numbers-arithmetic/overflowing-rsh-6.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _n = 1i64 >> [64][0];
+    |              ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-rsh-6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-6/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _n = 1i64 >> [64][0];
   |              ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-6.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _n = 1i64 >> [64][0];
   |              ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/numbers-arithmetic/overflowing-rsh-5.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let _n = 1i64 >> [64][0];
+    |              ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-rsh-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-5/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let _n = 1i64 >> [64][0];
   |              ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-5.rs:4:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let _n = 1i64 >> [64][0];
   |              ^^^^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/numbers-arithmetic/overflowing-rsh-4.rs stdout ----
diff of stderr:

10 LL | #![deny(arithmetic_overflow, const_err)]
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: this arithmetic operation will overflow
+    |
+    |
+ LL |     let x = 2_i8 >> 17;
+    |             ^^^^^^^^^^ attempt to shift right by `17_i32`, which would overflow
+ error: aborting due to 2 previous errors
14 
15 

---
To only update this specific test, also pass `--test-args numbers-arithmetic/overflowing-rsh-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-rsh-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
   |
   |
LL |     let x = 2_i8 >> 17;
   |             ^^^^^^^^^^ attempt to shift right by `17_i32`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/numbers-arithmetic/overflowing-rsh-4.rs:7:9
   |
   |
LL | #![deny(arithmetic_overflow, const_err)]


error: this arithmetic operation will overflow
   |
   |
LL |     let x = 2_i8 >> 17;
   |             ^^^^^^^^^^ attempt to shift right by `17_i32`, which would overflow
error: aborting due to 2 previous errors
------------------------------------------


---
8 
9 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:13:36
+    |
+ LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
+ 
+ error: this operation will panic at runtime
11    |
11    |
12 LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());

13    |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
15 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:15:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
+ 
+ error: this operation will panic at runtime
17    |
17    |
18 LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());

19    |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
21 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:17:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
+ 
+ error: this operation will panic at runtime
23    |
23    |
24 LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());

25    |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
27 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:19:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
+ 
+ error: this operation will panic at runtime
29    |
29    |
30 LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());

31    |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
33 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:21:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
+ 
+ error: this operation will panic at runtime
35    |
35    |
36 LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());

37    |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
39 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:23:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
+ 
+ error: this operation will panic at runtime
41    |
41    |
42 LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());

43    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
45 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:25:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
+ 
+ error: this operation will panic at runtime
47    |
47    |
48 LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());

49    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
51 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:27:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
+ 
+ error: this operation will panic at runtime
53    |
53    |
54 LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());

55    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
57 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:29:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
+ 
+ error: this operation will panic at runtime
59    |
59    |
60 LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());

61    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
63 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:31:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
65    |
65    |
66 LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());

67    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
69 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:33:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
+ 
+ error: this operation will panic at runtime
71    |
71    |
72 LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());

73    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
75 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:35:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
+ 
+ error: this operation will panic at runtime
77    |
77    |
78 LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());

79    |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
81 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:37:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
+ 
+ error: this operation will panic at runtime
83    |
83    |
84 LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());

85    |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
87 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:39:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
+ 
+ error: this operation will panic at runtime
89    |
89    |
90 LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());

91    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
93 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:41:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
+ 
+ error: this operation will panic at runtime
95    |
95    |
96 LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());

97    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
99 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:43:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
+ 
+ error: this operation will panic at runtime
101    |
101    |
102 LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());

103    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
105 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:45:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
+ 
+ error: this operation will panic at runtime
107    |
107    |
108 LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());

109    |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
111 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:47:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
+ 
+ error: this operation will panic at runtime
113    |
113    |
114 LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());

115    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
117 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:49:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
+ 
+ error: this operation will panic at runtime
119    |
119    |
120 LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());

121    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
123 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:51:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
+ 
+ error: this operation will panic at runtime
125    |
125    |
126 LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());

127    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
129 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:53:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
+ 
+ error: this operation will panic at runtime
131    |
131    |
132 LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());

133    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
135 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:55:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ 
+ error: this operation will panic at runtime
137    |
137    |
138 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());

139    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
141 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:57:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
+ 
+ error: this operation will panic at runtime
143    |
143    |
144 LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());

145    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
- error: aborting due to 24 previous errors
- error: aborting due to 24 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
+ error: aborting due to 48 previous errors
148 
149 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.noopt/issue-8460-const.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/issue-8460-const.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.noopt/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:13:36
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:13:36
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:15:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:15:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:17:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:17:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:19:36
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:19:36
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:21:36
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:21:36
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:23:36
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:23:36
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
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
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
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:37:36
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:37:36
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:39:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:39:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:41:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:41:36
   |
---
8 
9 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:13:36
+    |
+ LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
+ 
+ error: this operation will panic at runtime
11    |
11    |
12 LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());

13    |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
15 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:15:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
+ 
+ error: this operation will panic at runtime
17    |
17    |
18 LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());

19    |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
21 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:17:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
+ 
+ error: this operation will panic at runtime
23    |
23    |
24 LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());

25    |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
27 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:19:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
+ 
+ error: this operation will panic at runtime
29    |
29    |
30 LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());

31    |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
33 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:21:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
+ 
+ error: this operation will panic at runtime
35    |
35    |
36 LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());

37    |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
39 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:23:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
+ 
+ error: this operation will panic at runtime
41    |
41    |
42 LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());

43    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
45 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:25:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
+ 
+ error: this operation will panic at runtime
47    |
47    |
48 LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());

49    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
51 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:27:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
+ 
+ error: this operation will panic at runtime
53    |
53    |
54 LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());

55    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
57 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:29:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
+ 
+ error: this operation will panic at runtime
59    |
59    |
60 LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());

61    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
63 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:31:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
65    |
65    |
66 LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());

67    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
69 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:33:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
+ 
+ error: this operation will panic at runtime
71    |
71    |
72 LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());

73    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
75 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:35:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
+ 
+ error: this operation will panic at runtime
77    |
77    |
78 LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());

79    |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
81 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:37:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
+ 
+ error: this operation will panic at runtime
83    |
83    |
84 LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());

85    |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
87 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:39:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
+ 
+ error: this operation will panic at runtime
89    |
89    |
90 LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());

91    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
93 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:41:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
+ 
+ error: this operation will panic at runtime
95    |
95    |
96 LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());

97    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
99 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:43:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
+ 
+ error: this operation will panic at runtime
101    |
101    |
102 LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());

103    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
105 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:45:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
+ 
+ error: this operation will panic at runtime
107    |
107    |
108 LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());

109    |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
111 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:47:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
+ 
+ error: this operation will panic at runtime
113    |
113    |
114 LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());

115    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
117 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:49:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
+ 
+ error: this operation will panic at runtime
119    |
119    |
120 LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());

121    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
123 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:51:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
+ 
+ error: this operation will panic at runtime
125    |
125    |
126 LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());

127    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
129 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:53:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
+ 
+ error: this operation will panic at runtime
131    |
131    |
132 LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());

133    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
135 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:55:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ 
+ error: this operation will panic at runtime
137    |
137    |
138 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());

139    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
141 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:57:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
+ 
+ error: this operation will panic at runtime
143    |
143    |
144 LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());

145    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
- error: aborting due to 24 previous errors
- error: aborting due to 24 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
+ error: aborting due to 48 previous errors
148 
149 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt_with_overflow_checks/issue-8460-const.opt_with_overflow_checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/issue-8460-const.rs`

error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt_with_overflow_checks/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:13:36
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:13:36
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:15:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:15:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:17:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:17:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:19:36
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:19:36
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:21:36
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:21:36
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:23:36
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:23:36
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
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
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
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:37:36
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:37:36
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:39:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:39:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:41:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:41:36
   |
---
8 
9 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:13:36
+    |
+ LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
+ 
+ error: this operation will panic at runtime
11    |
11    |
12 LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());

13    |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
15 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:15:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
+ 
+ error: this operation will panic at runtime
17    |
17    |
18 LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());

19    |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
21 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:17:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
+ 
+ error: this operation will panic at runtime
23    |
23    |
24 LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());

25    |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
27 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:19:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
+ 
+ error: this operation will panic at runtime
29    |
29    |
30 LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());

31    |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
33 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:21:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
+ 
+ error: this operation will panic at runtime
35    |
35    |
36 LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());

37    |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
39 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:23:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
+ 
+ error: this operation will panic at runtime
41    |
41    |
42 LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());

43    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
45 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:25:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize / 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to divide `1_isize` by zero
+ 
+ error: this operation will panic at runtime
47    |
47    |
48 LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());

49    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
51 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:27:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 / 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to divide `1_i8` by zero
+ 
+ error: this operation will panic at runtime
53    |
53    |
54 LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());

55    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
57 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:29:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i16` by zero
+ 
+ error: this operation will panic at runtime
59    |
59    |
60 LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());

61    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
63 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:31:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i32` by zero
+ 
+ error: this operation will panic at runtime
65    |
65    |
66 LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());

67    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
69 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:33:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 / 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to divide `1_i64` by zero
+ 
+ error: this operation will panic at runtime
71    |
71    |
72 LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());

73    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
75 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:35:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 / 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to divide `1_i128` by zero
+ 
+ error: this operation will panic at runtime
77    |
77    |
78 LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());

79    |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
81 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:37:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
+ 
+ error: this operation will panic at runtime
83    |
83    |
84 LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());

85    |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
87 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:39:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
+ 
+ error: this operation will panic at runtime
89    |
89    |
90 LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());

91    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
93 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:41:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
+ 
+ error: this operation will panic at runtime
95    |
95    |
96 LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());

97    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
99 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:43:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i32::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
+ 
+ error: this operation will panic at runtime
101    |
101    |
102 LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());

103    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
105 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:45:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i64::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
+ 
+ error: this operation will panic at runtime
107    |
107    |
108 LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());

109    |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
111 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:47:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { i128::MIN % -1; }).join().is_err());
+    |                                    ^^^^^^^^^^^^^^ attempt to compute the remainder of `i128::MIN % -1_i128`, which would overflow
+ 
+ error: this operation will panic at runtime
113    |
113    |
114 LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());

115    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
117 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:49:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1isize % 0; }).join().is_err());
+    |                                    ^^^^^^^^^^ attempt to calculate the remainder of `1_isize` with a divisor of zero
+ 
+ error: this operation will panic at runtime
119    |
119    |
120 LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());

121    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
123 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:51:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i8 % 0; }).join().is_err());
+    |                                    ^^^^^^^ attempt to calculate the remainder of `1_i8` with a divisor of zero
+ 
+ error: this operation will panic at runtime
125    |
125    |
126 LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());

127    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
129 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:53:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i16 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i16` with a divisor of zero
+ 
+ error: this operation will panic at runtime
131    |
131    |
132 LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());

133    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
135 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:55:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i32 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i32` with a divisor of zero
+ 
+ error: this operation will panic at runtime
137    |
137    |
138 LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());

139    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
141 error: this operation will panic at runtime
+   --> $DIR/issue-8460-const.rs:57:36
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i64 % 0; }).join().is_err());
+    |                                    ^^^^^^^^ attempt to calculate the remainder of `1_i64` with a divisor of zero
+ 
+ error: this operation will panic at runtime
143    |
143    |
144 LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());

145    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
- error: aborting due to 24 previous errors
- error: aborting due to 24 previous errors
+ error: this operation will panic at runtime
+    |
+    |
+ LL |     assert!(thread::spawn(move|| { 1i128 % 0; }).join().is_err());
+    |                                    ^^^^^^^^^ attempt to calculate the remainder of `1_i128` with a divisor of zero
+ error: aborting due to 48 previous errors
148 
149 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt/issue-8460-const.opt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args numbers-arithmetic/issue-8460-const.rs`

error in revision `opt`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/issue-8460-const.opt/auxiliary"
stdout: none
--- stderr -------------------------------
error: this operation will panic at runtime
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
   = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:13:36
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:13:36
   |
LL |     assert!(thread::spawn(move|| { isize::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute `isize::MIN / -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:15:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:15:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute `i8::MIN / -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:17:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:17:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i16::MIN / -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:19:36
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:19:36
   |
   |
LL |     assert!(thread::spawn(move|| { i32::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i32::MIN / -1_i32`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:21:36
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:21:36
   |
   |
LL |     assert!(thread::spawn(move|| { i64::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute `i64::MIN / -1_i64`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:23:36
   |
   |
LL |     assert!(thread::spawn(move|| { i128::MIN / -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^ attempt to compute `i128::MIN / -1_i128`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:23:36
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
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:25:36
   |
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
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:37:36
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:37:36
   |
   |
LL |     assert!(thread::spawn(move|| { isize::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^^^ attempt to compute the remainder of `isize::MIN % -1_isize`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:39:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:39:36
   |
   |
LL |     assert!(thread::spawn(move|| { i8::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^ attempt to compute the remainder of `i8::MIN % -1_i8`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:41:36
   |
   |
LL |     assert!(thread::spawn(move|| { i16::MIN % -1; }).join().is_err());
   |                                    ^^^^^^^^^^^^^ attempt to compute the remainder of `i16::MIN % -1_i16`, which would overflow
error: this operation will panic at runtime
  --> /checkout/src/test/ui/numbers-arithmetic/issue-8460-const.rs:41:36
   |
