plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/static-pie/static-pie:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/static-pie/static-pie -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/static-pie/static-pie  --target x86_64-unknown-linux-gnu -C target-feature=+crt-static test-aslr.rs
# Check that no dynamic interpreter is set
! readelf -l /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/static-pie/static-pie/test-aslr | "/checkout/src/etc/cat-and-grep.sh" INTERP
[[[ begin stdout ]]]

Elf file type is EXEC (Executable file)
Entry point 0x400b80
There are 7 program headers, starting at offset 64
Program Headers:
Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  LOAD           0x0000000000000000 0x0000000000400000 0x0000000000400000
                 0x0000000000148ba6 0x0000000000148ba6  R E    0x200000
  LOAD           0x0000000000149320 0x0000000000749320 0x0000000000749320
                 0x0000000000009040 0x000000000000f610  RW     0x200000
  NOTE           0x00000000000001c8 0x00000000004001c8 0x00000000004001c8
                 0x0000000000000044 0x0000000000000044  R      0x4
  TLS            0x0000000000149320 0x0000000000749320 0x0000000000749320
                 0x0000000000000028 0x0000000000000110  R      0x10
  GNU_EH_FRAME   0x000000000012aa4c 0x000000000052aa4c 0x000000000052aa4c
                 0x00000000000045e4 0x00000000000045e4  R      0x4
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10
  GNU_RELRO      0x0000000000149320 0x0000000000749320 0x0000000000749320
                 0x0000000000006ce0 0x0000000000006ce0  R      0x1
 Section to Segment mapping:
  Segment Sections...
  Segment Sections...
   00     .note.ABI-tag .note.gnu.build-id .rela.plt .init .plt .text __libc_thread_freeres_fn .fini .rodata .stapsdt.base .eh_frame_hdr .eh_frame .gcc_except_table 
   01     .tdata .preinit_array .init_array .fini_array .data.rel.ro .got .data __libc_IO_vtables __libc_atexit __libc_thread_subfreeres .bss __libc_freeres_ptrs 
   02     .note.ABI-tag .note.gnu.build-id 
   03     .tdata .tbss 
   04     .eh_frame_hdr 
   05     
   06     .tdata .preinit_array .init_array .fini_array .data.rel.ro .got 

[[[ end stdout ]]]
Error: cannot match: INTERP
# Check that we have a dynamic executable
readelf -l /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/static-pie/static-pie/test-aslr | "/checkout/src/etc/cat-and-grep.sh" DYNAMIC
[[[ begin stdout ]]]

Elf file type is EXEC (Executable file)
Entry point 0x400b80
There are 7 program headers, starting at offset 64
Program Headers:
Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  LOAD           0x0000000000000000 0x0000000000400000 0x0000000000400000
                 0x0000000000148ba6 0x0000000000148ba6  R E    0x200000
  LOAD           0x0000000000149320 0x0000000000749320 0x0000000000749320
                 0x0000000000009040 0x000000000000f610  RW     0x200000
  NOTE           0x00000000000001c8 0x00000000004001c8 0x00000000004001c8
                 0x0000000000000044 0x0000000000000044  R      0x4
  TLS            0x0000000000149320 0x0000000000749320 0x0000000000749320
                 0x0000000000000028 0x0000000000000110  R      0x10
  GNU_EH_FRAME   0x000000000012aa4c 0x000000000052aa4c 0x000000000052aa4c
                 0x00000000000045e4 0x00000000000045e4  R      0x4
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10
  GNU_RELRO      0x0000000000149320 0x0000000000749320 0x0000000000749320
                 0x0000000000006ce0 0x0000000000006ce0  R      0x1
 Section to Segment mapping:
  Segment Sections...
  Segment Sections...
   00     .note.ABI-tag .note.gnu.build-id .rela.plt .init .plt .text __libc_thread_freeres_fn .fini .rodata .stapsdt.base .eh_frame_hdr .eh_frame .gcc_except_table 
   01     .tdata .preinit_array .init_array .fini_array .data.rel.ro .got .data __libc_IO_vtables __libc_atexit __libc_thread_subfreeres .bss __libc_freeres_ptrs 
   02     .note.ABI-tag .note.gnu.build-id 
   03     .tdata .tbss 
   04     .eh_frame_hdr 
   05     
   06     .tdata .preinit_array .init_array .fini_array .data.rel.ro .got 

[[[ end stdout ]]]
Error: cannot match: DYNAMIC
Makefile:11: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make/static-pie

test result: FAILED. 14 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out; finished in 2.27s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:24:36
