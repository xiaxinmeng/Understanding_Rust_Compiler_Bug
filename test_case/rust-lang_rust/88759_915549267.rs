plain
    Finished release [optimized] target(s) in 25.83s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12116 tests
...............................F....F.FF............................................................ 100/12116
............................................F.iiF....FF........ii................................... 200/12116
.....................................................................................F.............. 300/12116
.................................................................................................... 500/12116
.................................................................................................... 600/12116
.......................................................i............................................ 700/12116
.............................ii..................................................................... 800/12116
.............................ii..................................................................... 800/12116
F................................................................................................... 900/12116
.................................................................................................... 1000/12116
.................................................................................................... 1100/12116
.............................................i...................................................... 1200/12116
.............................F...................................................................... 1300/12116
.................................................................................................... 1400/12116
...................................................................................................i 1500/12116
.FF..i.F..FF..........i............................................................................. 1600/12116
...................i................................................................................ 1800/12116
.................................................................................................... 1900/12116
............................................................................i....................... 2000/12116
.................................................................................................... 2100/12116
---
...............................F.................................................................... 3100/12116
.................................................................................................... 3200/12116
......F............................................................................................. 3300/12116
.................................................................................................... 3400/12116
......F...................................i........i.........i...................................... 3500/12116
FFFFFF......................F....................................................................... 3600/12116
...........ii......................................FF............................................... 3700/12116
.........................................................................................F.......... 3900/12116
.................................................................................................... 4000/12116
.................................................................................................... 4100/12116
.................................................................................................... 4200/12116
---
.................................................................................................... 6100/12116
................i.......................................................................i........... 6200/12116
.................................................................................................... 6300/12116
...................................................ii.ii.......i...i................................ 6400/12116
............FF..FFF................................................................................. 6500/12116
.................................................................................................... 6700/12116
....................i............................................................................... 6800/12116
.....................................i.............................................................. 6900/12116
........................................................ii.......................................... 7000/12116
---
.................................................................................................... 7800/12116
.......................................................i..ii........................................ 7900/12116
......................ii............................................................................ 8000/12116
.................................................................................................... 8100/12116
.......F...................i.................................i.........F............................ 8200/12116
.............................................................i...................................... 8400/12116
.................................................................................................... 8500/12116
.......................................i............................................................ 8600/12116
.................................................................................................... 8700/12116
---
.................................................................................................... 11600/12116
.................................................................................................... 11700/12116
.................................................................................................... 11800/12116
.................................................................................................... 11900/12116
...................F...F............................................................................ 12000/12116
................
failures:

---- [ui] ui/abi/unsupported.rs#aarch64 stdout ----
---- [ui] ui/abi/unsupported.rs#aarch64 stdout ----
diff of stderr:

- error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:26:1
-    |
- LL | extern "ptx-kernel" fn ptx() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
6 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:28:1
-    |
- LL | extern "amdgpu-kernel" fn amdgpu() {}
+ error: aborting due to previous error
12 
12 
- error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:30:1
-    |
- LL | extern "wasm" fn wasm() {}
- 
- 
- error[E0570]: `"aapcs"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:32:1
-    |
- LL | extern "aapcs" fn aapcs() {}
- 
- 
- error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:36:1
-    |
- LL | extern "msp430-interrupt" fn msp430() {}
- 
- 
- error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:38:1
-    |
- LL | extern "avr-interrupt" fn avr() {}
- 
- 
- error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:40:1
-    |
- LL | extern "x86-interrupt" fn x86() {}
- 
- warning: use of calling convention not supported on this target
-   --> $DIR/unsupported.rs:43:1
-    |
-    |
- LL | extern "stdcall" fn stdcall() {}
-    |
-    = note: `#[warn(unsupported_calling_conventions)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
- 
- warning: use of calling convention not supported on this target
-   --> $DIR/unsupported.rs:50:1
-    |
- LL | extern "thiscall" fn thiscall() {}
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
- 
- 
- error: aborting due to 7 previous errors; 2 warnings emitted
- 
- For more information about this error, try `rustc --explain E0570`.
65 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/unsupported.aarch64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `aarch64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "aarch64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=aarch64-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/abi/unsupported.rs#arm stdout ----
diff of stderr:

- error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:26:1
-    |
- LL | extern "ptx-kernel" fn ptx() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
6 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:28:1
-    |
- LL | extern "amdgpu-kernel" fn amdgpu() {}
+ error: aborting due to previous error
12 
12 
- error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:30:1
-    |
- LL | extern "wasm" fn wasm() {}
- 
- 
- error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:36:1
-    |
- LL | extern "msp430-interrupt" fn msp430() {}
- 
- 
- error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:38:1
-    |
- LL | extern "avr-interrupt" fn avr() {}
- 
- 
- error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:40:1
-    |
- LL | extern "x86-interrupt" fn x86() {}
- 
- warning: use of calling convention not supported on this target
-   --> $DIR/unsupported.rs:43:1
-    |
-    |
- LL | extern "stdcall" fn stdcall() {}
-    |
-    = note: `#[warn(unsupported_calling_conventions)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
- 
- warning: use of calling convention not supported on this target
-   --> $DIR/unsupported.rs:50:1
-    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | extern "thiscall" fn thiscall() {}
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
- 
- 
- error: aborting due to 6 previous errors; 2 warnings emitted
- 
- For more information about this error, try `rustc --explain E0570`.
59 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.arm/unsupported.arm.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `arm`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "arm" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.arm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=armv7-unknown-linux-gnueabihf" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.arm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/abi/unsupported.rs#i686 stdout ----


