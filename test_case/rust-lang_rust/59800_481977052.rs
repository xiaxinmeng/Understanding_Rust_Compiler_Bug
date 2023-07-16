plain
travis_time:end:0c64e702:start=1554955919892886432,finish=1554955922122647095,duration=2229760663
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:50] 
[01:14:50] running 9 tests
[01:14:50] iiiiiiiii
[01:14:50] 
[01:14:50]  finished in 0.153
[01:14:50] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:05] 
[01:15:05] running 121 tests
[01:15:30] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:35] i.i......iii.i.....ii
[01:15:35] 
[01:15:35]  finished in 29.614
[01:15:35] travis_fold:end:test_debuginfo

---
[01:47:08] 
[01:47:08] running 196 tests
[01:47:39] ..................i...i................................................................i............ 100/196
[01:48:22] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:48:22] .......F............................................i......................................i....
[01:48:22] 
[01:48:22] ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
[01:48:22] 
[01:48:22] error: make failed
[01:48:22] error: make failed
[01:48:22] status: exit code: 2
[01:48:22] command: "make"
[01:48:22] stdout:
[01:48:22] ------------------------------------------
[01:48:22] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/issue-19371'
[01:48:22] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
[01:48:22] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371/foo /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 
[01:48:22] Makefile:8: recipe for target 'all' failed
[01:48:22] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/issue-19371'
[01:48:22] ------------------------------------------
[01:48:22] stderr:
[01:48:22] ------------------------------------------
[01:48:22] error: failed to find a `codegen-backends` folder in the sysroot candidates:
[01:48:22] error: failed to find a `codegen-backends` folder in the sysroot candidates:
[01:48:22] * /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371
[01:48:22] * /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371
[01:48:22] 
[01:48:22] make[1]: *** [all] Error 101
[01:48:22] ------------------------------------------
[01:48:22] 
[01:48:22] thread '[run-make] run-make-fulldeps/issue-19371' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:48:22] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:48:22] test result: FAILED. 190 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:48:22] 
[01:48:22] 
[01:48:22] 
[01:48:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:48:22] 
[01:48:22] 
[01:48:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:22] Build completed unsuccessfully in 0:44:36
[01:48:22] Build completed unsuccessfully in 0:44:36
[01:48:22] make: *** [check] Error 1
[01:48:22] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c1d55ff
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 06:00:35 UTC 2019
---
travis_time:end:309cfc90:start=1554962437532342265,finish=1554962437538370609,duration=6028344
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20ee74b5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3444eb28
travis_time:start:3444eb28
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1063bbc7
$ dmesg | grep -i kill
