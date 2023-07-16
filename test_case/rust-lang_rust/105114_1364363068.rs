plain
........................................................................................ 2376/14044
........................................................................................ 2464/14044
........................................................................................ 2552/14044
........................................................................................ 2640/14044
...................F.................................................................... 2728/14044
.F....................F................................................................. 2816/14044
........................................................................................ 2992/14044
..F..................................................................................... 3080/14044
....................................................FF.................................. 3168/14044
..............i.................................F....................i.................. 3256/14044
---
---- [ui] src/test/ui/associated-consts/defaults-not-assumed-fail.rs stdout ----
diff of stderr:

34    |
35    = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
+ note: erroneous constant used
+   --> $DIR/defaults-not-assumed-fail.rs:33:5
+    |
+    |
+ LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
+    |
+    |
+    = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
37 error: aborting due to previous error
38 
39 For more information about this error, try `rustc --explain E0080`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<() as Tr>::B` failed
   |
   |
LL |     const B: u8 = Self::A + 1;
   |                   ^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
note: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:33:16
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above

note: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:33:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |
   |
   = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
note: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:33:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |
   |
   = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
note: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:33:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |
   |
   = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
note: erroneous constant used
  --> /checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs:33:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |
   |
   = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-err-late.rs stdout ----
diff of stderr:

46 LL |     black_box((S::<i32>::FOO, S::<u32>::FOO));
48 
+ note: erroneous constant used
+   --> $DIR/const-err-late.rs:19:16
+    |
+    |
+ LL |     black_box((S::<i32>::FOO, S::<u32>::FOO));
+ 
+ note: erroneous constant used
+   --> $DIR/const-err-late.rs:19:31
+    |
+    |
+ LL |     black_box((S::<i32>::FOO, S::<u32>::FOO));
+ 
49 error: aborting due to 2 previous errors
50 
51 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/const-err-late.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err-late.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-late" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-late/auxiliary" "-C" "overflow-checks=on"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `S::<i32>::FOO` failed
   |
   |
LL |     const FOO: u8 = [5u8][1];
   |                     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:16
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:16
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant


error[E0080]: evaluation of `S::<u32>::FOO` failed
   |
   |
LL |     const FOO: u8 = [5u8][1];
   |                     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:31
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:31
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:16
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:31
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:16
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-err-late.rs:19:31
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
---
36 
+ note: erroneous constant used
+   --> $DIR/issue-44578.rs:25:20
+    |
+ LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
+    |
+    = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
37 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-44578.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-44578.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<Bar<u16, u8> as Foo>::AMT` failed
   |
   |
LL |     const AMT: usize = [A::AMT][(A::AMT > B::AMT) as usize]; //~ERROR evaluation of `<Bar<u16, u8> as Foo>::AMT` failed
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:25:20
   |
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:25:20
   |
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:25:20
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:25:20
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/issue-44578.rs:25:20
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
To only update this specific test, also pass `--test-args consts/const-eval/panic-assoc-never-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:9:21
   |
LL |     const VOID: ! = panic!();
   |                     ^^^^^^^^ the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:9:21
   |                     ^^^^^^^^ the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:9:21
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:14:13
   |
LL |     let _ = PrintName::VOID; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:14:13
   |
   |
LL |     let _ = PrintName::VOID; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:14:13
   |
   |
LL |     let _ = PrintName::VOID; //~ constant

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/consts/invalid-union.rs stdout ----
diff of 64bit.stderr:

27 LL |     let _: &'static _ = &C;
29 
+ note: erroneous constant used
+   --> $DIR/invalid-union.rs:43:25
+    |
+    |
+ LL |     let _: &'static _ = &C;
+ 
30 error: aborting due to previous error
31 
32 For more information about this error, try `rustc --explain E0080`.
32 For more information about this error, try `rustc --explain E0080`.


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.64bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ constructing invalid value at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
   |
LL |     let _: &'static _ = &C;

note: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
   |
   |
LL |     let _: &'static _ = &C;

note: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
   |
   |
LL |     let _: &'static _ = &C;

note: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
   |
   |
LL |     let _: &'static _ = &C;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/consts/miri_unleashed/assoc_const_2.rs stdout ----
diff of stderr:

22 LL |     let y = <String as Bar<String>>::F;
24 
+ note: erroneous constant used
+   --> $DIR/assoc_const_2.rs:27:13
+    |
+    |
+ LL |     let y = <String as Bar<String>>::F;
+ 
25 error: aborting due to previous error
26 
27 For more information about this error, try `rustc --explain E0080`.
27 For more information about this error, try `rustc --explain E0080`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/assoc_const_2.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<std::string::String as Bar<std::string::String>>::F` failed
   |
   |
LL |     const F: u32 = 100 / U::X; //~ ERROR evaluation of `<std::string::String as Bar<std::string::String>>::F` failed
   |                    ^^^^^^^^^^ attempt to divide `100_u32` by zero
note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:27:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:27:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:27:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const_2.rs:27:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ constant

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/consts/miri_unleashed/assoc_const.rs stdout ----
diff of stderr:

31 LL |     let y = <String as Bar<Vec<u32>, String>>::F;
33 
+ note: erroneous constant used
+   --> $DIR/assoc_const.rs:29:13
+    |
+    |
+ LL |     let y = <String as Bar<Vec<u32>, String>>::F;
+ 
34 warning: skipping const checks
35    |
36 help: skipping check that does not even have a feature gate
---
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary" "-Zunleash-the-miri-inside-of-you"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<std::string::String as Bar<std::vec::Vec<u32>, std::string::String>>::F` failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:490:1
   |
   = note: calling non-const function `<Vec<u32> as Drop>::drop`
   |
note: inside `std::ptr::drop_in_place::<Vec<u32>> - shim(Some(Vec<u32>))`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:490:1
note: inside `std::ptr::drop_in_place::<(Vec<u32>, u32)> - shim(Some((Vec<u32>, u32)))`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:490:1
note: inside `<String as Bar<Vec<u32>, String>>::F`
   |
   |
LL |     const F: u32 = (U::X, 42).1;

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:29:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:29:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:29:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ constant

note: erroneous constant used
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:29:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ constant

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:12:20
   |
LL |     const F: u32 = (U::X, 42).1;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/consts/uninhabited-const-issue-61744.rs stdout ----
diff of stderr:

663 LL |     dbg!(i32::CONSTANT);
665 
+ note: erroneous constant used
+   --> $DIR/uninhabited-const-issue-61744.rs:18:10
+    |
+    |
+ LL |     dbg!(i32::CONSTANT);
+ 
666 error: aborting due to previous error
667 
668 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<i32 as Const>::CONSTANT` failed
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
   |     ^^^^^^^^^^^^^^^^^^ reached the configured maximum number of stack frames
   |
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
   |
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> /checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs:8:5
   |
---
35 
+ note: erroneous constant used
+   --> $DIR/issue-55878.rs:7:26
+    |
+ LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
+    |
+    = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
36 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args limits/issue-55878.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/limits/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:309:5
note: inside `std::mem::size_of::<[u8; 18446744073709551615]>`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:309:5
note: inside `main`
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());

note: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
