plain
.................................................................................................... 9200/11446
.................................................................................................... 9300/11446
.................................................................................................... 9400/11446
.....i......i....................................................................................... 9500/11446
...........................................iiiiiii..iiiiii.i........................................ 9600/11446
.................................................................................................... 9800/11446
.................................................................................................... 9900/11446
.................................................................................................... 10000/11446
.................................................................................................... 10100/11446
---

---- [ui] ui/parser/better-expected.rs stdout ----
diff of stderr:

- error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `]`, found `3`
+ error: expected one of ';' or ']', found `3`
3    |
3    |
4 LL |     let x: [isize 3];
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |         -         ^ expected one of 7 possible tokens
+    |         -         ^
6    |         |
7    |         while parsing the type for `x`
7    |         while parsing the type for `x`
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/better-expected/better-expected.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/better-expected.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/better-expected.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/better-expected" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/better-expected/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of ';' or ']', found `3`
   |
   |
LL |     let x: [isize 3]; //~ ERROR expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `]`, found `3`
   |         -         ^
   |         while parsing the type for `x`

error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/removed-syntax-fixed-vec.rs stdout ----
diff of stderr:

- error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `]`, found `*`
+ error: expected one of ';' or ']', found `*`
3    |
3    |
4 LL | type v = [isize * 3];
-    |                 ^ expected one of 7 possible tokens
+    |                 ^
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-fixed-vec/removed-syntax-fixed-vec.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/removed-syntax-fixed-vec.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/removed-syntax-fixed-vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-fixed-vec" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-fixed-vec/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of ';' or ']', found `*`
   |
   |
LL | type v = [isize * 3]; //~ ERROR expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `]`, found `*`

error: aborting due to previous error


---
test result: FAILED. 11351 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 131.83s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:22
