plain

---- [ui] ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs stdout ----
diff of stderr:

1 error: changes to closure capture in Rust 2021 will affect which traits the closure implements
-   --> $DIR/mir_calls_to_shims.rs:20:38
3    |
4 LL |     let result = panic::catch_unwind(move || {
5    |                                      ^^^^^^^


11    |         --- in Rust 2018, this closure captures all of `f`, but in Rust 2021, it will only capture `f.0`
13 note: the lint level is defined here
-   --> $DIR/mir_calls_to_shims.rs:3:9
+   --> $DIR/mir_calls_to_shims.rs:4:9
15    |
---
3 #![deny(rust_2021_incompatible_closure_captures)]
4 //~^ NOTE: the lint level is defined here


The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims/mir_calls_to_shims.fixed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs`

error: 2 errors occurred comparing output.
error: 2 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: changes to closure capture in Rust 2021 will affect which traits the closure implements
   |
LL |     let result = panic::catch_unwind(move || {
   |                                      ^^^^^^^
   |                                      |
   |                                      |
   |                                      in Rust 2018, this closure implements `UnwindSafe` as `f` implements `UnwindSafe`, but in Rust 2021, this closure will no longer implement `UnwindSafe` because `f` is not fully captured and `f.0` does not implement `UnwindSafe`
   |                                      in Rust 2018, this closure implements `RefUnwindSafe` as `f` implements `RefUnwindSafe`, but in Rust 2021, this closure will no longer implement `RefUnwindSafe` because `f` is not fully captured and `f.0` does not implement `RefUnwindSafe`
LL |         f.0()
LL |         f.0()
   |         --- in Rust 2018, this closure captures all of `f`, but in Rust 2021, it will only capture `f.0`
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs:4:9
   |
LL | #![deny(rust_2021_incompatible_closure_captures)]
LL | #![deny(rust_2021_incompatible_closure_captures)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f` to be fully captured
   |
LL ~     let result = panic::catch_unwind(move || {
LL +         let _ = &f;

error: aborting due to previous error


---
test result: FAILED. 12306 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 145.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:17
