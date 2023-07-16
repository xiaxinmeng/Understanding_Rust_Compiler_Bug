plain
##[endgroup]
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 15053 tests
..............................F.F............Fii...F....................................    88/15053
.......................................................................................i   176/15053
........................................................................................   352/15053
........................................................................................   440/15053
........................................................................................   528/15053
........................................................................................   616/15053
---
.....

failures:

---- [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv32 stdout ----


+ '-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
-   --> $DIR/riscv-discoverability-guidance.rs:11:5
+   --> $DIR/riscv-discoverability-guidance.rs:12:5
4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^

8    = note: `#[warn(incomplete_features)]` on by default
8    = note: `#[warn(incomplete_features)]` on by default
9 
10 error[E0703]: invalid ABI: found `riscv-interrupt`
-   --> $DIR/riscv-discoverability-guidance.rs:19:8
+   --> $DIR/riscv-discoverability-guidance.rs:20:8
12    |
13 LL | extern "riscv-interrupt" fn isr() {}


20    = note: please use one of riscv-interrupt-m or riscv-interrupt-s for machine- or supervisor-level interrupts, respectively
21 
22 error[E0703]: invalid ABI: found `riscv-interrupt-u`
-   --> $DIR/riscv-discoverability-guidance.rs:25:8
+   --> $DIR/riscv-discoverability-guidance.rs:26:8
24    |
25 LL | extern "riscv-interrupt-u" fn isr_U() {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv32/riscv-discoverability-guidance.riscv32.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args abi/riscv-discoverability-guidance.rs`

error in revision `riscv32`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/riscv-discoverability-guidance.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "riscv32" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv32" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv32/auxiliary" "--target=riscv32i-unknown-none-elf" "-C" "target-feature=-unaligned-scalar-mem" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:12:5
LL |     abi_riscv_interrupt
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0703]: invalid ABI: found `riscv-interrupt`
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:20:8
   |
LL | extern "riscv-interrupt" fn isr() {}
   |        |
   |        invalid ABI
   |        invalid ABI
   |        help: did you mean: `"riscv-interrupt-m"`
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
   = note: please use one of riscv-interrupt-m or riscv-interrupt-s for machine- or supervisor-level interrupts, respectively

error[E0703]: invalid ABI: found `riscv-interrupt-u`
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:26:8
   |
LL | extern "riscv-interrupt-u" fn isr_U() {}
   |        |
   |        invalid ABI
   |        invalid ABI
   |        help: did you mean: `"riscv-interrupt-m"`
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
   = note: user-mode interrupt handlers have been removed from LLVM pending standardization, see: https://reviews.llvm.org/D149314
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0703`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv64 stdout ----


+ '-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
-   --> $DIR/riscv-discoverability-guidance.rs:11:5
+   --> $DIR/riscv-discoverability-guidance.rs:12:5
3    |
4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^


8    = note: `#[warn(incomplete_features)]` on by default
9 
10 error[E0703]: invalid ABI: found `riscv-interrupt`
-   --> $DIR/riscv-discoverability-guidance.rs:19:8
+   --> $DIR/riscv-discoverability-guidance.rs:20:8
12    |
13 LL | extern "riscv-interrupt" fn isr() {}


20    = note: please use one of riscv-interrupt-m or riscv-interrupt-s for machine- or supervisor-level interrupts, respectively
21 
22 error[E0703]: invalid ABI: found `riscv-interrupt-u`
-   --> $DIR/riscv-discoverability-guidance.rs:25:8
+   --> $DIR/riscv-discoverability-guidance.rs:26:8
24    |
25 LL | extern "riscv-interrupt-u" fn isr_U() {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv64/riscv-discoverability-guidance.riscv64.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args abi/riscv-discoverability-guidance.rs`

error in revision `riscv64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/riscv-discoverability-guidance.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "riscv64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv64/auxiliary" "--target=riscv64gc-unknown-none-elf" "-C" "target-feature=-unaligned-scalar-mem" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'-unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:12:5
LL |     abi_riscv_interrupt
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0703]: invalid ABI: found `riscv-interrupt`
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:20:8
   |
LL | extern "riscv-interrupt" fn isr() {}
   |        |
   |        invalid ABI
   |        invalid ABI
   |        help: did you mean: `"riscv-interrupt-m"`
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
   = note: please use one of riscv-interrupt-m or riscv-interrupt-s for machine- or supervisor-level interrupts, respectively

error[E0703]: invalid ABI: found `riscv-interrupt-u`
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:26:8
   |
LL | extern "riscv-interrupt-u" fn isr_U() {}
   |        |
   |        invalid ABI
   |        invalid ABI
   |        help: did you mean: `"riscv-interrupt-m"`
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
   = note: user-mode interrupt handlers have been removed from LLVM pending standardization, see: https://reviews.llvm.org/D149314
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0703`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/abi/unsupported.rs#riscv32 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.riscv32/unsupported.riscv32.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `riscv32`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/unsupported.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "riscv32" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.riscv32" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.riscv32/auxiliary" "--target=riscv32i-unknown-none-elf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/abi/unsupported.rs:26:5
LL |     abi_riscv_interrupt
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:32:1
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:34:1
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}


error[E0570]: `"wasm"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:36:1
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"aapcs"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:38:1
   |
LL | extern "aapcs" fn aapcs() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:44:1
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:46:1
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:53:1
   |
LL | extern "x86-interrupt" fn x86() {}


error[E0570]: `"thiscall"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:58:1
   |
LL | extern "thiscall" fn thiscall() {}

warning: use of calling convention not supported on this target
  --> fake-test-src-base/abi/unsupported.rs:64:1
   |
   |
LL | extern "stdcall" fn stdcall() {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
   = note: `#[warn(unsupported_calling_conventions)]` on by default
   = note: `#[warn(unsupported_calling_conventions)]` on by default

error: aborting due to 8 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0570`.
------------------------------------------


---- [ui] tests/ui/abi/unsupported.rs#riscv64 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.riscv64/unsupported.riscv64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `riscv64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/unsupported.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "riscv64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.riscv64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.riscv64/auxiliary" "--target=riscv64gc-unknown-none-elf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/abi/unsupported.rs:26:5
LL |     abi_riscv_interrupt
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:32:1
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:34:1
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}


error[E0570]: `"wasm"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:36:1
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"aapcs"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:38:1
   |
LL | extern "aapcs" fn aapcs() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:44:1
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:46:1
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:53:1
   |
LL | extern "x86-interrupt" fn x86() {}


error[E0570]: `"thiscall"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:58:1
   |
LL | extern "thiscall" fn thiscall() {}

warning: use of calling convention not supported on this target
  --> fake-test-src-base/abi/unsupported.rs:64:1
   |
   |
LL | extern "stdcall" fn stdcall() {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
   = note: `#[warn(unsupported_calling_conventions)]` on by default
   = note: `#[warn(unsupported_calling_conventions)]` on by default

error: aborting due to 8 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0570`.
------------------------------------------


---- [ui] tests/ui/feature-gates/feature-gate-abi-riscv-interrupt.rs stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 error[E0658]: riscv-interrupt ABIs are experimental and subject to change
2   --> $DIR/feature-gate-abi-riscv-interrupt.rs:11:8


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-riscv-interrupt/feature-gate-abi-riscv-interrupt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-riscv-interrupt.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-abi-riscv-interrupt.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-riscv-interrupt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-riscv-interrupt/auxiliary" "--target=riscv32imc-unknown-none-elf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
error[E0658]: riscv-interrupt ABIs are experimental and subject to change
  --> fake-test-src-base/feature-gates/feature-gate-abi-riscv-interrupt.rs:11:8
   |
LL | extern "riscv-interrupt-m" fn f() {}
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable

error[E0658]: riscv-interrupt ABIs are experimental and subject to change
  --> fake-test-src-base/feature-gates/feature-gate-abi-riscv-interrupt.rs:13:8
   |
LL | extern "riscv-interrupt-s" fn f_s() {}
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable

error[E0658]: riscv-interrupt ABIs are experimental and subject to change
  --> fake-test-src-base/feature-gates/feature-gate-abi-riscv-interrupt.rs:17:12
   |
LL |     extern "riscv-interrupt-m" fn m();
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable

error[E0658]: riscv-interrupt ABIs are experimental and subject to change
  --> fake-test-src-base/feature-gates/feature-gate-abi-riscv-interrupt.rs:23:12
   |
LL |     extern "riscv-interrupt-m" fn m() {}
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable

error[E0658]: riscv-interrupt ABIs are experimental and subject to change
  --> fake-test-src-base/feature-gates/feature-gate-abi-riscv-interrupt.rs:28:12
   |
LL |     extern "riscv-interrupt-m" fn im() {}
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable

error[E0658]: riscv-interrupt ABIs are experimental and subject to change
  --> fake-test-src-base/feature-gates/feature-gate-abi-riscv-interrupt.rs:32:18
   |
LL | type TA = extern "riscv-interrupt-m" fn();
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
---

27    |
28    = note: doesn't satisfy `_: Iterator`
29    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/mismatched_types/issue-36053-2/issue-36053-2.long-type-1255529590694674092.txt'
31    = note: the following trait bounds were not satisfied:
32            `<[closure@$DIR/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
33            which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:48]>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:32
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                |
   |                                expected due to this
   |
   |
   = note: expected closure signature `for<'a> fn(&'a &str) -> _`
              found closure signature `for<'a> fn(&'a str) -> _`
note: required by a bound in `filter`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:922:5
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &&str| true).count();


error[E0599]: the method `count` exists for struct `Filter<Fuse<Once<&str>>, [closure@issue-36053-2.rs:7:39]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:55
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                       |
   |                                       |
   |                                       doesn't satisfy `<_ as FnOnce<(&&str,)>>::Output = bool`
   |                                       doesn't satisfy `_: FnMut<(&&str,)>`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/filter.rs:18:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]: FnMut<(&&str,)>`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           which is required by `&mut Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
---

42    |
43    = note: doesn't satisfy `_: Iterator`
44    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/issue-31173/issue-31173.long-type-16864929411157086952.txt'
46    = note: the following trait bounds were not satisfied:
47            `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:7:21: 7:25]> as Iterator>::Item = &_`
48            which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:7:21: 7:25]>>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173/issue-31173.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-31173.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-31173.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `TakeWhile<&mut IntoIter<u8>, [closure@issue-31173.rs:7:21]>` to be an iterator that yields `&_`, but it yields `u8`
  --> fake-test-src-base/typeck/issue-31173.rs:11:10
   |
LL |         .cloned() //~ ERROR to be an iterator that yields `&_`, but it yields `u8`
   |          ^^^^^^ expected `&_`, found `u8`
   = note: expected reference `&_`
                   found type `u8`
note: the method call chain might not have had the expected associated types
  --> fake-test-src-base/typeck/issue-31173.rs:3:20
  --> fake-test-src-base/typeck/issue-31173.rs:3:20
   |
LL |   pub fn get_tok(it: &mut IntoIter<u8>) {
   |                      ^^^^^^^^^^^^^^^^^ `Iterator::Item` is `u8` here
...
LL |           .take_while(|&x| {
   |  __________-
LL | |             found_e = true;
LL | |             false
LL | |         })
   | |__________- `Iterator::Item` remains `u8` here
note: required by a bound in `cloned`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:3358:5

error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut IntoIter<u8>, [closure@issue-31173.rs:7:21]>>`, but its trait bounds were not satisfied
  --> fake-test-src-base/typeck/issue-31173.rs:12:10
   |
LL |       let temp: Vec<u8> = it
   |  _________________________-
LL | |         .take_while(|&x| {
LL | |             found_e = true;
LL | |             false
LL | |         })
LL | |         .cloned() //~ ERROR to be an iterator that yields `&_`, but it yields `u8`
LL | |         .collect(); //~ ERROR the method
   | |         -^^^^^^^ method cannot be called due to unsatisfied trait bounds
   | 
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/take_while.rs:15:1
   |
   |
   = note: doesn't satisfy `<_ as Iterator>::Item = &_`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/cloned.rs:17:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]> as Iterator>::Item = &_`
           which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
           `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
           which is required by `&mut Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.
------------------------------------------



failures:
    [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv32
    [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv64
    [ui] tests/ui/abi/unsupported.rs#riscv64
    [ui] tests/ui/feature-gates/feature-gate-abi-riscv-interrupt.rs
    [ui] tests/ui/mismatched_types/issue-36053-2.rs
    [ui] tests/ui/typeck/issue-31173.rs
