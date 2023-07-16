plain
.................................................................................................... 9000/11226
.................................................................................................... 9100/11226
.................................................................................................... 9200/11226
.............i........i............................................................................. 9300/11226
....................................................iiiiii..iiiiii.i................................ 9400/11226
.................................................................................................... 9600/11226
.................................................................................................... 9700/11226
.................................................................................................... 9800/11226
..............test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 238 tests
iii............iii...ii..i.ii.........i.ii..........i..............i.............i.i...iii.......iii 100/238
..........i.....i.......F.....i.i.i....ii..iiii......................................ii....i.i...i.. 200/238
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/naked-functions.rs stdout ----
---- [codegen] codegen/naked-functions.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll" "/checkout/src/test/codegen/naked-functions.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/naked-functions.rs:12:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: ret void
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:10:2: note: 'next' match was here
 ret void
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:8:7: note: previous match ended here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:9:1: note: non-matching line after previous match is here
 %0 = alloca {}, align 1

------------------------------------------


---
test result: FAILED. 192 passed; 1 failed; 45 ignored; 0 measured; 0 filtered out; finished in 3.45s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:02
