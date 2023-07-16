plain
   Compiling synstructure v0.12.4
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
warning: llvm-wrapper/RustWrapper.cpp:9:40: warning: extra tokens at end of #include directive
warning:  #include "llvm/Object/COFFImportFile.h":
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.25
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
---
   Compiling sha-1 v0.9.1
   Compiling rustc-rayon v0.3.1
   Compiling matchers v0.0.1
   Compiling tempfile v3.1.0
warning: llvm-wrapper/RustWrapper.cpp:9:40: warning: extra tokens at end of #include directive
warning:  #include "llvm/Object/COFFImportFile.h":
   Compiling synstructure v0.12.4
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
---
.................................................................................................... 9300/11761
.................................................................................................... 9400/11761
.................................................................................................... 9500/11761
...............................................................................................i.... 9600/11761
..i.F............................................................................................... 9700/11761
.........................................iiiiiii..iiiiii.i.......................................... 9800/11761
.................................................................................................... 10000/11761
.................................................................................................... 10100/11761
.................................................................................................... 10200/11761
.................................................................................................... 10300/11761
---

---- [ui] ui/rfc-2627-raw-dylib/feature-gate-raw-dylib.rs stdout ----
diff of stderr:

- error[E0658]: kind="raw-dylib" is unstable
+ error: `#[link(...)]` with `kind = "raw-dylib"` only supported on Windows
3    |
3    |
4 LL | #[link(name = "foo", kind = "raw-dylib")]
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |
-    = note: see issue #58713 <https://github.com/rust-lang/rust/issues/58713> for more information
-    = note: see issue #58713 <https://github.com/rust-lang/rust/issues/58713> for more information
-    = help: add `#![feature(raw_dylib)]` to the crate attributes to enable
10 error: aborting due to previous error
11 

- For more information about this error, try `rustc --explain E0658`.
- For more information about this error, try `rustc --explain E0658`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/feature-gate-raw-dylib/feature-gate-raw-dylib.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/feature-gate-raw-dylib.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/feature-gate-raw-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/feature-gate-raw-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/feature-gate-raw-dylib/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `#[link(...)]` with `kind = "raw-dylib"` only supported on Windows
   |
   |
LL | #[link(name = "foo", kind = "raw-dylib")]

error: aborting due to previous error


---
test result: FAILED. 11663 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 107.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:38
