plain
.................................................................................................... 2300/12023
...................................F................................................................ 2400/12023
.................................................................................................... 2500/12023
.................................................................................................... 2600/12023
.........F...................................F...................................................... 2700/12023
.................................................................................................... 2900/12023
.....................................................iiiii.......................................... 3000/12023
.................................................................................................... 3100/12023
.................................................................................................... 3200/12023
---
diff of 64bit.stderr:

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
10    |
11    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
12    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼
+                ╾───────alloc2────────╼ ╾───────alloc5────────╼ │ ╾──────╼╾──────╼
15 
16 error: aborting due to previous error



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/ub-upvars.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-upvars.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-upvars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-upvars.rs:6:1
   |
LL | / const BAD_UPVAR: &dyn FnOnce() = &{ //~ ERROR it is undefined behavior to use this value
LL | |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
LL | |     let another_var = 13;
LL | |     move || { let _ = bad_ref; let _ = another_var; }
LL | | };
   | |__^ type validation failed at .<deref>.<dyn-downcast>.<captured-var(bad_ref)>: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc2────────╼ ╾───────alloc5────────╼ │ ╾──────╼╾──────╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
---- [ui] ui/consts/issue-79690.rs stdout ----
diff of 64bit.stderr:

6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
8    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼
+                ╾───────alloc2────────╼ ╾───────alloc5────────╼ │ ╾──────╼╾──────╼
11 
12 error: aborting due to previous error



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79690/issue-79690.64bit.stderr
To only update this specific test, also pass `--test-args consts/issue-79690.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-79690.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79690" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79690/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/issue-79690.rs:30:1
   |
LL | const G: Fat = unsafe { Transmute { t: FOO }.u };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .1.<deref>.size.foo: encountered (potentially part of) a pointer, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc2────────╼ ╾───────alloc5────────╼ │ ╾──────╼╾──────╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
---- [ui] ui/consts/miri_unleashed/mutable_references_err.rs stdout ----
diff of 64bit.stderr:

19    |
20    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
21    = note: the raw bytes of the constant (size: 16, align: 8) {
-                ╾───────alloc6────────╼ ╾───────alloc7────────╼ │ ╾──────╼╾──────╼
+                ╾───────alloc6────────╼ ╾───────alloc8────────╼ │ ╾──────╼╾──────╼
24 
25 error[E0080]: it is undefined behavior to use this value



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_err/mutable_references_err.64bit.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:17:1
   |
LL | / const MUH: Meh = Meh { //~ ERROR: it is undefined behavior to use this value
LL | |     x: &UnsafeCell::new(42),
LL | | };
   | |__^ type validation failed at .x.<deref>: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:27:1
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:27:1
   |
LL | const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.<dyn-downcast>.x: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc6────────╼ ╾───────alloc8────────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:31:1
   |
   |
LL | const BLUNT: &mut i32 = &mut 42;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered mutable reference in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:18:8
   |
LL |     x: &UnsafeCell::new(42),
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:27:27
   |
   |
LL | const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:31:25
   |
   |
LL | const BLUNT: &mut i32 = &mut 42;

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
---
test result: FAILED. 11923 passed; 3 failed; 97 ignored; 0 measured; 0 filtered out; finished in 121.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:25
