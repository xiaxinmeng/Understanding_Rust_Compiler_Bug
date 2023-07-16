plain
travis_time:end:07ab4304:start=1553635445536095868,finish=1553635557887124155,duration=112351028287
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:34] 
[01:24:34] running 9 tests
[01:24:34] iiiiiiiii
[01:24:34] 
[01:24:34]  finished in 0.161
[01:24:34] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:50] 
[01:24:50] running 120 tests
[01:25:17] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:25:22] .i......iii.i.....ii
[01:25:22] 
[01:25:22]  finished in 31.841
[01:25:22] travis_fold:end:test_debuginfo

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:53:32] 
[01:53:32] running 195 tests
[01:54:00] ..................i...i................................................................i............ 100/195
[01:54:46] ...........................................F.......i......................................i....
[01:54:46] 
[01:54:46] ---- [run-make] run-make-fulldeps/rustdoc-error-lines stdout ----
[01:54:46] 
[01:54:46] error: make failed
[01:54:46] error: make failed
[01:54:46] status: exit code: 2
[01:54:46] command: "make"
[01:54:46] stdout:
[01:54:46] ------------------------------------------
[01:54:46] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/rustdoc-error-lines'
[01:54:46] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --test input.rs > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output || true
[01:54:46] "/checkout/src/etc/cat-and-grep.sh" 'input.rs:7:15' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
[01:54:46] [[[ begin stdout ]]]
[01:54:46] running 1 test
[01:54:46] test input.rs - foo (line 6) ... FAILED
[01:54:46] 
[01:54:46] failures:
---
[01:54:46] 
[01:54:46] error: aborting due to previous error
[01:54:46] 
[01:54:46] For more information about this error, try `rustc --explain E0308`.
[01:54:46] thread 'input.rs - foo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:54:46] 
[01:54:46] 
[01:54:46] failures:
[01:54:46]     input.rs - foo (line 6)
[01:54:46]     input.rs - foo (line 6)
[01:54:46] 
[01:54:46] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:54:46] 
[01:54:46] 
[01:54:46] [[[ end stdout ]]]
[01:54:46] Error: cannot match: input.rs:7:15
[01:54:46] Makefile:7: recipe for target 'all' failed
[01:54:46] 
[01:54:46] ------------------------------------------
[01:54:46] stderr:
[01:54:46] ------------------------------------------
[01:54:46] ------------------------------------------
[01:54:46] make[1]: *** [all] Error 1
[01:54:46] ------------------------------------------
[01:54:46] 
[01:54:46] thread '[run-make] run-make-fulldeps/rustdoc-error-lines' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:54:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:54:46] test result: FAILED. 189 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:54:46] 
[01:54:46] 
[01:54:46] 
[01:54:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:54:46] 
[01:54:46] 
[01:54:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:54:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:54:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:54:46] Build completed unsuccessfully in 0:42:21
[01:54:46] make: *** [check] Error 1
[01:54:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bbb7205
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 23:20:53 UTC 2019
---
travis_time:end:108e9b92:start=1553642455450145067,finish=1553642455457642399,duration=7497332
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:210eb58e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e55e27
travis_time:start:09e55e27
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