- error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:26:1
-    |
- LL | extern "ptx-kernel" fn ptx() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
6 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:28:1
-    |
- LL | extern "amdgpu-kernel" fn amdgpu() {}
+ error: aborting due to previous error
12 
12 
- error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:30:1
-    |
- LL | extern "wasm" fn wasm() {}
- 
- 
- error[E0570]: `"aapcs"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:32:1
-    |
- LL | extern "aapcs" fn aapcs() {}
- 
- 
- error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:36:1
-    |
- LL | extern "msp430-interrupt" fn msp430() {}
- 
- 
- error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:38:1
-    |
- LL | extern "avr-interrupt" fn avr() {}
- 
- error: aborting due to 6 previous errors
- 
- For more information about this error, try `rustc --explain E0570`.
- For more information about this error, try `rustc --explain E0570`.
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686/unsupported.i686.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`

error in revision `i686`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "i686" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/abi/unsupported.rs#x64 stdout ----


- error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:26:1
-    |
- LL | extern "ptx-kernel" fn ptx() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
6 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:28:1
-    |
- LL | extern "amdgpu-kernel" fn amdgpu() {}
+ error: aborting due to previous error
12 
12 
- error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:30:1
-    |
- LL | extern "wasm" fn wasm() {}
- 
- 
- error[E0570]: `"aapcs"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:32:1
-    |
- LL | extern "aapcs" fn aapcs() {}
- 
- 
- error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:36:1
-    |
- LL | extern "msp430-interrupt" fn msp430() {}
- 
- 
- error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:38:1
-    |
- LL | extern "avr-interrupt" fn avr() {}
- 
- warning: use of calling convention not supported on this target
-   --> $DIR/unsupported.rs:43:1
-    |
-    |
- LL | extern "stdcall" fn stdcall() {}
-    |
-    = note: `#[warn(unsupported_calling_conventions)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
- 
- warning: use of calling convention not supported on this target
-   --> $DIR/unsupported.rs:50:1
-    |
- LL | extern "thiscall" fn thiscall() {}
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
- 
- 
- error: aborting due to 6 previous errors; 2 warnings emitted
- 
- For more information about this error, try `rustc --explain E0570`.
59 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64/unsupported.x64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `x64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "x64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=x86_64-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/asm/bad-arch.rs#mirunsafeck stdout ----
diff of stderr:

12    |
13    = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 3 previous errors
16 
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-arch.mirunsafeck/bad-arch.mirunsafeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-arch.rs`

error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-arch.rs" "-Zthreads=1" "--cfg" "mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-arch.mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "sparc-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-arch.mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0472]: inline assembly is unsupported on this target
  --> /checkout/src/test/ui/asm/bad-arch.rs:22:9
   |
LL |         asm!("");

error[E0472]: inline assembly is unsupported on this target
  --> /checkout/src/test/ui/asm/bad-arch.rs:27:1
   |
   |
LL | global_asm!("");
   |
   |
   = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)

error: requires `drop_in_place` lang_item
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/bad-arch.rs#thirunsafeck stdout ----
diff of stderr:

12    |
13    = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 3 previous errors
16 
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-arch.thirunsafeck/bad-arch.thirunsafeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-arch.rs`

error in revision `thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-arch.rs" "-Zthreads=1" "--cfg" "thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-arch.thirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "sparc-unknown-linux-gnu" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-arch.thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0472]: inline assembly is unsupported on this target
  --> /checkout/src/test/ui/asm/bad-arch.rs:22:9
   |
LL |         asm!("");

error[E0472]: inline assembly is unsupported on this target
  --> /checkout/src/test/ui/asm/bad-arch.rs:27:1
   |
   |
LL | global_asm!("");
   |
   |
   = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)

error: requires `drop_in_place` lang_item
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/inline-syntax.rs#arm stdout ----

error in revision `arm`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "arm" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "armv7-unknown-linux-gnueabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/asm/inline-syntax.rs#x86_64 stdout ----

