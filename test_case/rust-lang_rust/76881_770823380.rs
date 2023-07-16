plain
.................................................................................................... 2100/11330
.................................................................................................... 2200/11330
...................................................F................................................ 2300/11330
.................................................................................................... 2400/11330
.............................................................F.............F........................ 2500/11330
.................................................................................................... 2700/11330
..............................................................iii.ii................................ 2800/11330
.................................................................................................... 2900/11330
.................................................................................................... 3000/11330
---

---- [ui] ui/const-generics/min_const_generics/invalid-patterns.rs stdout ----
diff of stderr:

29    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
30    |
31    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
+    = note: the raw bytes of the constant (size: 4, align: 4) {
+                __ __ __ __                                     │ ░░░░
32 
33 error[E0080]: it is undefined behavior to use this value
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
34   --> $DIR/invalid-patterns.rs:39:14
34   --> $DIR/invalid-patterns.rs:39:14

37    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
38    |
39    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
+    = note: the raw bytes of the constant (size: 1, align: 1) {
+            }
40 
41 error[E0080]: it is undefined behavior to use this value
42   --> $DIR/invalid-patterns.rs:41:14
42   --> $DIR/invalid-patterns.rs:41:14

45    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
46    |
47    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
+    = note: the raw bytes of the constant (size: 1, align: 1) {
+            }
48 
49 error[E0080]: it is undefined behavior to use this value
50   --> $DIR/invalid-patterns.rs:41:47
50   --> $DIR/invalid-patterns.rs:41:47

53    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
54    |
55    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
+    = note: the raw bytes of the constant (size: 4, align: 4) {
+                __ __ __ __                                     │ ░░░░
56 
57 error: aborting due to 8 previous errors
58 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/invalid-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/min_const_generics/invalid-patterns.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:28:21
   |
LL |   get_flag::<false, 0xFF>();
   |                     ^^^^ expected `char`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:30:14
   |
   |
LL |   get_flag::<7, 'c'>();
   |              ^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:32:14
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |              ^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:32:18
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |                  ^^^^^ expected `char`, found `u8`
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:37:21
   |
   |
LL |   get_flag::<false, { unsafe { char_raw.character } }>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:39:14
   |
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, 'z'>();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:41:14
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:41:14
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:41:47
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:41:47
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0080, E0308.
Some errors have detailed explanations: E0080, E0308.
For more information about an error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/consts/const-points-to-static.rs stdout ----

error: /checkout/src/test/ui/consts/const-points-to-static.rs:5: unexpected note: '5:1: 5:30: the raw bytes of the constant (size: 8, align: 8) {'
error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-points-to-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-points-to-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-points-to-static/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Note,
            Note,
        ),
        msg: "5:1: 5:30: the raw bytes of the constant (size: 8, align: 8) {",
]

thread '[ui] ui/consts/const-points-to-static.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/consts/miri_unleashed/const_refers_to_static2.rs stdout ----

error: /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:10: unexpected note: '10:1: 15:3: the raw bytes of the constant (size: 8, align: 8) {'

error: /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:18: unexpected note: '18:1: 23:3: the raw bytes of the constant (size: 8, align: 8) {'
error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Note,
            Note,
        ),
        msg: "10:1: 15:3: the raw bytes of the constant (size: 8, align: 8) {",
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "18:1: 23:3: the raw bytes of the constant (size: 8, align: 8) {",
]

thread '[ui] ui/consts/miri_unleashed/const_refers_to_static2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs stdout ----

error: /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:11: unexpected note: '11:1: 15:3: the raw bytes of the constant (size: 8, align: 8) {'

error: /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:17: unexpected note: '17:1: 21:3: the raw bytes of the constant (size: 8, align: 8) {'
error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Note,
            Note,
        ),
        msg: "11:1: 15:3: the raw bytes of the constant (size: 8, align: 8) {",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "17:1: 21:3: the raw bytes of the constant (size: 8, align: 8) {",
]

thread '[ui] ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---
test result: FAILED. 11238 passed; 4 failed; 88 ignored; 0 measured; 0 filtered out; finished in 133.35s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:42
