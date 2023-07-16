plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.181 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.424 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 11.84s
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 221 tests
..........iii...ii.....................F......................................F..................... 100/221
............i..................iiiiii.......i................iii...........ii....................... 200/221
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/extern-fn-generic stdout ----
---- [run-make] run-make-fulldeps/extern-fn-generic stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.o test.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.o
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic  testcrate.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic  test.rs
Makefile:4: recipe for target 'all' failed
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.o
------------------------------------------
stderr:
------------------------------------------
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-Wl,--as-needed" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.7rcbfp3g-cgu.9.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.5fi6c8ty3hqyycqf.rcgu.o" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtestcrate.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-51387cd2c9c30626.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-ecd5c5e1c282b643.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-c0143bf2c50b255c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-7bf6e3fbc40c5d01.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d3331d893ee64d71.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a669e8d4205ae232.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-b576880e691d50f6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e4ddc778e131365c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-305d2990e66c6637.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-1454b20899b9b9a5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-d36b28ae42a82410.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-2f7cba1229903bae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-b9e8fff02dfe2688.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-53fd254116b71059.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-c026c5cf6e838fcf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-42ab9f2e74780da6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-eabf50a85df947a1.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b047d35ef14db125.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--whole-archive" "-ltest" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test" "-pie" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--gc-sections" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.a(libtest.o): In function `call':
          test.c:(.text.call+0x0): multiple definition of `call'
          /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtestcrate.rlib(libtest.o):test.c:(.text.call+0x0): first defined here
          collect2: error: ld returned 1 exit status

error: aborting due to previous error


make: *** [all] Error 1
------------------------------------------



---- [run-make] run-make-fulldeps/issue-28595 stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/liba.o a.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/liba.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/liba.o
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/libb.o b.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/libb.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/libb.o
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595  a.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595  b.rs
Makefile:4: recipe for target 'all' failed
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/liba.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/libb.o
------------------------------------------
stderr:
------------------------------------------
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-Wl,--as-needed" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.b.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b.2y1usvmuzvm153n2.rcgu.o" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/liba.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-51387cd2c9c30626.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-ecd5c5e1c282b643.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-c0143bf2c50b255c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-7bf6e3fbc40c5d01.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-d3331d893ee64d71.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a669e8d4205ae232.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-b576880e691d50f6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e4ddc778e131365c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-305d2990e66c6637.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-1454b20899b9b9a5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-d36b28ae42a82410.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-2f7cba1229903bae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-b9e8fff02dfe2688.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-53fd254116b71059.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-c026c5cf6e838fcf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-42ab9f2e74780da6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-eabf50a85df947a1.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b047d35ef14db125.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--whole-archive" "-lb" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/b" "-pie" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--gc-sections" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-28595/issue-28595/libb.a(libb.o): In function `b':
          b.c:(.text.b+0x5): undefined reference to `a'
          collect2: error: ld returned 1 exit status

error: aborting due to previous error


make: *** [all] Error 1
------------------------------------------



---
test result: FAILED. 199 passed; 2 failed; 20 ignored; 0 measured; 0 filtered out; finished in 21.71s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:28:01