error in revision `x86_64`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "x86_64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL | global_asm!(".intel_syntax noprefix", "nop");
   |
   = note: `#[warn(bad_asm_style)]` on by default


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix", "nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax aaa noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
---
To only update this specific test, also pass `--test-args associated-types/associated-types-ICE-when-projecting-out-of-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/binding/fn-arg-incomplete-pattern-drop-order.rs stdout ----

error: test run failed!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/c-variadic/variadic-ffi-1.rs stdout ----
diff of stderr:

- error[E0045]: C-variadic function must have C or cdecl calling convention
-   --> $DIR/variadic-ffi-1.rs:9:5
-    |
- LL |     fn printf(_: *const u8, ...);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadics require C or cdecl calling convention
+ error: requires `drop_in_place` lang_item
- error[E0060]: this function takes at least 2 arguments but 0 arguments were supplied
-   --> $DIR/variadic-ffi-1.rs:20:9
-    |
- LL |         foo();
- LL |         foo();
-    |         ^^^-- supplied 0 arguments
-    |         |
-    |         expected at least 2 arguments
-    |
- note: function defined here
-   --> $DIR/variadic-ffi-1.rs:13:8
-    |
- LL |     fn foo(f: isize, x: u8, ...);
+ error: aborting due to previous error
20 
- error[E0060]: this function takes at least 2 arguments but 1 argument was supplied
-   --> $DIR/variadic-ffi-1.rs:21:9
---
-    |
- note: function defined here
-   --> $DIR/variadic-ffi-1.rs:13:8
-    |
- LL |     fn foo(f: isize, x: u8, ...);
- 
- error[E0308]: mismatched types
-   --> $DIR/variadic-ffi-1.rs:23:56
-    |
-    |
- LL |         let x: unsafe extern "C" fn(f: isize, x: u8) = foo;
-    |                -------------------------------------   ^^^ expected non-variadic fn, found variadic function
-    |                expected due to this
-    |
-    |
-    = note: expected fn pointer `unsafe extern "C" fn(_, _)`
-                  found fn item `unsafe extern "C" fn(_, _, ...) {foo}`
- error[E0308]: mismatched types
-   --> $DIR/variadic-ffi-1.rs:24:54
-    |
-    |
- LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar;
-    |                -----------------------------------   ^^^ expected variadic fn, found non-variadic function
-    |                expected due to this
-    |
-    |
-    = note: expected fn pointer `extern "C" fn(_, _, ...)`
-                  found fn item `extern "C" fn(_, _) {bar}`
- 
- error[E0617]: can't pass `f32` to variadic function
-   --> $DIR/variadic-ffi-1.rs:26:19
-    |
- LL |         foo(1, 2, 3f32);
-    |                   ^^^^ help: cast the value to `c_double`: `3f32 as c_double`
- 
- error[E0617]: can't pass `bool` to variadic function
-   --> $DIR/variadic-ffi-1.rs:27:19
-    |
- LL |         foo(1, 2, true);
-    |                   ^^^^ help: cast the value to `c_int`: `true as c_int`
- 
- error[E0617]: can't pass `i8` to variadic function
-   --> $DIR/variadic-ffi-1.rs:28:19
-    |
- LL |         foo(1, 2, 1i8);
-    |                   ^^^ help: cast the value to `c_int`: `1i8 as c_int`
- 
- error[E0617]: can't pass `u8` to variadic function
-   --> $DIR/variadic-ffi-1.rs:29:19
-    |
- LL |         foo(1, 2, 1u8);
-    |                   ^^^ help: cast the value to `c_uint`: `1u8 as c_uint`
- 
- error[E0617]: can't pass `i16` to variadic function
-   --> $DIR/variadic-ffi-1.rs:30:19
-    |
- LL |         foo(1, 2, 1i16);
-    |                   ^^^^ help: cast the value to `c_int`: `1i16 as c_int`
- 
- error[E0617]: can't pass `u16` to variadic function
-   --> $DIR/variadic-ffi-1.rs:31:19
-    |
- LL |         foo(1, 2, 1u16);
-    |                   ^^^^ help: cast the value to `c_uint`: `1u16 as c_uint`
- error: aborting due to 11 previous errors
- 
- Some errors have detailed explanations: E0045, E0060, E0308, E0617.
- For more information about an error, try `rustc --explain E0045`.
- For more information about an error, try `rustc --explain E0045`.
97 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/variadic-ffi-1.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-1.rs stdout ----
diff of stderr:

- error[E0781]: the `"C-cmse-nonsecure-call"` ABI is only allowed on function pointers.
-   --> $DIR/wrong-abi-location-1.rs:8:1
-    |
- LL | pub extern "C-cmse-nonsecure-call" fn test() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0781`.
- For more information about this error, try `rustc --explain E0781`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-1/wrong-abi-location-1.stderr
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-1.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-2.rs stdout ----
diff of stderr:

- error[E0781]: the `"C-cmse-nonsecure-call"` ABI is only allowed on function pointers.
-   --> $DIR/wrong-abi-location-2.rs:8:1
-    |
- LL | / extern "C-cmse-nonsecure-call" {
- LL | |     fn test();
- LL | | }
-    | |_^
+ error: requires `drop_in_place` lang_item
9 error: aborting due to previous error
10 

- For more information about this error, try `rustc --explain E0781`.
- For more information about this error, try `rustc --explain E0781`.
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-2/wrong-abi-location-2.stderr
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-2.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/wrong-abi-location-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/wrong-abi.rs stdout ----
diff of stderr:

