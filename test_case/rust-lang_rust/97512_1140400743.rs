plain

---- [ui] src/test/ui/codemap_tests/unicode.rs stdout ----
diff of stderr:

4 LL | extern "路濫狼á́́" fn foo() {}
5    |        ^^^^^^^^^ invalid ABI
6    |
-    = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
+    = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted, rust-cold
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary"
stdout: none
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
--- stderr -------------------------------
error[E0703]: invalid ABI: found `路濫狼á́́`
   |
   |
LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
   |        ^^^^^^^^^ invalid ABI
   |
   = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted, rust-cold
error: aborting due to previous error

For more information about this error, try `rustc --explain E0703`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/parser/issues/issue-8537.rs stdout ----
diff of stderr:

4 LL |   "invalid-ab_isize"
5    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
6    |
-    = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
+    = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted, rust-cold
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-8537/issue-8537.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-8537.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-8537.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-8537" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-8537/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0703]: invalid ABI: found `invalid-ab_isize`
   |
   |
LL |   "invalid-ab_isize" //~ ERROR invalid ABI
   |   ^^^^^^^^^^^^^^^^^^ invalid ABI
   |
   = help: valid ABIs: Rust, C, C-unwind, cdecl, cdecl-unwind, stdcall, stdcall-unwind, fastcall, fastcall-unwind, vectorcall, vectorcall-unwind, thiscall, thiscall-unwind, aapcs, aapcs-unwind, win64, win64-unwind, sysv64, sysv64-unwind, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted, rust-cold
error: aborting due to previous error

For more information about this error, try `rustc --explain E0703`.
------------------------------------------
