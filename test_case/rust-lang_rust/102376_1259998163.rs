plain
..............................iii....................................................... 13552/13601
.................................................
failures:

---- [ui] src/test/ui/macro-only/unsafe-foreign-mod.rs stdout ----


10 LL | unsafe extern "C++" {
11    |               ^^^^^ invalid ABI
12    |
-    = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted, rust-cold
+    = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
15 error: aborting due to 2 previous errors
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-only/unsafe-foreign-mod/unsafe-foreign-mod.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macro-only/unsafe-foreign-mod.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macro-only/unsafe-foreign-mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-only/unsafe-foreign-mod" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-only/unsafe-foreign-mod/auxiliary"
stdout: none
--- stderr -------------------------------
error: extern block cannot be declared unsafe
  --> /checkout/src/test/ui/macro-only/unsafe-foreign-mod.rs:3:1
   |
LL | unsafe extern "C++" { //~ERROR extern block cannot be declared unsafe


error[E0703]: invalid ABI: found `C++`
  --> /checkout/src/test/ui/macro-only/unsafe-foreign-mod.rs:3:15
   |
LL | unsafe extern "C++" { //~ERROR extern block cannot be declared unsafe
   |               ^^^^^ invalid ABI
   |
   = note: invoke `rustc --print=calling-conventions` for a full list of supported calling conventions.
error: aborting due to 2 previous errors
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

For more information about this error, try `rustc --explain E0703`.
For more information about this error, try `rustc --explain E0703`.
------------------------------------------



failures:
    [ui] src/test/ui/macro-only/unsafe-foreign-mod.rs
test result: FAILED. 13478 passed; 1 failed; 122 ignored; 0 measured; 0 filtered out; finished in 126.78s

Build completed unsuccessfully in 0:12:30
