plain
........................................................................................ 1144/13299
........................................................................................ 1232/13299
....................................................i................................... 1320/13299
........................................................................................ 1408/13299
..............F.....iF....F............................................................. 1496/13299
........................................................................................ 1672/13299
................................................i.....ii................................ 1760/13299
........................................................................................ 1848/13299
........................................................................................ 1936/13299
---
normalized stderr:
warning: unexpected `cfg` condition value
  --> $DIR/variadic-ffi.rs:15:16
   |
LL |     #[cfg_attr(target_arch = "asmjs", allow(improper_ctypes))]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/variadic-ffi/variadic-ffi.stderr
To only update this specific test, also pass `--test-args abi/variadic-ffi.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/variadic-ffi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/variadic-ffi/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/variadic-ffi/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/abi/variadic-ffi.rs:15:16
   |
   |
LL |     #[cfg_attr(target_arch = "asmjs", allow(improper_ctypes))]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/cfg/cfg-panic.rs stdout ----
normalized stderr:
warning: unexpected `cfg` condition value
  --> $DIR/cfg-panic.rs:15:7
   |
LL | #[cfg(panic = "some_imaginary_future_panic_handler")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `panic` are: abort, unwind
warning: 1 warning emitted


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args cfg/cfg-panic.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/cfg-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-panic/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/cfg/cfg-panic.rs:15:7
   |
   |
LL | #[cfg(panic = "some_imaginary_future_panic_handler")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `panic` are: abort, unwind
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/cfg/cfg-panic-abort.rs stdout ----
normalized stderr:
warning: unexpected `cfg` condition value
  --> $DIR/cfg-panic-abort.rs:12:7
   |
LL | #[cfg(panic = "some_imaginary_future_panic_handler")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `panic` are: abort, unwind
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args cfg/cfg-panic-abort.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/cfg-panic-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-panic-abort" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-panic-abort/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/cfg/cfg-panic-abort.rs:12:7
   |
   |
LL | #[cfg(panic = "some_imaginary_future_panic_handler")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `panic` are: abort, unwind
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/cfg/conditional-compile-arch.rs stdout ----
normalized stderr:
warning: unexpected `cfg` condition value
  --> $DIR/conditional-compile-arch.rs:31:7
   |
LL | #[cfg(target_arch = "asmjs")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args cfg/conditional-compile-arch.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/conditional-compile-arch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/conditional-compile-arch/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/conditional-compile-arch/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/cfg/conditional-compile-arch.rs:31:7
   |
   |
LL | #[cfg(target_arch = "asmjs")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs stdout ----
diff of stderr:

34    = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
35    = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
- error: aborting due to 4 previous errors
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-abi.rs:1:7
+    |
+    |
+ LL | #[cfg(target_abi = "x")]
+    |                    |
+    |                    |
+    |                    help: did you mean: `""`
+    = note: `#[warn(unexpected_cfgs)]` on by default
+    = note: `#[warn(unexpected_cfgs)]` on by default
+    = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-abi.rs:4:12
+    |
+    |
+ LL | #[cfg_attr(target_abi = "x", x)]
+    |                         |
+    |                         |
+    |                         help: did you mean: `""`
+    |
+    = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-abi.rs:7:19
+    |
+    |
+ LL | #[cfg(not(any(all(target_abi = "x"))))]
+    |                                |
+    |                                |
+    |                                help: did you mean: `""`
+    |
+    = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-abi.rs:11:10
+    |
+    |
+ LL |     cfg!(target_abi = "x");
+    |                       |
+    |                       |
+    |                       help: did you mean: `""`
+    |
+    = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
+ error: aborting due to 4 previous errors; 4 warnings emitted
38 
39 For more information about this error, try `rustc --explain E0658`.
40 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg-target-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-abi/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL | #[cfg(target_abi = "x")] //~ ERROR `cfg(target_abi)` is experimental
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable

error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL | #[cfg_attr(target_abi = "x", x)] //~ ERROR `cfg(target_abi)` is experimental
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable

error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL | #[cfg(not(any(all(target_abi = "x"))))] //~ ERROR `cfg(target_abi)` is experimental
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable

