plain
test [debuginfo-gdb] debuginfo/vec.rs ... ignored

failures:

---- [debuginfo-gdb] debuginfo/pretty-slices.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support

error: line not found in debugger output:  $1 = struct &[i32](size=3) = {0, 1, 2}
status: exit status: 0
command: "arm-linux-androideabi-gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-slices.gdb/pretty-slices.debugger.script"
------------------------------------------
------------------------------------------
0xb6f22a30 in ?? ()
Breakpoint 1 at 0xb6f36dd4: file /checkout/src/test/debuginfo/pretty-slices.rs, line 44.

Breakpoint 1, pretty_slices::main::h2580ea4221c8f0c6 () at /checkout/src/test/debuginfo/pretty-slices.rs:44
44     b(); // #break
$1 = {data_ptr = 0xb6f38fec, length = 3}
$2 = {data_ptr = 0xbeeb4acc, length = 4}
$3 = {data_ptr = 0xb6f38ff8 "string slicemutable string slice\001gdb_load_rust_pretty_printers.py", length = 12}
$4 = {data_ptr = 0xb70b2078 "mutable string sliceQ\017", length = 20}
A debugging session is active.

 Inferior 1 [process 23796] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-slices.gdb/a.
Use `info auto-load python-scripts [REGEXP]' to list them.
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
warning: Could not load shared library symbols for 4 libraries, e.g. /system/bin/linker.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
------------------------------------------




failures:
    [debuginfo-gdb] debuginfo/pretty-slices.rs

test result: FAILED. 80 passed; 1 failed; 46 ignored; 0 measured; 0 filtered out; finished in 122.64s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-arm-linux-androideabi" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:32:23
