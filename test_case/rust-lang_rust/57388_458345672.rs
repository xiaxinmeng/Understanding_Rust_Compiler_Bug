plain
[02:25:08] status: exit code: 2
[02:25:08] command: "make"
[02:25:08] stdout:
[02:25:08] ------------------------------------------
[02:25:08] LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-bootstrap-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as -L /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as  hello.rs -C no_integrated_as
[02:25:08] Makefile:6: recipe for target 'all' failed
[02:25:08] ------------------------------------------
[02:25:08] stderr:
[02:25:08] ------------------------------------------
[02:25:08] error: linking with `cc` failed: exit code: 1
[02:25:08] error: linking with `cc` failed: exit code: 1
[02:25:08] 
[02:25:08] note: "cc" "-c" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s"
[02:25:08] 
[02:25:08] note: /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s: Assembler messages:
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:8: Error: invalid instruction suffix for `push'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:10: Error: invalid instruction suffix for `push'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:12: Error: invalid instruction suffix for `push'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:16: Error: bad register expression
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:17: Error: bad register expression
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:17: Error: register save offset not a multiple of 8
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:18: Error: bad register expression
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:19: Error: invalid instruction suffix for `call'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:22: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:26: Error: 8-byte relocation cannot be applied to 4-byte field
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:27: Error: 8-byte relocation cannot be applied to 4-byte field
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:38: Error: invalid instruction suffix for `call'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:43: Error: invalid instruction suffix for `call'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:46: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:48: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:50: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:52: Error: invalid instruction suffix for `ret'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:63: Error: invalid instruction suffix for `push'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:65: Error: invalid instruction suffix for `push'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:69: Error: bad register expression
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:69: Error: register save offset not a multiple of 8
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:70: Error: bad register expression
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:71: Error: invalid instruction suffix for `call'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:74: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:80: Error: 8-byte relocation cannot be applied to 4-byte field
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:85: Error: invalid instruction suffix for `call'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:88: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:90: Error: invalid instruction suffix for `pop'
[02:25:08] /checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps/no-integrated-as/no-integrated-as/hello.hello.7rcbfp3g-cgu.5.rcgu.s:92: Error: invalid instruction suffix for `ret'
[02:25:08] 
[02:25:08] 
[02:25:08] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1453: worker thread panicked', src/librustc/util/bug.rs:37:26
[02:25:08] error: aborting due to previous error
[02:25:08] 
[02:25:08] 
[02:25:08] make: *** [all] Error 1
[02:25:08] ------------------------------------------
[02:25:08] 
[02:25:08] thread '[run-make] run-make-fulldeps/no-integrated-as' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3288:9
[02:25:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[02:25:08] 
[02:25:08] 
[02:25:08] 
[02:25:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[02:25:08] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "run-make" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m32 -march=i686" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm-project/llvm/include -I/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/include -std=c++11  -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_DEBUG -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:25:08] 
[02:25:08] 
[02:25:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:25:08] Build completed unsuccessfully in 2:21:41
---
travis_time:end:0b3f9900:start=1548718365891812987,finish=1548718365911166061,duration=19353074
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:173ec3ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:32b0f8e4
travis_time:start:32b0f8e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2ac36eeb
$ dmesg | grep -i kill
