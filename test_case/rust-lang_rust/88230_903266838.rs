plain

---- [ui] ui/closures/2229_closure_analysis/migrations/insignificant_drop.rs stdout ----
diff of fixed:

3 #![deny(rust_2021_incompatible_closure_captures)]
4 //~^ NOTE: the lint level is defined here
5 
- // Test cases for types that implement a insignificant drop (stlib defined)
+ // Test cases for types that implement an insignificant drop (stlib defined)
7 
8 // `t` needs Drop because one of its elements needs drop,
9 // therefore precise capture might affect drop ordering

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop/insignificant_drop.fixed
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/insignificant_drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = || {
   |             ^^
...
...
LL |         let _t = t.0;
   |                  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL |         let _t1 = t1.0;
   |                   ---- in Rust 2018, this closure captures all of `t1`, but in Rust 2021, it will only capture `t1.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `t1`, but in Rust 2021, it will only capture `t1.0`
LL |         let _t2 = t2.0;
   |                   ---- in Rust 2018, this closure captures all of `t2`, but in Rust 2021, it will only capture `t2.0`
LL | }
   | -
   | |
   | |
   | in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure
   | in Rust 2018, `t1` is dropped here, but in Rust 2021, only `t1.0` will be dropped here as part of the closure
   | in Rust 2018, `t2` is dropped here, but in Rust 2021, only `t2.0` will be dropped here as part of the closure
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.rs:3:9
   |
   |
LL | #![deny(rust_2021_incompatible_closure_captures)]
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t`, `t1`, `t2` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = (&t, &t1, &t2);


error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = || {
   |             ^^
...
...
LL |         let _t = t.0;
   |                  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL |         //~^ NOTE: in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL |         let _t1 = t1.0;
   |                   ---- in Rust 2018, this closure captures all of `t1`, but in Rust 2021, it will only capture `t1.0`
LL | }
   | -
   | |
   | |
   | in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure
   | in Rust 2018, `t1` is dropped here, but in Rust 2021, only `t1.0` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t`, `t1` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = (&t, &t1);


error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = || {
   |             ^^
...
...
LL |         let _t = t.0;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL | }
LL | }
   | - in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &t;


error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = || {
   |             ^^
...
...
LL |         let _t = t.0;
   |                  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL | }
LL | }
   | - in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &t;


error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = || {
   |             ^^
...
...
LL |         let _t = t.0;
   |                  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL | }
LL | }
   | - in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &t;


error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = move || {
   |             ^^^^^^^
...
...
LL |         println!("{} {}", t1.1, t.1);
   |                           ----  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.1`
   |                           |
   |                           in Rust 2018, this closure captures all of `t1`, but in Rust 2021, it will only capture `t1.1`
LL | }
   | -
   | |
   | |
   | in Rust 2018, `t1` is dropped here, but in Rust 2021, only `t1.1` will be dropped here as part of the closure
   | in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.1` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t1`, `t` to be fully captured
LL ~     let c = move || {
LL ~     let c = move || {
LL +         let _ = (&t1, &t);


error: changes to closure capture in Rust 2021 will affect drop order
   |
LL |     let c = || {
   |             ^^
...
...
LL |         let _t = t.0;
   |                  --- in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
LL | }
LL | }
   | - in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `t` to be fully captured
   |
LL ~     let c = || {
LL +         let _ = &t;

error: aborting due to 7 previous errors


---
test result: FAILED. 12064 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 121.13s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:14
