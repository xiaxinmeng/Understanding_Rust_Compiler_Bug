plain
[01:34:20] command: "make"
[01:34:20] stdout:
[01:34:20] ------------------------------------------
[01:34:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:34:20] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang  -Clinker=clang -Clinker-plugin-lto=on -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang/librustlib-xlto.a -Copt-level=2 -Ccodegen-units=1 ./rustlib.rs
[01:34:20] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/clang -flto=thin -fuse-ld=lld -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang -lrustlib-xlto -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang/cmain ./cmain.c -O3
[01:34:20] ------------------------------------------
[01:34:20] stderr:
[01:34:20] ------------------------------------------
[01:34:20] ------------------------------------------
[01:34:20] warning: ignoring --out-dir flag due to -o flag
[01:34:20] 
[01:34:20] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/clang: relocation error: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/clang: symbol _ZN4llvm23EnableABIBreakingChecksE version LLVM_8 not defined in file libLLVM-8.so with link time reference
[01:34:20] make: *** [Makefile:12: cpp-executable] Error 127
[01:34:20] ------------------------------------------
[01:34:20] 
[01:34:20] thread '[run-make] run-make-fulldeps/cross-lang-lto-clang' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:34:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:34:20] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 194 filtered out
[01:34:20] 
[01:34:20] 
[01:34:20] 
[01:34:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "clang" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--lldb-version" "lldb version 8.0.0\n  rust-enabled\n" "--lldb-python-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/python2.7/site-packages" "--run-clang-based-tests-with" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/clang" "clang" "--llvm-version" "8.0.0\n" "--cc" "clang" "--cxx" "clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm-project/llvm/include -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -std=c++11  -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_DEBUG -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:34:20] 
[01:34:20] 
[01:34:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-make-fulldeps --test-args clang
[01:34:20] Build completed unsuccessfully in 0:01:34
---
travis_time:end:0aab1f85:start=1551454469118376734,finish=1551454469129212333,duration=10835599
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06a04420
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:008b3770
travis_time:start:008b3770
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13a4850a
$ dmesg | grep -i kill
