plain
................i................................................................................... 6000/11260
............i........................................................i.............................. 6100/11260
............................................................................iiii.......i...i........ 6200/11260
.................................................................................................... 6300/11260
..................................i.........................i..................F.................... 6400/11260
.....FF..............................................................i..................FF.......... 6500/11260
...........................................ii..........................................i............ 6700/11260
.................................................................................................... 6800/11260
.................................................................................................... 6900/11260
..........................................i......................................................... 7000/11260
---
failures:

---- [ui] ui/error-codes/E0453.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/issue-70819-dont-override-forbid-in-same-scope.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-70819-dont-override-forbid-in-same-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-70819-dont-override-forbid-in-same-scope" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "deduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-70819-dont-override-forbid-in-same-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-forbid-cmdline.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "deprecated" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-forbid-attr.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---


---- [ui] ui/lint/reasons-forbidden.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons-forbidden.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-forbidden" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "deduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-forbidden/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/outer-forbid.rs stdout ----
diff of stderr:

- error[E0453]: allow(unused_variables) incompatible with previous forbid
-   --> $DIR/outer-forbid.rs:19:9
+ error: function is never used: `foo`
3    |
3    |
- LL | #![forbid(unused, non_snake_case)]
-    |           ------ `forbid` level set here
- LL | 
- LL | #[allow(unused_variables)]
-    |         ^^^^^^^^^^^^^^^^ overruled by previous forbid
- 
- error[E0453]: allow(unused) incompatible with previous forbid
-   --> $DIR/outer-forbid.rs:22:9
+ LL | fn foo() {}
12    |
12    |
- LL | #![forbid(unused, non_snake_case)]
-    |           ------ `forbid` level set here
- ...
- LL | #[allow(unused)]
-    |         ^^^^^^ overruled by previous forbid
- 
- error[E0453]: allow(nonstandard_style) incompatible with previous forbid
-   --> $DIR/outer-forbid.rs:25:9
+ note: the lint level is defined here
21    |
21    |
22 LL | #![forbid(unused, non_snake_case)]
-    |                   -------------- `forbid` level set here
- ...
- LL | #[allow(nonstandard_style)]
-    |         ^^^^^^^^^^^^^^^^^ overruled by previous forbid
+    |           ^^^^^^
+    = note: `#[forbid(dead_code)]` implied by `#[forbid(unused)]`
- error: aborting due to 3 previous errors
+ error: aborting due to previous error
29 
- For more information about this error, try `rustc --explain E0453`.
- For more information about this error, try `rustc --explain E0453`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/outer-forbid/outer-forbid.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/outer-forbid.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/outer-forbid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/outer-forbid" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "deduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/outer-forbid/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: function is never used: `foo`
   |
   |
LL | fn foo() {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/outer-forbid.rs:17:11
   |
   |
LL | #![forbid(unused, non_snake_case)]
   |           ^^^^^^
   = note: `#[forbid(dead_code)]` implied by `#[forbid(unused)]`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11168 passed; 6 failed; 86 ignored; 0 measured; 0 filtered out; finished in 136.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:50
