
[01:07:49] failures:
[01:07:49] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
[01:07:49] 
[01:07:49] ---- [run-make] run-make/stdin-non-utf8 stdout ----
[01:07:49] 	
[01:07:49] error: make failed
[01:07:49] status: exit code: 2
[01:07:49] command: "make"
[01:07:49] stdout:
[01:07:49] ------------------------------------------
[01:07:49] make[1]: Entering directory '/checkout/src/test/run-make/stdin-non-utf8'
[01:07:49] echo '\xD2' | LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/stdin-non-utf8.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/stdin-non-utf8.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/stdin-non-utf8.stage2-x86_64-unknown-linux-gnu  - 2>&1 \
[01:07:49] 	| "/checkout/src/etc/cat-and-grep.sh" "error: couldn't read from stdin, as it did not contain valid UTF-8"
[01:07:49] [[[ begin stdout ]]]
[01:07:49] error: unknown start of token: \\
[01:07:49]  --> <anon>:1:1
[01:07:49]   |
[01:07:49] 1 | \xD2
[01:07:49]   | ^
[01:07:49] 
[01:07:49] 
[01:07:49] [[[ end stdout ]]]
[01:07:49] Error: cannot match: error: couldn't read from stdin, as it did not contain valid UTF-8
[01:07:49] Makefile:4: recipe for target 'all' failed
[01:07:49] make[1]: Leaving directory '/checkout/src/test/run-make/stdin-non-utf8'
[01:07:49] 
[01:07:49] ------------------------------------------
[01:07:49] stderr:
[01:07:49] ------------------------------------------
[01:07:49] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[01:07:49] make[1]: *** [all] Error 1
[01:07:49] 
[01:07:49] ------------------------------------------
[01:07:49] 
[01:07:49] thread '[run-make] run-make/stdin-non-utf8' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:07:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:49] 
[01:07:49] 
[01:07:49] failures:
[01:07:49]     [run-make] run-make/stdin-non-utf8
[01:07:49] 
[01:07:49] test result: FAILED. 173 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:49] expected success, got: exit code: 101
[01:07:49] 
[01:07:49] 
[01:07:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:49] Build completed unsuccessfully in 0:29:29
[01:07:49] Makefile:52: recipe for target 'check' failed
[01:07:49] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
