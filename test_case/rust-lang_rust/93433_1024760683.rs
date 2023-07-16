plain
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/a"
stdout:
------------------------------------------
status: signal: 11 (core dumped)
stderr: 

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: stderr.contains(\"has overflowed its stack\\n\")', /checkout/src/test/ui/abi/stack-probes.rs:66:5

------------------------------------------



---- [ui] ui/abi/stack-probes-lto.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a"
stdout:
------------------------------------------
status: signal: 11 (core dumped)
stderr: 

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: stderr.contains(\"has overflowed its stack\\n\")', /checkout/src/test/ui/abi/stack-probes.rs:66:5

------------------------------------------


---
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some(11)`,
 right: `Some(6)`', /checkout/src/test/ui/runtime/out-of-stack.rs:42:5

------------------------------------------


---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:01
*** Segmentation fault
*** Segmentation fault
Register dump:

 RAX: 0000000000000000   RBX: 000055bdf1bd5b30   RCX: 0000000000000000
 RDX: 0000000000000000   RSI: 000055bdf1bd5b00   RDI: 000055bdf1bd1014
 RBP: 0000000000000030   R8 : 0000000000000000   R9 : 00007f512e5dd813
 R10: 8080808080808080   R11: 00007f512e43abe0   R12: 00007ffc67da8980
 R13: 0000000000000003   R14: 0000000000000002   R15: 000055bdf1bd5bc0
 RSP: 00007ffc67da8930

 RIP: 000055bdf031cca1   EFLAGS: 00010246

 CS: 0033   FS: 0000   GS: 0000

 Trap: 0000000e   Error: 00000006   OldMask: 00000000   CR2: 00000001

 FPUCW: 0000037f   FPUSW: 00000000   TAG: 00000000
 RIP: 00000000   RDP: 00000000

 ST(0) 0000 0000000000000000   ST(1) 0000 0000000000000000
 ST(2) 0000 0000000000000000   ST(3) 0000 0000000000000000
 ST(4) 0000 0000000000000000   ST(5) 0000 0000000000000000
 ST(6) 0000 0000000000000000   ST(7) 0000 0000000000000000
 mxcsr: 1f80
 XMM0:  000000000000000000000000f1fdffc0 XMM1:  000000000000000000000000f1fdffc0
 XMM2:  000000000000000000000000f1fdffc0 XMM3:  000000000000000000000000f1fdffc0
 XMM4:  000000000000000000000000f1fdffc0 XMM5:  000000000000000000000000f1fdffc0
 XMM6:  000000000000000000000000f1fdffc0 XMM7:  000000000000000000000000f1fdffc0
 XMM8:  000000000000000000000000f1fdffc0 XMM9:  000000000000000000000000f1fdffc0
 XMM10: 000000000000000000000000f1fdffc0 XMM11: 000000000000000000000000f1fdffc0
 XMM12: 000000000000000000000000f1fdffc0 XMM13: 000000000000000000000000f1fdffc0
 XMM14: 000000000000000000000000f1fdffc0 XMM15: 000000000000000000000000f1fdffc0
Backtrace:
Backtrace:
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a(+0x2ca1)[0x55bdf031cca1]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a(+0x3723)[0x55bdf031d723]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a(+0x3719)[0x55bdf031d719]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so(_ZN3std2rt19lang_start_internal17h4c51e2056dfe1eacE+0x1b)[0x7f512e537abb]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a(+0x37b2)[0x55bdf031d7b2]
/lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf3)[0x7f512e2760b3]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a(+0x21ae)[0x55bdf031c1ae]
Memory map:


55bdf031a000-55bdf031c000 r--p 00000000 08:01 16260127 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a
55bdf031c000-55bdf031e000 r-xp 00002000 08:01 16260127 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a
55bdf031e000-55bdf031f000 r--p 00004000 08:01 16260127 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a
55bdf031f000-55bdf0320000 r--p 00004000 08:01 16260127 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a
55bdf0320000-55bdf0321000 rw-p 00005000 08:01 16260127 /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/signal-exit-status/a
55bdf1bd1000-55bdf1bf2000 rw-p 00000000 00:00 0 [heap]
7f512e246000-7f512e249000 rw-p 00000000 00:00 0
7f512e249000-7f512e24a000 r--p 00000000 00:34 3623539 /usr/lib/x86_64-linux-gnu/libdl-2.31.so
7f512e24a000-7f512e24c000 r-xp 00001000 00:34 3623539 /usr/lib/x86_64-linux-gnu/libdl-2.31.so
7f512e24c000-7f512e24d000 r--p 00003000 00:34 3623539 /usr/lib/x86_64-linux-gnu/libdl-2.31.so
7f512e24d000-7f512e24e000 r--p 00003000 00:34 3623539 /usr/lib/x86_64-linux-gnu/libdl-2.31.so
7f512e24e000-7f512e24f000 rw-p 00004000 00:34 3623539 /usr/lib/x86_64-linux-gnu/libdl-2.31.so
7f512e24f000-7f512e274000 r--p 00000000 00:34 3623528 /usr/lib/x86_64-linux-gnu/libc-2.31.so
7f512e274000-7f512e3ec000 r-xp 00025000 00:34 3623528 /usr/lib/x86_64-linux-gnu/libc-2.31.so
7f512e3ec000-7f512e436000 r--p 0019d000 00:34 3623528 /usr/lib/x86_64-linux-gnu/libc-2.31.so
7f512e436000-7f512e437000 ---p 001e7000 00:34 3623528 /usr/lib/x86_64-linux-gnu/libc-2.31.so
7f512e437000-7f512e43a000 r--p 001e7000 00:34 3623528 /usr/lib/x86_64-linux-gnu/libc-2.31.so
7f512e43a000-7f512e43d000 rw-p 001ea000 00:34 3623528 /usr/lib/x86_64-linux-gnu/libc-2.31.so
7f512e43d000-7f512e443000 rw-p 00000000 00:00 0
7f512e443000-7f512e44a000 r--p 00000000 00:34 3623622 /usr/lib/x86_64-linux-gnu/libpthread-2.31.so
7f512e44a000-7f512e45b000 r-xp 00007000 00:34 3623622 /usr/lib/x86_64-linux-gnu/libpthread-2.31.so
7f512e45b000-7f512e460000 r--p 00018000 00:34 3623622 /usr/lib/x86_64-linux-gnu/libpthread-2.31.so
7f512e460000-7f512e461000 r--p 0001c000 00:34 3623622 /usr/lib/x86_64-linux-gnu/libpthread-2.31.so
7f512e461000-7f512e462000 rw-p 0001d000 00:34 3623622 /usr/lib/x86_64-linux-gnu/libpthread-2.31.so
7f512e462000-7f512e466000 rw-p 00000000 00:00 0
7f512e466000-7f512e469000 r--p 00000000 00:34 3623553 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
7f512e469000-7f512e47b000 r-xp 00003000 00:34 3623553 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
7f512e47b000-7f512e47f000 r--p 00015000 00:34 3623553 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
7f512e47f000-7f512e480000 r--p 00018000 00:34 3623553 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
7f512e480000-7f512e481000 rw-p 00019000 00:34 3623553 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
7f512e487000-7f512e4ed000 r--p 00000000 08:01 14197623 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so
7f512e4ed000-7f512e5c6000 r-xp 00066000 08:01 14197623 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so
7f512e5c6000-7f512e617000 r--p 0013f000 08:01 14197623 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so
7f512e617000-7f512e618000 ---p 00190000 08:01 14197623 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so
7f512e618000-7f512e62a000 r--p 00190000 08:01 14197623 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so
7f512e62a000-7f512e62b000 rw-p 001a2000 08:01 14197623 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b016fd4922ae8247.so
7f512e62b000-7f512e62c000 r--p 00000000 00:34 3623510 /usr/lib/x86_64-linux-gnu/libSegFault.so
7f512e62c000-7f512e62f000 r-xp 00001000 00:34 3623510 /usr/lib/x86_64-linux-gnu/libSegFault.so
7f512e62f000-7f512e630000 r--p 00004000 00:34 3623510 /usr/lib/x86_64-linux-gnu/libSegFault.so
7f512e630000-7f512e631000 r--p 00004000 00:34 3623510 /usr/lib/x86_64-linux-gnu/libSegFault.so
7f512e631000-7f512e632000 rw-p 00005000 00:34 3623510 /usr/lib/x86_64-linux-gnu/libSegFault.so
7f512e632000-7f512e634000 rw-p 00000000 00:00 0
7f512e634000-7f512e635000 r--p 00000000 00:34 3623506 /usr/lib/x86_64-linux-gnu/ld-2.31.so
7f512e635000-7f512e658000 r-xp 00001000 00:34 3623506 /usr/lib/x86_64-linux-gnu/ld-2.31.so
7f512e658000-7f512e660000 r--p 00024000 00:34 3623506 /usr/lib/x86_64-linux-gnu/ld-2.31.so
7f512e661000-7f512e662000 r--p 0002c000 00:34 3623506 /usr/lib/x86_64-linux-gnu/ld-2.31.so
7f512e662000-7f512e663000 rw-p 0002d000 00:34 3623506 /usr/lib/x86_64-linux-gnu/ld-2.31.so
7f512e663000-7f512e664000 rw-p 00000000 00:00 0
7ffc67d89000-7ffc67daa000 rw-p 00000000 00:00 0 [stack]
7ffc67dc8000-7ffc67dcc000 r--p 00000000 00:00 0 [vvar]
7ffc67dcc000-7ffc67dce000 r-xp 00000000 00:00 0 [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0 [vsyscall]