- error[E0776]: `#[cmse_nonsecure_entry]` requires C ABI
-   --> $DIR/wrong-abi.rs:9:1
-    |
- LL | #[cmse_nonsecure_entry]
-    | ^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0776`.
---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-entry/wrong-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/wrong-abi.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/wrong-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/wrong-abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/deriving/deriving-with-helper.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deriving/deriving-with-helper.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-with-helper" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-with-helper/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/drop/drop-trait-enum.rs stdout ----

error: test run failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop-trait-enum/a"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'Failed', /checkout/src/test/ui/drop/drop-trait-enum.rs:44:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fatal runtime error: failed to initiate panic, error 5
------------------------------------------


---- [ui] ui/drop/dynamic-drop.rs stdout ----
---- [ui] ui/drop/dynamic-drop.rs stdout ----

error: test run failed!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a"
------------------------------------------

------------------------------------------
stderr:
---


---- [ui] ui/drop/dynamic-drop-async.rs stdout ----

error: test run failed!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs stdout ----
diff of stderr:

124    = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
125    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
- error: aborting due to 14 previous errors
- error: aborting due to 14 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 15 previous errors
128 
129 For more information about this error, try `rustc --explain E0658`.
130 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-avr-interrupt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=avr-unknown-gnu-atmega328" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-non-blocking-interrupt" fn fu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-interrupt" fn f() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn m();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn mu();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn dm() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn dmu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn m() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn mu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn im() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn imu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | type TA = extern "avr-interrupt" fn();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | type TAU = extern "avr-non-blocking-interrupt" fn();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-interrupt" {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-non-blocking-interrupt" {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi-msp430-interrupt.rs stdout ----
diff of stderr:

61    = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
62    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
- error: aborting due to 7 previous errors
- error: aborting due to 7 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 8 previous errors
65 
66 For more information about this error, try `rustc --explain E0658`.
67 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-msp430-interrupt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-msp430-interrupt.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-msp430-interrupt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=msp430-none-elf" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-msp430-interrupt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL | extern "msp430-interrupt" fn f() {}
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "msp430-interrupt" fn m();
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "msp430-interrupt" fn dm() {}
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "msp430-interrupt" fn m() {}
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "msp430-interrupt" fn im() {}
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL | type TA = extern "msp430-interrupt" fn();
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error[E0658]: msp430-interrupt ABI is experimental and subject to change
   |
   |
LL | extern "msp430-interrupt" {}
   |
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = note: see issue #38487 <https://github.com/rust-lang/rust/issues/38487> for more information
   = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi-x86-interrupt.rs stdout ----
diff of stderr:

61    = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
62    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
- error: aborting due to 7 previous errors
- error: aborting due to 7 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 8 previous errors
65 
66 For more information about this error, try `rustc --explain E0658`.
67 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-x86-interrupt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-x86-interrupt.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-x86-interrupt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=x86_64-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-x86-interrupt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL | type A7 = extern "x86-interrupt" fn(); //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL | extern "x86-interrupt" {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi.rs stdout ----
diff of stderr:

226    = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
227    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
228 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:22:32
-    |
- LL |     extern "rust-intrinsic" fn m1();
-    |                                ^^
+ error: requires `drop_in_place` lang_item
234 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:24:36
-    |
- LL |     extern "platform-intrinsic" fn m2();
- 
- 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:13:33
-    |
- LL | extern "rust-intrinsic" fn f1() {}
- 
- 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:15:37
-    |
- LL | extern "platform-intrinsic" fn f2() {}
- 
- 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:37:37
-    |
- LL |     extern "rust-intrinsic" fn m1() {}
- 
- 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:39:41
-    |
- LL |     extern "platform-intrinsic" fn m2() {}
- 
- 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:47:38
-    |
- LL |     extern "rust-intrinsic" fn im1() {}
- 
- 
- error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
-   --> $DIR/feature-gate-abi.rs:49:42
-    |
- LL |     extern "platform-intrinsic" fn im2() {}
- 
- error: aborting due to 34 previous errors
+ error: aborting due to 27 previous errors
278 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:13:8
   |
LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:8
   |
LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:17:8
   |
   |
LL | extern "rust-call" fn f4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | extern "efiapi" fn f10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:22:12
   |
   |
LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:24:12
   |
LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:26:12
   |
   |
LL |     extern "rust-call" fn m4(_: ()); //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn m10(); //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:29:12
   |
   |
LL |     extern "rust-call" fn dm4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn dm10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:37:12
   |
   |
LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:39:12
   |
LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:41:12
   |
   |
LL |     extern "rust-call" fn m4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn m10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:47:12
   |
   |
LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:49:12
   |
LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:51:12
   |
   |
LL |     extern "rust-call" fn im4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn im10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:56:18
   |
   |
LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:57:18
   |
LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:58:18
   |
   |
LL | type A4 = extern "rust-call" fn(_: ()); //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | type A10 = extern "efiapi" fn(); //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:62:8
   |
   |
LL | extern "rust-intrinsic" {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:63:8
   |
LL | extern "platform-intrinsic" {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:64:8
   |
   |
LL | extern "rust-call" {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | extern "efiapi" {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 27 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi_amdgpu_kernel.rs stdout ----
diff of stderr:

61    = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
62    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
63 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/feature-gate-abi_amdgpu_kernel.rs:29:1
-    |
- LL | extern "amdgpu-kernel" {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
69 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/feature-gate-abi_amdgpu_kernel.rs:7:1
-    |
- LL | extern "amdgpu-kernel" fn fu() {}
+ error: aborting due to 8 previous errors
75 
75 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/feature-gate-abi_amdgpu_kernel.rs:12:5
-    |
- LL |     extern "amdgpu-kernel" fn dmu() {}
- 
- 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/feature-gate-abi_amdgpu_kernel.rs:18:5
-    |
- LL |     extern "amdgpu-kernel" fn mu() {}
- 
- 
- error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/feature-gate-abi_amdgpu_kernel.rs:23:5
-    |
- LL |     extern "amdgpu-kernel" fn imu() {}
- 
- error: aborting due to 12 previous errors
- 
- Some errors have detailed explanations: E0570, E0658.
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi_amdgpu_kernel.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL | extern "amdgpu-kernel" fn fu() {} //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL |     extern "amdgpu-kernel" fn mu(); //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL |     extern "amdgpu-kernel" fn dmu() {} //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL |     extern "amdgpu-kernel" fn mu() {} //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL |     extern "amdgpu-kernel" fn imu() {} //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL | type TAU = extern "amdgpu-kernel" fn(); //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error[E0658]: amdgpu-kernel ABI is experimental and subject to change
   |
   |
LL | extern "amdgpu-kernel" {} //~ ERROR amdgpu-kernel ABI is experimental
   |
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = note: see issue #51575 <https://github.com/rust-lang/rust/issues/51575> for more information
   = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi_ptx.rs stdout ----
diff of stderr:

61    = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
62    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
- error: aborting due to 7 previous errors
- error: aborting due to 7 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 8 previous errors
65 
66 For more information about this error, try `rustc --explain E0658`.
67 
67 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_ptx/feature-gate-abi_ptx.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi_ptx.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi_ptx.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_ptx" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=nvptx64-nvidia-cuda" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_ptx/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL | extern "ptx-kernel" fn fu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn mu(); //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn dmu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn mu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn imu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL | type TAU = extern "ptx-kernel" fn(); //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL | extern "ptx-kernel" {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-cfg-target-has-atomic.rs stdout ----
diff of stderr:

268    = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
269    = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable
- error: aborting due to 30 previous errors
- error: aborting due to 30 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 31 previous errors
272 
273 For more information about this error, try `rustc --explain E0658`.
274 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg-target-has-atomic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-has-atomic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-has-atomic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "8")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "8")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "16")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "16")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "32")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "32")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "64")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "64")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "128")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "128")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "ptr")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL | #[cfg(target_has_atomic = "ptr")]
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic = "8");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic = "16");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic = "32");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic = "64");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic = "128");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic = "ptr");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_load_store = "8");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_load_store = "16");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_load_store = "32");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_load_store = "64");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_load_store = "128");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_load_store = "ptr");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_equal_alignment)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_equal_alignment = "8");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_equal_alignment)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_equal_alignment = "16");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_equal_alignment)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_equal_alignment = "32");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_equal_alignment)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_equal_alignment = "64");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_equal_alignment)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_equal_alignment = "128");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_equal_alignment)` is experimental and subject to change
   |
   |
