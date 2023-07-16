plain
travis_time:end:01ee0784:start=1550197315098556938,finish=1550197318591352780,duration=3492795842
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:30] 
[01:12:30] running 119 tests
[01:12:55] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:59] i......iii.i.....ii
[01:12:59] 
[01:12:59]  finished in 29.695
[01:12:59] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:39:19] 
[01:39:19] running 195 tests
[01:39:45] ..................i...i............................................F....................i........... 100/195
[01:40:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:40:31] ......F............................................i......................................i....
[01:40:31] 
[01:40:31] ---- [run-make] run-make-fulldeps/issue-14500 stdout ----
[01:40:31] 
[01:40:31] error: make failed
[01:40:31] error: make failed
[01:40:31] status: exit code: 2
[01:40:31] command: "make"
[01:40:31] stdout:
[01:40:31] ------------------------------------------
[01:40:31] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/issue-14500'
[01:40:31] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500  foo.rs --crate-type=rlib
[01:40:31] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500  bar.rs --crate-type=staticlib -C lto -L. -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500/libbar.a
[01:40:31] cc -ffunction-sections -fdata-sections -fPIC -m64 foo.c /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500/libbar.a -lm -lrt -ldl -lpthread -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500/foo
[01:40:31] Makefile:14: recipe for target 'all' failed
[01:40:31] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/issue-14500'
[01:40:31] ------------------------------------------
[01:40:31] stderr:
[01:40:31] ------------------------------------------
[01:40:31] ------------------------------------------
[01:40:31] warning: ignoring --out-dir flag due to -o flag
[01:40:31] 
[01:40:31] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-14500/issue-14500/libbar.a(compiler_builtins-ec024234a641382a.compiler_builtins.17giirf6-cgu.0.rcgu.o): In function `__clzsi2':
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0x5d): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0x8a): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0xa9): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0xb8): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] collect2: error: ld returned 1 exit status
[01:40:31] make[1]: *** [all] Error 1
[01:40:31] ------------------------------------------
[01:40:31] 
[01:40:31] thread '[run-make] run-make-fulldeps/issue-14500' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:40:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:40:31] status: exit code: 2
[01:40:31] command: "make"
[01:40:31] stdout:
[01:40:31] ------------------------------------------
[01:40:31] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/lto-smoke-c'
[01:40:31] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-smoke-c/lto-smoke-c:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-smoke-c/lto-smoke-c -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-smoke-c/lto-smoke-c  foo.rs -C lto
[01:40:31] cc -ffunction-sections -fdata-sections -fPIC -m64 bar.c /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-smoke-c/lto-smoke-c/libfoo.a \
[01:40:31]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-smoke-c/lto-smoke-c/bar \
[01:40:31]  -lm -lrt -ldl -lpthread -lstdc++
[01:40:31] Makefile:7: recipe for target 'all' failed
[01:40:31] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/lto-smoke-c'
[01:40:31] ------------------------------------------
[01:40:31] stderr:
[01:40:31] ------------------------------------------
[01:40:31] ------------------------------------------
[01:40:31] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-smoke-c/lto-smoke-c/libfoo.a(compiler_builtins-ec024234a641382a.compiler_builtins.17giirf6-cgu.0.rcgu.o): In function `__clzsi2':
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0x5d): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0x8a): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0xa9): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] compiler_builtins.17giirf6-cgu.0:(.text.__clzsi2+0xb8): undefined reference to `core::panicking::panic::h05784555a26cf2e4'
[01:40:31] collect2: error: ld returned 1 exit status
[01:40:31] make[1]: *** [all] Error 1
[01:40:31] ------------------------------------------
[01:40:31] 
[01:40:31] thread '[run-make] run-make-fulldeps/lto-smoke-c' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:40:31] 
---
[01:40:31] test result: FAILED. 188 passed; 2 failed; 5 ignored; 0 measured; 0 filtered out
[01:40:31] 
[01:40:31] 
[01:40:31] 
[01:40:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:40:31] 
[01:40:31] 
[01:40:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:31] Build completed unsuccessfully in 0:39:45
[01:40:31] Build completed unsuccessfully in 0:39:45
[01:40:31] make: *** [check] Error 1
[01:40:31] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02f042fd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 04:02:39 UTC 2019
