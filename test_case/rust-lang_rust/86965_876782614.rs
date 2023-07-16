plain

---- [ui] ui/closures/2229_closure_analysis/migrations/multi_diagnostics.rs stdout ----
diff of fixed:

136     test_capturing_several_disjoint_fields_individually_2();
137     test_multi_traits_issues();
+ 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics/multi_diagnostics.fixed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/multi_diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: changes to closure capture in Rust 2021 will affect `Clone` trait implementation for closure, and drop order
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
...
LL |         let _f_1 = f1.0;
   |                    ---- in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
LL |         //~^ NOTE: in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
LL |         let _f_2 = f2.1;
   |                    ---- in Rust 2018, closure captures all of `f2`, but in Rust 2021, it only captures `f2.1`
LL | }
LL | }
   | - in Rust 2018, `f2` would be dropped here, but in Rust 2021, only `f2.1` would be dropped here alongside the closure
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/multi_diagnostics.rs:2:9
   |
   |
LL | #![deny(rust_2021_incompatible_closure_captures)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1`, `f2` to be fully captured
   |
LL |     let c = || { let _ = (&f1, &f2); 
LL |         //~^ ERROR: `Clone` trait implementation for closure, and drop order
LL |         //~| NOTE: in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `f1`, `f2` to be fully captured
LL |         let _f_1 = f1.0;


error: changes to closure capture in Rust 2021 will affect `Clone` trait implementation for closure
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
...
LL |         let _f_1 = f1.0;
   |                    ---- in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
   |
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1` to be fully captured
   |
LL |     let c = || { let _ = &f1; 
LL |         //~^ ERROR: `Clone` trait implementation for closure
LL |         //~| NOTE: in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `f1` to be fully captured
LL |         let _f_1 = f1.0;


error: changes to closure capture in Rust 2021 will affect `Clone` trait implementation for closure
   |
LL |     let c = || {
   |             ^^
   |             |
   |             |
   |             in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
   |             in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.2` does not implement `Clone`
...
LL |         let _f_0 = f1.0;
   |                    ---- in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
LL |         //~^ NOTE: in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
LL |         let _f_2 = f1.2;
   |                    ---- in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.2`
   |
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1` to be fully captured
   |
LL |     let c = || { let _ = &f1; 
LL |         //~^ ERROR: `Clone` trait implementation for closure
LL |         //~| NOTE: in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
LL |         //~| NOTE: in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.2` does not implement `Clone`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `f1` to be fully captured


error: changes to closure capture in Rust 2021 will affect `Clone` trait implementation for closure, and drop order
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
...
LL |         let _f_0 = f1.0;
   |                    ---- in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
LL |         //~^ NOTE: in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.0`
LL |         let _f_1 = f1.1;
   |                    ---- in Rust 2018, closure captures all of `f1`, but in Rust 2021, it only captures `f1.1`
LL | }
   | -
   | |
   | |
   | in Rust 2018, `f1` would be dropped here, but in Rust 2021, only `f1.0` would be dropped here alongside the closure
   | in Rust 2018, `f1` would be dropped here, but in Rust 2021, only `f1.1` would be dropped here alongside the closure
   |
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f1` to be fully captured
   |
LL |     let c = || { let _ = &f1; 
LL |         //~^ ERROR: `Clone` trait implementation for closure, and drop order
LL |         //~| NOTE: in Rust 2018, this closure would implement `Clone` as `f1` implements `Clone`, but in Rust 2021, this closure would no longer implement `Clone` as `f1.0` does not implement `Clone`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `f1` to be fully captured
LL |         let _f_0 = f1.0;


error: changes to closure capture in Rust 2021 will affect `Sync`, `Send` trait implementation for closure
   |
   |
LL |     thread::spawn(move || unsafe {
   |                   |
   |                   |
   |                   in Rust 2018, this closure would implement `Sync`, `Send` as `fptr1` implements `Sync`, `Send`, but in Rust 2021, this closure would no longer implement `Sync`, `Send` as `fptr1.0.0` does not implement `Sync`, `Send`
   |                   in Rust 2018, this closure would implement `Send` as `fptr2` implements `Send`, but in Rust 2021, this closure would no longer implement `Send` as `fptr2.0` does not implement `Send`
...
LL |         *fptr1.0.0 = 20;
   |         ---------- in Rust 2018, closure captures all of `fptr1`, but in Rust 2021, it only captures `fptr1.0.0`
LL |         //~^ NOTE: in Rust 2018, closure captures all of `fptr1`, but in Rust 2021, it only captures `fptr1.0.0`
LL |         *fptr2.0 = 20;
   |         -------- in Rust 2018, closure captures all of `fptr2`, but in Rust 2021, it only captures `fptr2.0`
   |
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `fptr1`, `fptr2` to be fully captured
   |
LL |     thread::spawn(move || { let _ = (&fptr1, &fptr2); unsafe {
LL |         //~^ ERROR: `Sync`, `Send` trait implementation for closure
LL |         //~| NOTE: in Rust 2018, this closure would implement `Sync`, `Send` as `fptr1` implements `Sync`, `Send`, but in Rust 2021, this closure would no longer implement `Sync`, `Send` as `fptr1.0.0` does not implement `Sync`, `Send`
LL |         //~| NOTE: in Rust 2018, this closure would implement `Send` as `fptr2` implements `Send`, but in Rust 2021, this closure would no longer implement `Send` as `fptr2.0` does not implement `Send`
LL |         //~| NOTE: for more information, see
LL |         //~| HELP: add a dummy let to cause `fptr1`, `fptr2` to be fully captured

error: aborting due to 5 previous errors


---
test result: FAILED. 11966 passed; 1 failed; 100 ignored; 0 measured; 0 filtered out; finished in 120.67s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:02
