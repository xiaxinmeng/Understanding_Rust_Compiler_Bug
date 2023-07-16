plain
failures:

---- [ui] tests/ui/consts/const-eval/promoted_errors.rs#opt stdout ----

error in revision `opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/auxiliary" "-O"
stdout: none
--- stderr -------------------------------
warning: this arithmetic operation will overflow
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:15:5
LL |     0 - 1
   |     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow
   |
note: the lint level is defined here
note: the lint level is defined here
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:11:9
   |
LL | #![warn(arithmetic_overflow, unconditional_panic)]

warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:19:5
   |
   |
LL |     1 / 0
   |     ^^^^^ attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:11:30
   |
   |
LL | #![warn(arithmetic_overflow, unconditional_panic)]

warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:23:5
   |
   |
LL |     1 / (1 - 1)
   |     ^^^^^^^^^^^ attempt to divide `1_i32` by zero
warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:27:5
   |
LL |     1 / (false as i32)
LL |     1 / (false as i32)
   |     ^^^^^^^^^^^^^^^^^^ attempt to divide `1_i32` by zero
warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:31:5
   |
LL |     [1, 2, 3][4]
LL |     [1, 2, 3][4]
   |     ^^^^^^^^^^^^ index out of bounds: the length is 3 but the index is 4

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:19:5
   |
LL |     1 / 0
   |     ^^^^^ attempt to divide `1_i32` by zero
note: inside `div_by_zero1`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:19:5
   |
LL |     1 / 0
LL |     1 / 0
   |     ^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:42:33
   |
LL |         let _x: &'static i32 = &div_by_zero1();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:23:5
   |
   |
LL |     1 / (1 - 1)
   |     ^^^^^^^^^^^ attempt to divide `1_i32` by zero
note: inside `div_by_zero2`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:23:5
   |
LL |     1 / (1 - 1)
LL |     1 / (1 - 1)
   |     ^^^^^^^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:43:33
   |
LL |         let _x: &'static i32 = &div_by_zero2();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:27:5
   |
   |
LL |     1 / (false as i32)
   |     ^^^^^^^^^^^^^^^^^^ attempt to divide `1_i32` by zero
note: inside `div_by_zero3`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:27:5
   |
LL |     1 / (false as i32)
LL |     1 / (false as i32)
   |     ^^^^^^^^^^^^^^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:44:33
   |
LL |         let _x: &'static i32 = &div_by_zero3();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:31:5
   |
---
   |     ^^^^^^^^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:45:33
   |
LL |         let _x: &'static i32 = &oob();

error: aborting due to 4 previous errors; 5 warnings emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] tests/ui/consts/const-eval/promoted_errors.rs#opt_with_overflow_checks stdout ----

error in revision `opt_with_overflow_checks`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/auxiliary" "-C" "overflow-checks=on" "-O"
stdout: none
--- stderr -------------------------------
warning: this arithmetic operation will overflow
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:15:5
LL |     0 - 1
   |     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow
   |
note: the lint level is defined here
note: the lint level is defined here
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:11:9
   |
LL | #![warn(arithmetic_overflow, unconditional_panic)]

warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:19:5
   |
   |
LL |     1 / 0
   |     ^^^^^ attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:11:30
   |
   |
LL | #![warn(arithmetic_overflow, unconditional_panic)]

warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:23:5
   |
   |
LL |     1 / (1 - 1)
   |     ^^^^^^^^^^^ attempt to divide `1_i32` by zero
warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:27:5
   |
LL |     1 / (false as i32)
LL |     1 / (false as i32)
   |     ^^^^^^^^^^^^^^^^^^ attempt to divide `1_i32` by zero
warning: this operation will panic at runtime
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:31:5
   |
LL |     [1, 2, 3][4]
---
   |     ^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:41:33
   |
LL |         let _x: &'static u32 = &overflow();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:19:5
   |
   |
LL |     1 / 0
   |     ^^^^^ attempt to divide `1_i32` by zero
note: inside `div_by_zero1`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:19:5
   |
LL |     1 / 0
LL |     1 / 0
   |     ^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:42:33
   |
LL |         let _x: &'static i32 = &div_by_zero1();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:23:5
   |
   |
LL |     1 / (1 - 1)
   |     ^^^^^^^^^^^ attempt to divide `1_i32` by zero
note: inside `div_by_zero2`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:23:5
   |
LL |     1 / (1 - 1)
LL |     1 / (1 - 1)
   |     ^^^^^^^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:43:33
   |
LL |         let _x: &'static i32 = &div_by_zero2();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:27:5
   |
   |
LL |     1 / (false as i32)
   |     ^^^^^^^^^^^^^^^^^^ attempt to divide `1_i32` by zero
note: inside `div_by_zero3`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:27:5
   |
LL |     1 / (false as i32)
LL |     1 / (false as i32)
   |     ^^^^^^^^^^^^^^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:44:33
   |
LL |         let _x: &'static i32 = &div_by_zero3();

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:31:5
   |
---
   |     ^^^^^^^^^^^^
note: inside `Y`
  --> fake-test-src-base/consts/const-eval/promoted_errors.rs:45:33
   |
LL |         let _x: &'static i32 = &oob();

error: aborting due to 5 previous errors; 5 warnings emitted

For more information about this error, try `rustc --explain E0080`.
