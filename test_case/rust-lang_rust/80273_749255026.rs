plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
.................................................................................................... 9800/11196
.................................................................................................... 9900/11196
---
 finished in 0.419 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.067 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii....Fiiii.....i 100/116
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [debuginfo-gdb] debuginfo/rc_arc.rs stdout ----
---- [debuginfo-gdb] debuginfo/rc_arc.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 8001001

error: line not found in debugger output: [...]$1 = Rc(strong=2, weak=1) = {value = 42, strong = 2, weak = 1}
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/rc_arc.gdb/rc_arc.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 8.1.1-0ubuntu1) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x197d: file /checkout/src/test/debuginfo/rc_arc.rs, line 37.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, rc_arc::main () at /checkout/src/test/debuginfo/rc_arc.rs:37
37     print!(""); // #break
$1 = alloc::rc::Rc<i32> {ptr: core::ptr::non_null::NonNull<alloc::rc::RcBox<i32>> {pointer: 0x55555575cae0}, phantom: core::marker::PhantomData<alloc::rc::RcBox<i32>>}
$2 = alloc::sync::Arc<i32> {ptr: core::ptr::non_null::NonNull<alloc::sync::ArcInner<i32>> {pointer: 0x55555575cb00}, phantom: core::marker::PhantomData<alloc::sync::ArcInner<i32>>}
A debugging session is active.

 Inferior 1 [process 26935] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
Python Exception <class 'gdb.error'> There is no member named strong.: 
Python Exception <class 'gdb.error'> There is no member named strong.: 
Python Exception <class 'gdb.error'> There is no member named strong.: 
Python Exception <class 'gdb.error'> There is no member named strong.: 
------------------------------------------




failures:
    [debuginfo-gdb] debuginfo/rc_arc.rs

test result: FAILED. 77 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:11