error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL |     cfg!(target_abi = "x");
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs:1:7
   |
   |
LL | #[cfg(target_abi = "x")] //~ ERROR `cfg(target_abi)` is experimental
   |                    |
   |                    |
   |                    help: did you mean: `""`
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs:4:12
   |
   |
LL | #[cfg_attr(target_abi = "x", x)] //~ ERROR `cfg(target_abi)` is experimental
   |                         |
   |                         |
   |                         help: did you mean: `""`
   |
   = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs:7:19
   |
   |
LL | #[cfg(not(any(all(target_abi = "x"))))] //~ ERROR `cfg(target_abi)` is experimental
   |                                |
   |                                |
   |                                help: did you mean: `""`
   |
   = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs:11:10
   |
   |
LL |     cfg!(target_abi = "x");
   |                       |
   |                       |
   |                       help: did you mean: `""`
   |
   = note: expected values for `target_abi` are: , abi64, eabi, eabihf, fortanix, ilp32, llvm, macabi, sim, softfloat, spe, uwp, x32
error: aborting due to 4 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/feature-gates/feature-gate-cfg-target-compact.rs stdout ----
diff of stderr:

34    = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
35    = help: add `#![feature(cfg_target_compact)]` to the crate attributes to enable
- error: aborting due to 4 previous errors
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-compact.rs:1:14
+    |
+    |
+ LL | #[cfg(target(os = "x"))]
+    |
+    = note: `#[warn(unexpected_cfgs)]` on by default
+    = note: `#[warn(unexpected_cfgs)]` on by default
+    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-compact.rs:4:19
+    |
+    |
+ LL | #[cfg_attr(target(os = "x"), x)]
+    |
+    |
+    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-compact.rs:7:26
+    |
+    |
+ LL | #[cfg(not(any(all(target(os = "x")))))]
+    |
+    |
+    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
+ warning: unexpected `cfg` condition value
+   --> $DIR/feature-gate-cfg-target-compact.rs:11:17
+    |
+    |
+ LL |     cfg!(target(os = "x"));
+    |
+    |
+    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
+ error: aborting due to 4 previous errors; 4 warnings emitted
38 
39 For more information about this error, try `rustc --explain E0658`.
40 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg-target-compact.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-target-compact.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-compact" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-compact/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: compact `cfg(target(..))` is experimental and subject to change
   |
   |
LL | #[cfg(target(os = "x"))] //~ ERROR compact `cfg(target(..))` is experimental
   |
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = help: add `#![feature(cfg_target_compact)]` to the crate attributes to enable

error[E0658]: compact `cfg(target(..))` is experimental and subject to change
   |
   |
LL | #[cfg_attr(target(os = "x"), x)] //~ ERROR compact `cfg(target(..))` is experimental
   |
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = help: add `#![feature(cfg_target_compact)]` to the crate attributes to enable

error[E0658]: compact `cfg(target(..))` is experimental and subject to change
   |
   |
LL | #[cfg(not(any(all(target(os = "x")))))] //~ ERROR compact `cfg(target(..))` is experimental
   |
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = help: add `#![feature(cfg_target_compact)]` to the crate attributes to enable

error[E0658]: compact `cfg(target(..))` is experimental and subject to change
   |
   |
LL |     cfg!(target(os = "x"));
   |
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = note: see issue #96901 <https://github.com/rust-lang/rust/issues/96901> for more information
   = help: add `#![feature(cfg_target_compact)]` to the crate attributes to enable
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-compact.rs:1:14
   |
   |
LL | #[cfg(target(os = "x"))] //~ ERROR compact `cfg(target(..))` is experimental
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-compact.rs:4:19
   |
   |
LL | #[cfg_attr(target(os = "x"), x)] //~ ERROR compact `cfg(target(..))` is experimental
   |
   |
   = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-compact.rs:7:26
   |
   |
LL | #[cfg(not(any(all(target(os = "x")))))] //~ ERROR compact `cfg(target(..))` is experimental
   |
   |
   = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-target-compact.rs:11:17
   |
   |
LL |     cfg!(target(os = "x"));
   |
   |
   = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
error: aborting due to 4 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
