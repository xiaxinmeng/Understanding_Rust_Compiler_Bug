plain
...............ii................................................................................... 8100/12385
.................................................................................................... 8200/12385
.............i.....................................i................................................ 8300/12385
..............i..................................................................................... 8400/12385
......................................F........................i.................................... 8500/12385
.......................................................2021-11-20T13:35:16.518952Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/parser/issues/auxiliary/issue-89971-outer-attr-following-inner-attr-ice.rs` source not found"
F............................................ 8600/12385
.................................................................................................... 8800/12385
.................................................................................................... 8900/12385
.................................................................................................... 9000/12385
.................................................................................................... 9100/12385
---

---- [ui] ui/parser/issues/issue-21146.rs stdout ----
diff of stderr:

- error: expected one of `!` or `::`, found `<eof>`
-   --> $DIR/auxiliary/issue-21146-inc.rs:3:1
+ error: couldn't read $DIR/auxiliary/issue-21146-inc.rs: No such file or directory (os error 2)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
3    |
- LL | parse_error
- LL | parse_error
-    | ^^^^^^^^^^^ expected one of `!` or `::`
+ LL | include!("auxiliary/issue-21146-inc.rs");
+    |
+    |
+    = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-21146/issue-21146.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-21146.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-21146.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-21146" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-21146/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: couldn't read /checkout/src/test/ui/parser/issues/auxiliary/issue-21146-inc.rs: No such file or directory (os error 2)
   |
   |
LL | include!("auxiliary/issue-21146-inc.rs");
   |
   |
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issues/issue-89971-outer-attr-following-inner-attr-ice.rs stdout ----

error: aux-build `/checkout/src/test/ui/parser/issues/auxiliary/issue-89971-outer-attr-following-inner-attr-ice.rs` source not found
thread '[ui] ui/parser/issues/issue-89971-outer-attr-following-inner-attr-ice.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2083:9

failures:
    [ui] ui/parser/issues/issue-21146.rs
    [ui] ui/parser/issues/issue-89971-outer-attr-following-inner-attr-ice.rs
    [ui] ui/parser/issues/issue-89971-outer-attr-following-inner-attr-ice.rs

test result: FAILED. 12272 passed; 2 failed; 111 ignored; 0 measured; 0 filtered out; finished in 135.73s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:01