LL |     cfg!(target_has_atomic_equal_alignment = "ptr");
   |
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = note: see issue #32976 <https://github.com/rust-lang/rust/issues/32976> for more information
   = help: add `#![feature(cfg_target_has_atomic)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 31 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-vectorcall.rs stdout ----
diff of stderr:

54    |
55    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
- error: aborting due to 7 previous errors
- error: aborting due to 7 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 8 previous errors
58 
59 For more information about this error, try `rustc --explain E0658`.
60 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-vectorcall.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-vectorcall" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-vectorcall/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:12:8
   |
LL | extern "vectorcall" fn f() {} //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:15:12
   |
   |
LL |     extern "vectorcall" fn m(); //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:17:12
   |
   |
LL |     extern "vectorcall" fn dm() {} //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:22:12
   |
   |
LL |     extern "vectorcall" fn m() {} //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:26:12
   |
   |
LL |     extern "vectorcall" fn im() {} //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:29:18
   |
   |
LL | type TA = extern "vectorcall" fn(); //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-vectorcall.rs:31:8
   |
   |
LL | extern "vectorcall" {} //~ ERROR vectorcall is experimental
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-wasm_abi.rs stdout ----
diff of stderr:

61    = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
62    = help: add `#![feature(wasm_abi)]` to the crate attributes to enable
- error: aborting due to 7 previous errors
- error: aborting due to 7 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 8 previous errors
65 
66 For more information about this error, try `rustc --explain E0658`.
67 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-wasm_abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-wasm_abi.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-wasm_abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=wasm32-unknown-unknown" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-wasm_abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL | extern "wasm" fn fu() {} //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL |     extern "wasm" fn mu(); //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL |     extern "wasm" fn dmu() {} //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL |     extern "wasm" fn mu() {} //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL |     extern "wasm" fn imu() {} //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL | type TAU = extern "wasm" fn(); //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error[E0658]: wasm ABI is experimental and subject to change
   |
   |
