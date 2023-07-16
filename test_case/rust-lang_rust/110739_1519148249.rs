plain

---- [ui] tests/ui/consts/const-eval/const_panic_2021.rs stdout ----
diff of stderr:

4 LL | const A: () = std::panic!("blåhaj");
5    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'blåhaj', $DIR/const_panic_2021.rs:6:15
-    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `std::panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `std::panic` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
9 error[E0080]: evaluation of constant value failed
9 error[E0080]: evaluation of constant value failed
10   --> $DIR/const_panic_2021.rs:9:15

44 LL | const A_CORE: () = core::panic!("shark");
45    |                    ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'shark', $DIR/const_panic_2021.rs:21:20
-    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `core::panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `core::panic` (in Nightly builds, run with -Z macro-backtrace for more info)
48 
49 error[E0080]: evaluation of constant value failed
49 error[E0080]: evaluation of constant value failed
50   --> $DIR/const_panic_2021.rs:24:20


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/const_panic_2021.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_2021.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/const_panic_2021.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/auxiliary" "--edition=2021"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:6:15
   |
   |
LL | const A: () = std::panic!("blåhaj");
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'blåhaj', fake-test-src-base/consts/const-eval/const_panic_2021.rs:6:15
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `std::panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:9:15
---

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:12:15
   |
LL | const C: () = std::unreachable!();
   |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', fake-test-src-base/consts/const-eval/const_panic_2021.rs:12:15
   = note: this error originates in the macro `$crate::panic::unreachable_2021` which comes from the expansion of the macro `std::unreachable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:15:15
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:15:15
   |
LL | const D: () = std::unimplemented!();
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', fake-test-src-base/consts/const-eval/const_panic_2021.rs:15:15
   = note: this error originates in the macro `std::unimplemented` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:18:15
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:18:15
   |
LL | const E: () = std::panic!("{}", MSG);
   |               ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', fake-test-src-base/consts/const-eval/const_panic_2021.rs:18:15
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `std::panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:21:20
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:21:20
   |
LL | const A_CORE: () = core::panic!("shark");
   |                    ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'shark', fake-test-src-base/consts/const-eval/const_panic_2021.rs:21:20
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `core::panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:24:20
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:24:20
   |
LL | const B_CORE: () = core::panic!();
   |                    ^^^^^^^^^^^^^^ the evaluated program panicked at 'explicit panic', fake-test-src-base/consts/const-eval/const_panic_2021.rs:24:20
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `core::panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:27:20
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:27:20
   |
LL | const C_CORE: () = core::unreachable!();
   |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', fake-test-src-base/consts/const-eval/const_panic_2021.rs:27:20
   = note: this error originates in the macro `$crate::panic::unreachable_2021` which comes from the expansion of the macro `core::unreachable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:30:20
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:30:20
   |
LL | const D_CORE: () = core::unimplemented!();
   |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', fake-test-src-base/consts/const-eval/const_panic_2021.rs:30:20
   = note: this error originates in the macro `core::unimplemented` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:33:20
  --> fake-test-src-base/consts/const-eval/const_panic_2021.rs:33:20
   |
LL | const E_CORE: () = core::panic!("{}", MSG);
   |                    ^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', fake-test-src-base/consts/const-eval/const_panic_2021.rs:33:20
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `core::panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors

---
---- [ui] tests/ui/consts/const-float-bits-reject-conv.rs stdout ----
diff of stderr:

12    |
13 LL |     const MASKED_NAN1: u32 = f32::NAN.to_bits() ^ 0x002A_AAAA;
-    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
16 
17 error[E0080]: evaluation of constant value failed
17 error[E0080]: evaluation of constant value failed
18   --> $SRC_DIR/core/src/num/f32.rs:LL:COL

28    |
29 LL |     const MASKED_NAN2: u32 = f32::NAN.to_bits() ^ 0x0055_5555;
-    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
33 note: erroneous constant used
33 note: erroneous constant used
34   --> $DIR/const-float-bits-reject-conv.rs:35:34

68    |
69 LL |     const MASKED_NAN1: u64 = f64::NAN.to_bits() ^ 0x000A_AAAA_AAAA_AAAA;
-    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
72 
73 error[E0080]: evaluation of constant value failed
73 error[E0080]: evaluation of constant value failed
74   --> $SRC_DIR/core/src/num/f64.rs:LL:COL

84    |
85 LL |     const MASKED_NAN2: u64 = f64::NAN.to_bits() ^ 0x0005_5555_5555_5555;
-    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
88 
89 note: erroneous constant used
---
To only update this specific test, also pass `--test-args consts/const-float-bits-reject-conv.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-float-bits-reject-conv.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv/auxiliary" "-Zmir-opt-level=0"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
   |
   |
   = note: the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
   |
note: inside `core::f32::<impl f32>::to_bits::ct_f32_to_u32`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
note: inside `core::f32::<impl f32>::to_bits`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1045:18
note: inside `f32::MASKED_NAN1`
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:28:30
   |
LL |     const MASKED_NAN1: u32 = f32::NAN.to_bits() ^ 0x002A_AAAA;
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
   |
   = note: the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
   |
note: inside `core::f32::<impl f32>::to_bits::ct_f32_to_u32`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1025:21
note: inside `core::f32::<impl f32>::to_bits`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f32.rs:1045:18
note: inside `f32::MASKED_NAN2`
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:30:30
   |
LL |     const MASKED_NAN2: u32 = f32::NAN.to_bits() ^ 0x0055_5555;
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:35:34
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:35:34
   |
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:36:34
   |
   |
LL |     const_assert!(f32::from_bits(MASKED_NAN1).is_nan());

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:42:34
   |
   |
LL |     const_assert!(f32::from_bits(MASKED_NAN1).to_bits(), MASKED_NAN1);

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:43:34
   |
   |
LL |     const_assert!(f32::from_bits(MASKED_NAN2).to_bits(), MASKED_NAN2);

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
   |
   |
   = note: the evaluated program panicked at 'const-eval error: cannot use f64::to_bits on a NaN', /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
   |
note: inside `core::f64::<impl f64>::to_bits::ct_f64_to_u64`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
note: inside `core::f64::<impl f64>::to_bits`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1038:18
note: inside `f64::MASKED_NAN1`
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:50:30
   |
LL |     const MASKED_NAN1: u64 = f64::NAN.to_bits() ^ 0x000A_AAAA_AAAA_AAAA;
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
   |
   = note: the evaluated program panicked at 'const-eval error: cannot use f64::to_bits on a NaN', /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
   |
note: inside `core::f64::<impl f64>::to_bits::ct_f64_to_u64`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1018:21
note: inside `core::f64::<impl f64>::to_bits`
  --> /rustc/FAKE_PREFIX/library/core/src/num/f64.rs:1038:18
note: inside `f64::MASKED_NAN2`
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:52:30
   |
LL |     const MASKED_NAN2: u64 = f64::NAN.to_bits() ^ 0x0005_5555_5555_5555;
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:57:34
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:57:34
   |
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:58:34
   |
   |
LL |     const_assert!(f64::from_bits(MASKED_NAN1).is_nan());

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:61:34
   |
   |
LL |     const_assert!(f64::from_bits(MASKED_NAN1).to_bits(), MASKED_NAN1);

note: erroneous constant used
  --> fake-test-src-base/consts/const-float-bits-reject-conv.rs:62:34
   |
   |
LL |     const_assert!(f64::from_bits(MASKED_NAN2).to_bits(), MASKED_NAN2);

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0080`.
