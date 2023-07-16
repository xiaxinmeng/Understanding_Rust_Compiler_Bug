plain
.................................................................................................... 9200/11451
.................................................................................................... 9300/11451
.................................................................................................... 9400/11451
..........i......i.................................................................................. 9500/11451
................................................iiiiiii..iiiiii..i.................................. 9600/11451
.................................................................................................... 9800/11451
.................................................................................................... 9900/11451
.................................................................................................... 10000/11451
.................................................................................................... 10100/11451
---
 finished in 0.442 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.081 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.385 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.487 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiii........

 finished in 0.627 seconds
Build completed successfully in 0:35:06
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 154 tests
................................i............................i..............i....................... 100/154
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.....................i................F..........F....

---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
- // MIR for `drop_in_place` before AddMovesForPackedDrops
+ // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
- fn drop_in_place(_1: *mut [String]) -> () {
+ fn std::ptr::drop_in_place(_1: *mut [String]) -> () {
4     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
5     let mut _2: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
6     let mut _3: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL

thread '[mir-opt] mir-opt/slice-drop-shim.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/slice_drop_shim.core.ptr-drop_in_place.[String].AddMovesForPackedDrops.before.32bit.mir', src/tools/compiletest/src/runtest.rs:3457:25

---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
- // MIR for `drop_in_place` before AddMovesForPackedDrops
+ // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
- fn drop_in_place(_1: *mut Vec<i32>) -> () {
+ fn std::ptr::drop_in_place(_1: *mut Vec<i32>) -> () {
4     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
5     let mut _2: &mut std::vec::Vec<i32>; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
6     let mut _3: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL

thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.32bit.mir', src/tools/compiletest/src/runtest.rs:3457:25

failures:
    [mir-opt] mir-opt/slice-drop-shim.rs
    [mir-opt] mir-opt/unusual-item-types.rs
    [mir-opt] mir-opt/unusual-item-types.rs

test result: FAILED. 148 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.61s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:56