LL | extern "wasm" {} //~ ERROR wasm ABI is experimental
   |
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = note: see issue #83788 <https://github.com/rust-lang/rust/issues/83788> for more information
   = help: add `#![feature(wasm_abi)]` to the crate attributes to enable

error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.

---


---- [ui] ui/issues/issue-14875.rs stdout ----

error: test run failed!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14875/a"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/issues/issue-19660.rs stdout ----
diff of stderr:

- error: requires `copy` lang_item
+ error: requires `drop_in_place` lang_item
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19660/issue-19660.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-19660.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-19660.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19660" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19660/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-31076.rs stdout ----
diff of stderr:

- error[E0369]: cannot add `{integer}` to `{integer}`
-   --> $DIR/issue-31076.rs:13:15
-    |
- LL |     let x = 5 + 6;
-    |             - ^ - {integer}
-    |             {integer}
-    |             {integer}
+ error: requires `drop_in_place` lang_item
8 
- error[E0369]: cannot add `i32` to `i32`
-   --> $DIR/issue-31076.rs:15:18
-    |
- LL |     let y = 5i32 + 6i32;
-    |             ---- ^ ---- i32
-    |             i32
+ error: aborting due to previous error
16 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args issues/issue-31076.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lang-items/lang-item-correct-generics.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/lang-item-correct-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-correct-generics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-correct-generics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
---
-    |
- LL |     one()
-    |     ^^^^^
-    |
-    = help: make sure the `fn`/`fn_mut`/`fn_once` lang items are defined and have associated `call`/`call_mut`/`call_once` functions
+ error: requires `drop_in_place` lang_item
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-86238/issue-86238.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-items/issue-86238.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/issue-86238.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-86238" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-86238/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lang-items/lang-item-generic-requirements.rs stdout ----
diff of stderr:

32 LL | struct MyPhantomData<T, U>;
33    |                     ------ this struct has 2 generic arguments
- error[E0392]: parameter `T` is never used
-   --> $DIR/lang-item-generic-requirements.rs:25:22
-    |
-    |
- LL | struct MyPhantomData<T, U>;
-    |                      ^ unused parameter
-    = help: consider removing `T` or referring to it in a field
-    = help: consider removing `T` or referring to it in a field
-    = help: if you intended `T` to be a const parameter, use `const T: usize` instead
+ error: requires `drop_in_place` lang_item
- error[E0392]: parameter `U` is never used
-   --> $DIR/lang-item-generic-requirements.rs:25:25
-    |
-    |
- LL | struct MyPhantomData<T, U>;
-    |                         ^ unused parameter
-    |
-    = help: consider removing `U` or referring to it in a field
-    = help: if you intended `U` to be a const parameter, use `const U: usize` instead
+ error: aborting due to 5 previous errors
- error: aborting due to 6 previous errors
- 
- Some errors have detailed explanations: E0392, E0718.
- For more information about an error, try `rustc --explain E0392`.
---
To only update this specific test, also pass `--test-args lang-items/lang-item-generic-requirements.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/lang-item-generic-requirements.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-generic-requirements" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-generic-requirements/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0718]: `add` language item must be applied to a trait with 1 generic argument
   |
   |
LL | #[lang = "add"]
   | ^^^^^^^^^^^^^^^
LL | trait MyAdd<'a, T> {}
   |            ------- this trait has 2 generic arguments

error[E0718]: `drop_in_place` language item must be applied to a function with at least 1 generic argument
   |
   |
LL | #[lang = "drop_in_place"]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR `drop_in_place` language item must be applied to a function with at least 1 generic
LL | fn my_ptr_drop() {}
   |               - this function has 0 generic arguments

error[E0718]: `index` language item must be applied to a trait with 1 generic argument
   |
   |
LL | #[lang = "index"]
   | ^^^^^^^^^^^^^^^^^
LL | trait MyIndex<'a, T> {}
   |              ------- this trait has 2 generic arguments

error[E0718]: `phantom_data` language item must be applied to a struct with 1 generic argument
   |
   |
LL | #[lang = "phantom_data"]
   | ^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR `phantom_data` language item must be applied to a struct with 1 generic argument
LL | struct MyPhantomData<T, U>;
   |                     ------ this struct has 2 generic arguments

error: requires `drop_in_place` lang_item
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0718`.


------------------------------------------


---- [ui] ui/lang-items/lang-item-missing-generator.rs stdout ----
diff of stderr:

