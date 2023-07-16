plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:27:06] 
[01:27:06] running 187 tests
[01:27:50] ....................i.....................................................................F.........
[01:28:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:28:14] .....................................F.................................................
[01:28:14] 
[01:28:14] ---- [run-make] run-make-fulldeps/long-linker-command-lines stdout ----
[01:28:14] 
[01:28:14] error: make failed
[01:28:14] error: make failed
[01:28:14] status: exit code: 2
[01:28:14] command: "make"
[01:28:14] stdout:
[01:28:14] ------------------------------------------
[01:28:14] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/long-linker-command-lines'
[01:28:14] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines  foo.rs -g -O
[01:28:14] RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines/foo
[01:28:14] attempt: 100
[01:28:14] Makefile:4: recipe for target 'all' failed
[01:28:14] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/long-linker-command-lines'
[01:28:14] ------------------------------------------
[01:28:14] stderr:
[01:28:14] ------------------------------------------
[01:28:14] thread 'main' panicked at 'status: exit code: 101
[01:28:14] thread 'main' panicked at 'status: exit code: 101
[01:28:14] stdout:
[01:28:14] 
[01:28:14] stderr:
[01:28:14] error: couldn't infer linker flavor from specified linker
[01:28:14] error: aborting due to previous error
[01:28:14] 
[01:28:14] ', foo.rs:64:13
[01:28:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:28:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:28:14] make[1]: *** [all] Error 101
[01:28:14] ------------------------------------------
[01:28:14] 
[01:28:14] thread '[run-make] run-make-fulldeps/long-linker-command-lines' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:28:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:28:14] command: "make"
[01:28:14] stdout:
[01:28:14] ------------------------------------------
[01:28:14] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/reproducible-build'
[01:28:14] rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
[01:28:14] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  linker.rs -O
[01:28:14] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs
[01:28:14] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
[01:28:14] Makefile:12: recipe for target 'smoke' failed
[01:28:14] 
[01:28:14] ------------------------------------------
[01:28:14] stderr:
[01:28:14] ------------------------------------------
[01:28:14] ------------------------------------------
[01:28:14] error: couldn't infer linker flavor from specified linker
[01:28:14] error: aborting due to previous error
[01:28:14] 
[01:28:14] 
[01:28:14] make[1]: *** [smoke] Error 101
[01:28:14] ------------------------------------------
[01:28:14] 
[01:28:14] 
[01:28:14] thread '[run-make] run-make-fulldeps/reproducible-build' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:28:14] 
[01:28:14] failures:
[01:28:14]     [run-make] run-make-fulldeps/long-linker-command-lines
[01:28:14]     [run-make] run-make-fulldeps/reproducible-build
[01:28:14]     [run-make] run-make-fulldeps/reproducible-build
[01:28:14] 
[01:28:14] test result: FAILED. 184 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
[01:28:14] 
[01:28:14] 
[01:28:14] 
[01:28:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:28:14] 
[01:28:14] 
[01:28:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:14] Build completed unsuccessfully in 0:44:53
[01:28:14] Build completed unsuccessfully in 0:44:53
[01:28:14] Makefile:58: recipe for target 'check' failed
[01:28:14] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25299ecb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
