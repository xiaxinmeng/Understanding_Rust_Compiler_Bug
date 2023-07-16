plain
.................................................................................................... 9400/11731
.................................................................................................... 9500/11731
........................................................................i......i.................... 9600/11731
.................................................................................................... 9700/11731
..................iiiiiii..iiiiii.i................................................................. 9800/11731
.................................................................................................... 10000/11731
.................................................................................................... 10100/11731
.................................................................................................... 10200/11731
.................................................................................................... 10300/11731
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 268 tests
ii....i.F.iF.....F.Fiii.F......ii.....i.iii.........i.ii.........i.i..............i............i.i.. 100/268
.iii........ii.iiii.i.........i.....i...iiii.....F..i.i.i...iii..iiii............................... 200/268
......i.ii...i..ii..i....................iiiii.ii...................
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] codegen/asm-multiple-options.rs stdout ----

error: compilation failed!
error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/asm-multiple-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-multiple-options" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-multiple-options/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: variable `y` is assigned to, but never used
   |
   |
12 |     let y: i32;
   |
   = note: `#[warn(unused_variables)]` on by default
   = note: `#[warn(unused_variables)]` on by default
   = note: consider using `_y` instead

warning: value assigned to `y` is never read
   |
   |
13 |     asm!("", out("ax") y, in("bx") x, options(pure), options(nomem));
   |
   = note: `#[warn(unused_assignments)]` on by default
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
13 |     asm!("", out("ax") y, in("bx") x, options(pure), options(nomem));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
25 |     asm!("", out("ax") DUMMY_OUTPUT, options(pure), options(readonly));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
37 |     asm!("", out("ax") DUMMY_OUTPUT, options(pure), options(nomem));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
50 |     asm!("", out("ax") DUMMY_OUTPUT, options(pure), options(readonly));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option
error: aborting due to 4 previous errors; 2 warnings emitted


------------------------------------------
------------------------------------------


---- [codegen] codegen/asm-options.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/asm-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-options" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-options/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: variable `y` is assigned to, but never used
   |
   |
12 |     let y: i32;
   |
   = note: `#[warn(unused_variables)]` on by default
   = note: `#[warn(unused_variables)]` on by default
   = note: consider using `_y` instead

warning: value assigned to `y` is never read
   |
   |
13 |     asm!("", out("ax") y, in("bx") x, options(pure, nomem));
   |
   = note: `#[warn(unused_assignments)]` on by default
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
13 |     asm!("", out("ax") y, in("bx") x, options(pure, nomem));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
21 |     asm!("", options(noreturn));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
33 |     asm!("", out("ax") DUMMY_OUTPUT, options(pure, readonly));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
43 |     asm!("", out("ax") DUMMY_OUTPUT, options());
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
55 |     asm!("", out("ax") DUMMY_OUTPUT, options(pure, nomem));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
68 |     asm!("", out("ax") DUMMY_OUTPUT, options(nomem));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
81 |     asm!("", out("ax") DUMMY_OUTPUT, options(pure, readonly));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
93 |     asm!("", options());
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
94 |     asm!("", options(nomem));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
95 |     asm!("", options(readonly));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option
error: aborting due to 10 previous errors; 2 warnings emitted


------------------------------------------
------------------------------------------


---- [codegen] codegen/asm-sanitize-llvm.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/asm-sanitize-llvm.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-sanitize-llvm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-sanitize-llvm/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
22 | /     asm!(
22 | /     asm!(
23 | |         r"banana$:",
   | |_____^
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
29 | /     asm!(
29 | /     asm!(
30 | |         r"banana\36:",
   | |_____^
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [codegen] codegen/asm-target-clobbers.rs#avx512 stdout ----

error in revision `avx512`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/asm-target-clobbers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "avx512" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-target-clobbers.avx512" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx512f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-target-clobbers.avx512/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
13 |     asm!("", out("zmm31") _, options(nostack, nomem, preserves_flags));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
20 |     asm!("", out("eax") _, options(nostack, nomem, preserves_flags));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [codegen] codegen/asm-target-clobbers.rs#base stdout ----

error in revision `base`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/asm-target-clobbers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-target-clobbers.base" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-target-clobbers.base/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
13 |     asm!("", out("zmm31") _, options(nostack, nomem, preserves_flags));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option

error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
20 |     asm!("", out("eax") _, options(nostack, nomem, preserves_flags));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [codegen] codegen/naked-noinline.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/naked-noinline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Intel syntax inline assembly requires LLVM 10.0.1 or later
   |
   |
17 |     asm!("", options(noreturn));
   |
   |
   = help: Consider using AT&T syntax instead with the att_syntax option
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 201 passed; 6 failed; 61 ignored; 0 measured; 0 filtered out; finished in 2.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:32