- error: requires `generator` lang_item
-   --> $DIR/lang-item-missing-generator.rs:15:17
-    |
- LL | pub fn abc() -> impl FnOnce(f32) {
-    |                 ^^^^^^^^^^^^^^^^
+ error: requires `drop_in_place` lang_item
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-missing-generator/lang-item-missing-generator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-items/lang-item-missing-generator.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/lang-item-missing-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-missing-generator" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-missing-generator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lang-items/lang-item-missing.rs stdout ----
diff of stderr:

- error: requires `sized` lang_item
+ error: requires `drop_in_place` lang_item
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-missing/lang-item-missing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-items/lang-item-missing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/lang-item-missing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-missing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-missing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/mir/mir_drop_panics.rs stdout ----

error: error pattern 'drop 2' not found!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_panics/a"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/panic-handler/panic-handler-requires-panic-info.rs stdout ----
diff of stderr:

- error: language item required, but not found: `panic_info`
+ error: requires `drop_in_place` lang_item
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-requires-panic-info/panic-handler-requires-panic-info.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-handler/panic-handler-requires-panic-info.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-requires-panic-info.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-requires-panic-info" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-requires-panic-info/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/panics/panic-in-dtor-drops-fields.rs stdout ----

error: test run failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-in-dtor-drops-fields/a"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'explicit panic', /checkout/src/test/ui/panics/panic-in-dtor-drops-fields.rs:21:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fatal runtime error: failed to initiate panic, error 5
------------------------------------------


---- [ui] ui/privacy/privacy1.rs stdout ----
---- [ui] ui/privacy/privacy1.rs stdout ----
diff of stderr:

154 LL |     trait B {
155    |     ^^^^^^^
156 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:77:23
- LL |             fn bar() {}
-    |             -------- private associated function defined here
- ...
- ...
- LL |         self::baz::A::bar();
-    |                       ^^^ private associated function
+ error: requires `drop_in_place` lang_item
165 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:95:13
- LL |         fn bar() {}
-    |         -------- private associated function defined here
- ...
- ...
- LL |     bar::A::bar();
+ error: aborting due to 14 previous errors
174 
174 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:102:19
- LL |         fn bar() {}
-    |         -------- private associated function defined here
- ...
- ...
- LL |         ::bar::A::bar();
- 
- 
- error[E0624]: associated function `bar` is private
-   --> $DIR/privacy1.rs:105:24
- LL |             fn bar() {}
-    |             -------- private associated function defined here
- ...
- ...
- LL |         ::bar::baz::A::bar();
- 
- 
- error[E0624]: associated function `bar2` is private
-   --> $DIR/privacy1.rs:108:23
- LL |             fn bar2(&self) {}
-    |             -------------- private associated function defined here
- ...
- ...
- LL |         ::bar::baz::A.bar2();
- 
- error: aborting due to 18 previous errors
- 
- Some errors have detailed explanations: E0603, E0624.
---
To only update this specific test, also pass `--test-args privacy/privacy1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: module `baz` is private
   |
   |
LL |         use bar::baz::{foo, bar};
   |                  ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         use bar::baz::{foo, bar};
   |                  ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         use bar::baz;
   |                  ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `i` is private
  --> /checkout/src/test/ui/privacy/privacy1.rs:165:20
   |
LL |     use self::foo::i::A; //~ ERROR: module `i` is private
   |                    ^ private module
note: the module `i` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:170:9
   |
LL |         mod i {
LL |         mod i {
   |         ^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A::foo();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A::bar();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A.foo2();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::A.bar2();   //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: trait `B` is private
   |
   |
LL |         ::bar::B::foo();        //~ ERROR: trait `B` is private
   |                ^ private trait
   |
note: the trait `B` is defined here
   |
LL |     trait B {
   |     ^^^^^^^


error[E0603]: function `epriv` is private
   |
   |
LL |             ::bar::epriv(); //~ ERROR: function `epriv` is private
   |
   |
note: the function `epriv` is defined here
   |
   |
LL |         fn epriv();


error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::foo(); //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: module `baz` is private
   |
   |
LL |         ::bar::baz::bar(); //~ ERROR: module `baz` is private
   |                ^^^ private module
note: the module `baz` is defined here
  --> /checkout/src/test/ui/privacy/privacy1.rs:50:5
   |
LL |     mod baz {
LL |     mod baz {
   |     ^^^^^^^

error[E0603]: trait `B` is private
   |
   |
LL |     impl ::bar::B for f32 { fn foo() -> f32 { 1.0 } }
   |                 ^ private trait
   |
note: the trait `B` is defined here
   |
LL |     trait B {
   |     ^^^^^^^


error: requires `drop_in_place` lang_item
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0603`.

---

21 LL | pub fn foo() {}
22    | ^^^^^^^^^^^^ consider importing it directly
23 
- error: requires `sized` lang_item
+ error: requires `drop_in_place` lang_item
26 error: aborting due to 3 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/privacy2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0603]: function import `foo` is private
  --> /checkout/src/test/ui/privacy/privacy2.rs:23:20
   |
LL |     use bar::glob::foo;
   |                    ^^^ private function import
note: the function import `foo` is defined here...
  --> /checkout/src/test/ui/privacy/privacy2.rs:10:13
   |
LL |         use foo;
LL |         use foo;
   |             ^^^
note: ...and refers to the function `foo` which is defined here
   |
LL | pub fn foo() {}
   | ^^^^^^^^^^^^ consider importing it directly


error: requires `drop_in_place` lang_item
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0432, E0603.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.

------------------------------------------


---- [ui] ui/privacy/privacy3.rs stdout ----
diff of stderr:

4 LL |     use bar::gpriv;
5    |         ^^^^^^^^^^ no `gpriv` in `bar`
- error: requires `sized` lang_item
- error: requires `sized` lang_item
+ error: requires `drop_in_place` lang_item
9 error: aborting due to 2 previous errors
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/privacy3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/privacy3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `bar::gpriv`
   |
   |
LL |     use bar::gpriv;
   |         ^^^^^^^^^^ no `gpriv` in `bar`

error: requires `drop_in_place` lang_item
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.

---
10 LL |     mod glob {
11    |     ^^^^^^^^
12 
- error: aborting due to previous error
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 2 previous errors
14 
15 For more information about this error, try `rustc --explain E0603`.
16 
---
To only update this specific test, also pass `--test-args privacy/privacy4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: module `glob` is private
   |
   |
LL |     use bar::glob::gpriv; //~ ERROR: module `glob` is private
   |              ^^^^ private module
   |
note: the module `glob` is defined here
   |
LL |     mod glob {
   |     ^^^^^^^^


error: requires `drop_in_place` lang_item
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0603`.


------------------------------------------


---- [ui] ui/required-lang-item.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/required-lang-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/required-lang-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/required-lang-item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
---
        ExitStatus(
            134,
        ),
    ),
    stdout: "",
    stderr: "thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/rt-explody-panic-payloads.rs:23:17\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\nthread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/rt-explody-panic-payloads.rs:15:9\nfatal runtime error: failed to initiate panic, error 5\n",

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
thread 'main' panicked at 'assertion failed: stderr.map(|v|\n               {\n                   v.ends_with(\"drop of the panic payload panicked\")\n               }).unwrap_or(false)', /checkout/src/test/ui/rt-explody-panic-payloads.rs:27:5

------------------------------------------



---- [ui] ui/static_sized_requirement.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static_sized_requirement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static_sized_requirement" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static_sized_requirement/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `drop_in_place` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/structs-enums/unit-like-struct-drop-run.rs stdout ----

error: test run failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/unit-like-struct-drop-run/a"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'This panic should happen.', /checkout/src/test/ui/structs-enums/unit-like-struct-drop-run.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fatal runtime error: failed to initiate panic, error 5
------------------------------------------


---- [ui] ui/unwind-abis/feature-gate-stdcall-unwind.rs stdout ----
---- [ui] ui/unwind-abis/feature-gate-stdcall-unwind.rs stdout ----
diff of stderr:

61    = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
62    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
63 
- error: aborting due to 7 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 8 previous errors
65 
66 For more information about this error, try `rustc --explain E0658`.
67 
---
To only update this specific test, also pass `--test-args unwind-abis/feature-gate-stdcall-unwind.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-stdcall-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-stdcall-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:12:8
   |
LL | extern "stdcall-unwind" fn fu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:15:12
   |
LL |     extern "stdcall-unwind" fn mu(); //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:16:12
   |
LL |     extern "stdcall-unwind" fn dmu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:21:12
   |
LL |     extern "stdcall-unwind" fn mu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:25:12
   |
LL |     extern "stdcall-unwind" fn imu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:28:19
   |
LL | type TAU = extern "stdcall-unwind" fn(); //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:30:8
   |
LL | extern "stdcall-unwind" {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error: requires `drop_in_place` lang_item
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0658`.

---
117    = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
118    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
119 
- error: aborting due to 14 previous errors
+ error: requires `drop_in_place` lang_item
+ error: aborting due to 15 previous errors
121 
122 For more information about this error, try `rustc --explain E0658`.
123 
---
To only update this specific test, also pass `--test-args unwind-abis/feature-gate-thiscall-unwind.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-thiscall-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-thiscall-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:13:8
   |
LL | extern "thiscall-unwind" fn fu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:14:8
   |
LL | extern "thiscall" fn f() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:17:12
   |
   |
LL |     extern "thiscall" fn m(); //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:18:12
   |
   |
LL |     extern "thiscall-unwind" fn mu(); //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:20:12
   |
LL |     extern "thiscall" fn dm() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:21:12
   |
   |
LL |     extern "thiscall-unwind" fn dmu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:26:12
   |
LL |     extern "thiscall" fn m() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:27:12
   |
   |
LL |     extern "thiscall-unwind" fn mu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:31:12
   |
LL |     extern "thiscall" fn im() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:32:12
   |
   |
LL |     extern "thiscall-unwind" fn imu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:35:18
   |
LL | type TA = extern "thiscall" fn(); //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:36:19
   |
   |
LL | type TAU = extern "thiscall-unwind" fn(); //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:38:8
   |
LL | extern "thiscall" {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:39:8
   |
   |
LL | extern "thiscall-unwind" {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error: requires `drop_in_place` lang_item
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 11963 passed; 51 failed; 102 ignored; 0 measured; 0 filtered out; finished in 132.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:13
