plain
##[endgroup]
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 15047 tests
...............................FF.............ii.FF.....................................    88/15047
.......................................................................................i   176/15047
........................................................................................   352/15047
........................................................................................   440/15047
........................................................................................   528/15047
........................................................................................   616/15047
---
.......................................................................................

failures:

---- [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv64 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
2   --> $DIR/riscv-discoverability-guidance.rs:11:5


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv64/riscv-discoverability-guidance.riscv64.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args abi/riscv-discoverability-guidance.rs`

error in revision `riscv64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/riscv-discoverability-guidance.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "riscv64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv64/auxiliary" "--target=riscv64gc-unknown-none-elf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:11:5
LL |     abi_riscv_interrupt
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0703]: invalid ABI: found `riscv-interrupt`
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:17:8
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
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:23:8
   |
LL | extern "riscv-interrupt-u" fn isr_U() {}
   |        |
   |        invalid ABI
   |        invalid ABI
   |        help: did you mean: `"riscv-interrupt-m"`
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
   = note: user-mode interrupt handlers have been removed from LLVM pending standardization, see: https://reviews.llvm.org/D149314
error: requires `sized` lang_item

error: aborting due to 3 previous errors; 1 warning emitted


Build completed unsuccessfully in 0:12:47
For more information about this error, try `rustc --explain E0703`.
------------------------------------------


---- [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv32 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
2   --> $DIR/riscv-discoverability-guidance.rs:11:5


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv32/riscv-discoverability-guidance.riscv32.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args abi/riscv-discoverability-guidance.rs`

error in revision `riscv32`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/riscv-discoverability-guidance.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "riscv32" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv32" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/riscv-discoverability-guidance.riscv32/auxiliary" "--target=riscv32i-unknown-none-elf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
'+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:11:5
LL |     abi_riscv_interrupt
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0703]: invalid ABI: found `riscv-interrupt`
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:17:8
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
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:23:8
   |
LL | extern "riscv-interrupt-u" fn isr_U() {}
   |        |
   |        invalid ABI
   |        invalid ABI
   |        help: did you mean: `"riscv-interrupt-m"`
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
   = note: user-mode interrupt handlers have been removed from LLVM pending standardization, see: https://reviews.llvm.org/D149314
error: requires `sized` lang_item

error: aborting due to 3 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0703`.
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
------------------------------------------



failures:
    [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv64
    [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv32
    [ui] tests/ui/abi/unsupported.rs#riscv32
    [ui] tests/ui/feature-gates/feature-gate-abi-riscv-interrupt.rs

test result: FAILED. 14907 passed; 5 failed; 135 ignored; 0 measured; 0 filtered out; finished in 160.43s
