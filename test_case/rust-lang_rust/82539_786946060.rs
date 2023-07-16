plain
.................................................................................................... 9200/11503
.................................................................................................... 9300/11503
.................................................................................................... 9400/11503
......................................................i......i...................................... 9500/11503
.............................................................................................iiiiiii 9600/11503
..iiiiii.i.......................................................................................... 9700/11503
.................................................................................................... 9900/11503
.................................................................................................... 10000/11503
.................................................................................................... 10100/11503
.................................................................................................... 10200/11503
---
 finished in 0.441 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.073 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.28s

 finished in 2.357 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.........................................................................................
test result: ok. 289 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

   Doc-tests core
warning: `doc(include)` is deprecated
   |
   |
12 | #[doc(include = "core_arch_docs.md")]
   |
   |
help: use `#![feature(extended_key_value_attributes)]` instead
   |
12 | #[doc = include_str!("core_arch_docs.md")]

warning: 1 warning emitted


---
Set({"src/doc"}) not skipped for "bootstrap::doc::Standalone" -- not in ["src/tools/tidy"]
Documenting stage2 std (x86_64-unknown-linux-gnu)
Set({"library/alloc", "library/core", "library/panic_abort", "library/panic_unwind", "library/proc_macro", "library/std", "library/term", "library/test", "library/unwind"}) not skipped for "bootstrap::doc::Std" -- not in ["src/tools/tidy"]
 Documenting core v0.0.0 (/checkout/library/core)
warning: `doc(include)` is deprecated
   |
   |
12 | #[doc(include = "core_arch_docs.md")]
   |
   |
help: use `#![feature(extended_key_value_attributes)]` instead
   |
12 | #[doc = include_str!("core_arch_docs.md")]

warning: 1 warning emitted

    Finished release [optimized] target(s) in 16.47s
---

---- [ui] rustdoc-ui/doc-include-deprecated.rs stdout ----
diff of stderr:

- warning: doc(include) is deprecated
+ warning: `doc(include)` is deprecated
3    |
3    |
4 LL | #[doc(include = "auxiliary/docs.md")]

9 LL | #[doc = include_str!("auxiliary/docs.md")]
11 
11 
- warning: doc(include) is deprecated
+ warning: `doc(include)` is deprecated
14    |
14    |
15 LL | #[doc(include = "auxiliary/docs.md")]

20 LL | #[doc = include_str!("auxiliary/docs.md")]
22 
22 
- warning: doc(include) is deprecated
+ warning: `doc(include)` is deprecated
25    |
25    |
26 LL | #![doc(include = "auxiliary/docs.md")]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-include-deprecated/doc-include-deprecated.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-include-deprecated/doc-include-deprecated.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doc-include-deprecated.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-include-deprecated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-include-deprecated" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-include-deprecated/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `doc(include)` is deprecated
   |
   |
LL | #[doc(include = "auxiliary/docs.md")] //~ WARNING deprecated
   |
   |
help: use `#![feature(extended_key_value_attributes)]` instead
   |
LL | #[doc = include_str!("auxiliary/docs.md")] //~ WARNING deprecated


warning: `doc(include)` is deprecated
   |
   |
LL | #[doc(include = "auxiliary/docs.md")] //~ WARNING deprecated
   |
   |
help: use `#![feature(extended_key_value_attributes)]` instead
   |
LL | #[doc = include_str!("auxiliary/docs.md")] //~ WARNING deprecated


warning: `doc(include)` is deprecated
   |
   |
LL | #![doc(include = "auxiliary/docs.md")] //~ WARNING deprecated
   |
   |
help: use `#![feature(extended_key_value_attributes)]` instead
   |
LL | #![doc = include_str!("auxiliary/docs.md")] //~ WARNING deprecated

warning: 3 warnings emitted


---
test result: FAILED. 105 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.30s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:28:31
