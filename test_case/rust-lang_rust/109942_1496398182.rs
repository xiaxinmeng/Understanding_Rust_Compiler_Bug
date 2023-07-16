plain
28 
- note: erroneous constant used
-   --> $DIR/defaults-not-assumed-fail.rs:33:5
-    |
- LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
-    |
-    = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
37 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args associated-consts/defaults-not-assumed-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-consts/defaults-not-assumed-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<() as Tr>::B` failed
  --> fake-test-src-base/associated-consts/defaults-not-assumed-fail.rs:8:19
   |
LL |     const B: u8 = Self::A + 1;
   |                   ^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
note: erroneous constant used
  --> fake-test-src-base/associated-consts/defaults-not-assumed-fail.rs:33:16
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above

note: erroneous constant used
  --> fake-test-src-base/associated-consts/defaults-not-assumed-fail.rs:33:5
   |
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |
   = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> fake-test-src-base/associated-consts/defaults-not-assumed-fail.rs:33:5
   |
LL |     assert_eq!(<() as Tr>::B, 0);    // causes the error above
   |
   = note: this note originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

---- [ui] tests/ui/consts/const-err-late.rs stdout ----
diff of stderr:

28 LL |     black_box((S::<i32>::FOO, S::<u32>::FOO));
30 
- note: erroneous constant used
-   --> $DIR/const-err-late.rs:19:16
-    |
-    |
- LL |     black_box((S::<i32>::FOO, S::<u32>::FOO));
- 
37 error: aborting due to 2 previous errors
38 
39 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/const-err-late.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-err-late.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-late" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-late/auxiliary" "-C" "overflow-checks=on"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `S::<i32>::FOO` failed
  --> fake-test-src-base/consts/const-err-late.rs:13:21
   |
LL |     const FOO: u8 = [5u8][1];
   |                     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: erroneous constant used
  --> fake-test-src-base/consts/const-err-late.rs:19:16
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant


error[E0080]: evaluation of `S::<u32>::FOO` failed
  --> fake-test-src-base/consts/const-err-late.rs:13:21
   |
LL |     const FOO: u8 = [5u8][1];
   |                     ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: erroneous constant used
  --> fake-test-src-base/consts/const-err-late.rs:19:31
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

note: erroneous constant used
  --> fake-test-src-base/consts/const-err-late.rs:19:16
   |
   |
LL |     black_box((S::<i32>::FOO, S::<u32>::FOO)); //~ constant

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
---
28 
- note: erroneous constant used
-   --> $DIR/issue-44578.rs:25:20
-    |
- LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
-    |
-    = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
37 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-44578.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/issue-44578.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-44578/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<Bar<u16, u8> as Foo>::AMT` failed
  --> fake-test-src-base/consts/const-eval/issue-44578.rs:13:24
   |
LL |     const AMT: usize = [A::AMT][(A::AMT > B::AMT) as usize]; //~ERROR evaluation of `<Bar<u16, u8> as Foo>::AMT` failed
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/issue-44578.rs:25:20
   |
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/issue-44578.rs:25:20
   |
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/issue-44578.rs:25:20
   |
LL |     println!("{}", <Bar<u16, u8> as Foo>::AMT);
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

---- [ui] tests/ui/consts/miri_unleashed/assoc_const_2.rs stdout ----
diff of stderr:

16 LL |     let y = <String as Bar<String>>::F;
18 
- note: erroneous constant used
-   --> $DIR/assoc_const_2.rs:27:13
-    |
-    |
- LL |     let y = <String as Bar<String>>::F;
- 
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/miri_unleashed/assoc_const_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const_2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<std::string::String as Bar<std::string::String>>::F` failed
  --> fake-test-src-base/consts/miri_unleashed/assoc_const_2.rs:10:20
   |
LL |     const F: u32 = 100 / U::X; //~ ERROR evaluation of `<std::string::String as Bar<std::string::String>>::F` failed
   |                    ^^^^^^^^^^ attempt to divide `100_u32` by zero
note: erroneous constant used
  --> fake-test-src-base/consts/miri_unleashed/assoc_const_2.rs:27:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ constant

note: erroneous constant used
  --> fake-test-src-base/consts/miri_unleashed/assoc_const_2.rs:27:13
   |
   |
LL |     let y = <String as Bar<String>>::F; //~ constant

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/consts/miri_unleashed/assoc_const.rs stdout ----
diff of stderr:

25 LL |     let y = <String as Bar<Vec<u32>, String>>::F;
27 
- note: erroneous constant used
-   --> $DIR/assoc_const.rs:29:13
-    |
-    |
- LL |     let y = <String as Bar<Vec<u32>, String>>::F;
- 
34 warning: skipping const checks
35    |
36 help: skipping check that does not even have a feature gate
---
To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary" "-Zunleash-the-miri-inside-of-you"
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
  --> fake-test-src-base/consts/miri_unleashed/assoc_const.rs:12:31
   |
LL |     const F: u32 = (U::X, 42).1;

note: erroneous constant used
  --> fake-test-src-base/consts/miri_unleashed/assoc_const.rs:29:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ constant

note: erroneous constant used
  --> fake-test-src-base/consts/miri_unleashed/assoc_const.rs:29:13
   |
   |
LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ constant

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
help: skipping check that does not even have a feature gate
  --> fake-test-src-base/consts/miri_unleashed/assoc_const.rs:12:20
   |
LL |     const F: u32 = (U::X, 42).1;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/consts/uninhabited-const-issue-61744.rs stdout ----
diff of stderr:

657 LL |     dbg!(i32::CONSTANT);
659 
- note: erroneous constant used
-   --> $DIR/uninhabited-const-issue-61744.rs:18:10
-    |
-    |
- LL |     dbg!(i32::CONSTANT);
- 
666 error: aborting due to previous error
667 
668 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/uninhabited-const-issue-61744.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<i32 as Const>::CONSTANT` failed
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
   |     ^^^^^^^^^^^^^^^^^^ reached the configured maximum number of stack frames
   |
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
LL |     fake_type()
LL |     fake_type()
   |     ^^^^^^^^^^^
note: inside `fake_type::<!>`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:4:5
   |
LL |     hint_unreachable() //~ ERROR evaluation of `<i32 as Const>::CONSTANT` failed
note: inside `hint_unreachable`
  --> fake-test-src-base/consts/uninhabited-const-issue-61744.rs:8:5
   |
---
27 
- note: erroneous constant used
-   --> $DIR/issue-55878.rs:7:26
-    |
- LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
-    |
-    = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
36 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args limits/issue-55878.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/limits/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: values of the type `[u8; usize::MAX]` are too big for the current architecture
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:309:5
   |
note: inside `std::mem::size_of::<[u8; usize::MAX]>`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:309:5
  --> fake-test-src-base/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());

note: erroneous constant used
  --> fake-test-src-base/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
note: erroneous constant used
  --> fake-test-src-base/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |
   = note: this note originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
