plain
travis_time:end:011394a2:start=1551993327115208044,finish=1551993329384618885,duration=2269410841
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:33:09] 
[01:33:09] running 119 tests
[01:33:39] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:33:44] i......iii.i.....ii
[01:33:44] 
[01:33:44]  finished in 34.433
[01:33:44] travis_fold:end:test_debuginfo

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:04:27] 
[02:04:27] running 195 tests
[02:05:00] ..................i...i.................................................................i........... 100/195
[02:05:46] ...................................................i..............................F.......i....
[02:05:46] 
[02:05:46] ---- [run-make] run-make-fulldeps/treat-err-as-bug stdout ----
[02:05:46] 
[02:05:46] error: make failed
[02:05:46] error: make failed
[02:05:46] status: exit code: 2
[02:05:46] command: "make"
[02:05:46] stdout:
[02:05:46] ------------------------------------------
[02:05:46] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/treat-err-as-bug'
[02:05:46] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/treat-err-as-bug/treat-err-as-bug:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/treat-err-as-bug/treat-err-as-bug -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/treat-err-as-bug/treat-err-as-bug  err.rs -Z treat-err-as-bug 2>&1 \
[02:05:46]     | "/checkout/src/etc/cat-and-grep.sh" "panicked at 'aborting due to \`-Z treat_err_as_bug=1\`'"
[02:05:46] [[[ begin stdout ]]]
[02:05:46] error[E0080]: could not evaluate static initializer
[02:05:46]  --> err.rs:3:21
[02:05:46]   |
[02:05:46] 3 | pub static C: u32 = 0-1;
[02:05:46]   |                     ^^^ attempt to subtract with overflow
[02:05:46] 
[02:05:46] thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:532:13
[02:05:46] 
[02:05:46] error: internal compiler error: unexpected panic
[02:05:46] 
[02:05:46] note: the compiler unexpectedly panicked. this is a bug.
---
[02:05:46] 
[02:05:46] note: compiler flags: -Z treat-err-as-bug
[02:05:46] 
[02:05:46] 
[02:05:46] [[[ end stdout ]]]
[02:05:46] Error: cannot match: panicked at 'aborting due to `-Z treat_err_as_bug=1`'
[02:05:46] Makefile:4: recipe for target 'all' failed
[02:05:46] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/treat-err-as-bug'
[02:05:46] ------------------------------------------
[02:05:46] stderr:
[02:05:46] ------------------------------------------
[02:05:46] ------------------------------------------
[02:05:46] make[1]: *** [all] Error 1
[02:05:46] ------------------------------------------
[02:05:46] 
[02:05:46] thread '[run-make] run-make-fulldeps/treat-err-as-bug' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[02:05:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[02:05:46] 
[02:05:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[02:05:46] 
[02:05:46] 
[02:05:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:05:46] 
[02:05:46] 
[02:05:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:05:46] Build completed unsuccessfully in 0:46:42
[02:05:46] Build completed unsuccessfully in 0:46:42
[02:05:46] Makefile:48: recipe for target 'check' failed
[02:05:46] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ddf56c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar  7 23:21:26 UTC 2019
---
travis_time:end:09410760:start=1552000889078120759,finish=1552000889083562074,duration=5441315
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:141e1332
$ ln -s . checkout && for CORE in obj
