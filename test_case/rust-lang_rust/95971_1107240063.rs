plain

1 error[E0080]: evaluation of constant value failed
2   --> $SRC_DIR/core/src/num/f32.rs:LL:COL
3    |
- LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN")
+ LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN");
6    |                     |
6    |                     |
7    |                     the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', $SRC_DIR/core/src/num/f32.rs:LL:COL
30 error[E0080]: evaluation of constant value failed
31   --> $SRC_DIR/core/src/num/f32.rs:LL:COL
32    |
32    |
- LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN")
+ LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN");
35    |                     |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
36    |                     the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', $SRC_DIR/core/src/num/f32.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv/const-float-bits-reject-conv.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-float-bits-reject-conv.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-float-bits-reject-conv.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-float-bits-reject-conv/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN");
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', /checkout/library/core/src/num/f32.rs:926:21
   |                     inside `core::f32::<impl f32>::to_bits::ct_f32_to_u32` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f32_to_u32, rt_f32_to_u32) }
   |                  -------------------------------------------------------------------- inside `core::f32::<impl f32>::to_bits` at /checkout/library/core/src/num/f32.rs:942:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32} as FnOnce<(f32,)>>::call_once - shim(fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2389:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f32,), fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32}, [closure@core::f32::<impl f32>::to_bits::{closure#0}], u32>` at /checkout/library/core/src/intrinsics.rs:2389:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:27:30
   |
   |
LL |     const MASKED_NAN1: u32 = f32::NAN.to_bits() ^ 0x002A_AAAA;
   |                              ------------------ inside `f32::MASKED_NAN1` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:27:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/f32.rs:926:21
  --> /checkout/library/core/src/num/f32.rs:926:21
   |
LL |                     panic!("const-eval error: cannot use f32::to_bits on a NaN");
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f32::to_bits on a NaN', /checkout/library/core/src/num/f32.rs:926:21
   |                     inside `core::f32::<impl f32>::to_bits::ct_f32_to_u32` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f32_to_u32, rt_f32_to_u32) }
   |                  -------------------------------------------------------------------- inside `core::f32::<impl f32>::to_bits` at /checkout/library/core/src/num/f32.rs:942:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32} as FnOnce<(f32,)>>::call_once - shim(fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2389:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f32,), fn(f32) -> u32 {core::f32::<impl f32>::to_bits::ct_f32_to_u32}, [closure@core::f32::<impl f32>::to_bits::{closure#0}], u32>` at /checkout/library/core/src/intrinsics.rs:2389:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:28:30
   |
   |
LL |     const MASKED_NAN2: u32 = f32::NAN.to_bits() ^ 0x0055_5555;
   |                              ------------------ inside `f32::MASKED_NAN2` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:28:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/f64.rs:919:21
  --> /checkout/library/core/src/num/f64.rs:919:21
   |
LL |                     panic!("const-eval error: cannot use f64::to_bits on a NaN")
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f64::to_bits on a NaN', /checkout/library/core/src/num/f64.rs:919:21
   |                     inside `core::f64::<impl f64>::to_bits::ct_f64_to_u64` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f64_to_u64, rt_f64_to_u64) }
   |                  -------------------------------------------------------------------- inside `core::f64::<impl f64>::to_bits` at /checkout/library/core/src/num/f64.rs:935:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64} as FnOnce<(f64,)>>::call_once - shim(fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2389:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f64,), fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64}, [closure@core::f64::<impl f64>::to_bits::{closure#0}], u64>` at /checkout/library/core/src/intrinsics.rs:2389:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:46:30
   |
   |
LL |     const MASKED_NAN1: u64 = f64::NAN.to_bits() ^ 0x000A_AAAA_AAAA_AAAA;
   |                              ------------------ inside `f64::MASKED_NAN1` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:46:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/f64.rs:919:21
  --> /checkout/library/core/src/num/f64.rs:919:21
   |
LL |                     panic!("const-eval error: cannot use f64::to_bits on a NaN")
   |                     |
   |                     |
   |                     the evaluated program panicked at 'const-eval error: cannot use f64::to_bits on a NaN', /checkout/library/core/src/num/f64.rs:919:21
   |                     inside `core::f64::<impl f64>::to_bits::ct_f64_to_u64` at /checkout/library/core/src/panic.rs:57:9
...
LL |         unsafe { intrinsics::const_eval_select((self,), ct_f64_to_u64, rt_f64_to_u64) }
   |                  -------------------------------------------------------------------- inside `core::f64::<impl f64>::to_bits` at /checkout/library/core/src/num/f64.rs:935:18
  ::: /checkout/library/core/src/ops/function.rs:248:5
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |     ------------------------------------------------------------------ inside `<fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64} as FnOnce<(f64,)>>::call_once - shim(fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64})` at /checkout/library/core/src/ops/function.rs:248:5
  ::: /checkout/library/core/src/intrinsics.rs:2389:5
   |
LL |     called_in_const.call_once(arg)
LL |     called_in_const.call_once(arg)
   |     ------------------------------ inside `const_eval_select::<(f64,), fn(f64) -> u64 {core::f64::<impl f64>::to_bits::ct_f64_to_u64}, [closure@core::f64::<impl f64>::to_bits::{closure#0}], u64>` at /checkout/library/core/src/intrinsics.rs:2389:5
  ::: /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:47:30
   |
   |
LL |     const MASKED_NAN2: u64 = f64::NAN.to_bits() ^ 0x0005_5555_5555_5555;
   |                              ------------------ inside `f64::MASKED_NAN2` at /checkout/src/test/ui/consts/const-float-bits-reject-conv.rs:47:30
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 4 previous errors

