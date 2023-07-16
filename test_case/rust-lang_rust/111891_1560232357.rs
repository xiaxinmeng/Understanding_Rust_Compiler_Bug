plain
##[endgroup]
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 15047 tests
..............................F.F.........F.F.iiFFFF....................................    88/15047
.......................................................................................i   176/15047
........................................................................................   352/15047
........................................................................................   440/15047
........................................................................................   528/15047
........................................................................................   616/15047
---
.......................................................................................

failures:

---- [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv32 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
2   --> $DIR/riscv-discoverability-guidance.rs:11:5

4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0703]: invalid ABI: found `riscv-interrupt`

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
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:16:8
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
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:22:8
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


---- [ui] tests/ui/abi/riscv-discoverability-guidance.rs#riscv64 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
2   --> $DIR/riscv-discoverability-guidance.rs:11:5

4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0703]: invalid ABI: found `riscv-interrupt`

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
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:16:8
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
  --> fake-test-src-base/abi/riscv-discoverability-guidance.rs:22:8
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

---
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.arm/unsupported.arm.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `arm`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/unsupported.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "arm" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.arm" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.arm/auxiliary" "--target=armv7-unknown-linux-gnueabihf" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
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


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:44:1
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:46:1
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"riscv-interrupt-m"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:48:1
   |
LL | extern "riscv-interrupt-m" fn riscv() {}


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
---
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/unsupported.aarch64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `aarch64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/unsupported.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/auxiliary" "--target=aarch64-unknown-linux-gnu" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
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


error[E0570]: `"riscv-interrupt-m"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:48:1
   |
LL | extern "riscv-interrupt-m" fn riscv() {}


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

error: aborting due to 9 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0570`.
------------------------------------------


---- [ui] tests/ui/abi/unsupported.rs#i686 stdout ----

4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686/unsupported.i686.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`

error in revision `i686`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/unsupported.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "i686" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686/auxiliary" "--target=i686-unknown-linux-gnu" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
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


error[E0570]: `"riscv-interrupt-m"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:48:1
   |
LL | extern "riscv-interrupt-m" fn riscv() {}

error: aborting due to 7 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0570`.
For more information about this error, try `rustc --explain E0570`.
------------------------------------------


---- [ui] tests/ui/abi/unsupported.rs#riscv64 stdout ----


+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
+ '+unaligned-scalar-mem' is not a recognized feature for this target (ignoring feature)
1 warning: the feature `abi_riscv_interrupt` is incomplete and may not be safe to use and/or cause compiler crashes
3    |

4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target

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

4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target

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


---- [ui] tests/ui/abi/unsupported.rs#x64 stdout ----

4 LL |     abi_riscv_interrupt
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = note: `#[warn(incomplete_features)]` on by default
8 
9 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64/unsupported.x64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `x64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/abi/unsupported.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--cfg" "x64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64/auxiliary" "--target=x86_64-unknown-linux-gnu" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
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


error[E0570]: `"riscv-interrupt-m"` is not a supported ABI for the current target
  --> fake-test-src-base/abi/unsupported.rs:48:1
   |
LL | extern "riscv-interrupt-m" fn riscv() {}


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


4 LL | extern "riscv-interrupt-m" fn f() {}
6    |
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
+    = note: see issue #111889 <https://github.com/rust-lang/rust/issues/111889> for more information
7    = help: add `#![feature(abi_riscv_interrupt)]` to the crate attributes to enable
