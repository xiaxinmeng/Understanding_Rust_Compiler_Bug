plain
travis_time:end:018b87af:start=1543168873127460606,finish=1543168874279320257,duration=1151859651
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:32] .................................................................................................... 100/5053
[00:50:35] .................................................................................................... 200/5053
[00:50:38] .............................ii............................................ii...................ii.. 300/5053
[00:50:41] ..............................................................................................iii... 400/5053
[00:50:44] .....iiiiiiii.iii............................iii...........................................i........ 500/5053
[00:50:51] .................................................................................................... 700/5053
[00:50:56] .....................................................................................i...........i.. 800/5053
[00:51:00] .................................................................................................... 900/5053
[00:51:03] ....iiiii..................ii.iiii.................................................................. 1000/5053
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:01] 
[01:06:01] running 120 tests
[01:06:04] i..ii...iii..iiii.....i...i.........i..iii...........i.....i........ii...i..i.ii..............i...ii 100/120
[01:06:05] ..ii.i.....i.iii....
[01:06:05] 
[01:06:05]  finished in 3.534
[01:06:05] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:19] 
[01:06:19] running 118 tests
[01:06:44] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:48] ......iii.i.....ii
[01:06:48] 
[01:06:48]  finished in 28.524
[01:06:48] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:39:23] 
[01:39:23] running 192 tests
[01:39:50] .....................................................F.............................................. 100/192
[01:40:46] ...........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:41:34] failures:
[01:41:34] 
[01:41:34] ---- [run-make] run-make-fulldeps/inline-always-many-cgu stdout ----
[01:41:34] 
[01:41:34] 
[01:41:34] error: make failed
[01:41:34] status: exit code: 2
[01:41:34] command: "make"
[01:41:34] stdout:
[01:41:34] ------------------------------------------
[01:41:34] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/inline-always-many-cgu'
[01:41:34] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/inline-always-many-cgu/inline-always-many-cgu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/inline-always-many-cgu/inline-always-many-cgu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/inline-always-many-cgu/inline-always-many-cgu  foo.rs --emit llvm-ir -C codegen-units=2
[01:41:34] if cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/inline-always-many-cgu/inline-always-many-cgu/*.ll | "/checkout/src/etc/cat-and-grep.sh" -e '\bcall\b'; then \
[01:41:34]  echo "found call instruction when one wasn't expected"; \
[01:41:34]  exit 1; \
[01:41:34] fi
[01:41:34] [[[ begin stdout ]]]
[01:41:34] ; ModuleID = 'foo.3a1fbbbh-cgu.0'
[01:41:34] source_filename = "foo.3a1fbbbh-cgu.0"
[01:41:34] target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
[01:41:34] target triple = "x86_64-unknown-linux-gnu"
[01:41:34] 
[01:41:34] ; foo::a::bar
[01:41:34] ; Function Attrs: noinline nonlazybind optnone uwtable
[01:41:34] define void @_ZN3foo1a3bar17h93d7ab41e03a35d6E() unnamed_addr #0 {
[01:41:34]   ret void
[01:41:34] }
[01:41:34] 
[01:41:34] 
[01:41:34] attributes #0 = { noinline nonlazybind optnone uwtable "probe-stack"="__rust_probestack" }
[01:41:34] 
[01:41:34] !llvm.module.flags = !{!0}
[01:41:34] 
[01:41:34] !0 = !{i32 2, !"RtLibUseGOT", i32 1}
[01:41:34] ; ModuleID = 'foo.3a1fbbbh-cgu.1'
[01:41:34] source_filename = "foo.3a1fbbbh-cgu.1"
[01:41:34] target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
[01:41:34] target triple = "x86_64-unknown-linux-gnu"
[01:41:34] 
[01:41:34] ; foo::a::foo
[01:41:34] ; Function Attrs: noinline nonlazybind optnone uwtable
[01:41:34] define internal void @_ZN3foo1a3foo17h4934260c56dc6ae5E() unnamed_addr #0 {
[01:41:34]   ret void
[01:41:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:41:34] }
[01:41:34] 
[01:41:34] 
[01:41:34] ; Function Attrs: noinline nonlazybind optnone uwtable
[01:41:34] define void @bar() unnamed_addr #0 {
[01:41:34] start:
[01:41:34] ; call foo::a::foo
[01:41:34]   call void @_ZN3foo1a3foo17h4934260c56dc6ae5E()
[01:41:34]   br label %bb1
[01:41:34] 
[01:41:34] bb1:                                              ; preds = %start
[01:41:34]   ret void
[01:41:34] }
[01:41:34] 
[01:41:34] attributes #0 = { noinline nonlazybind optnone uwtable "probe-stack"="__rust_probestack" }
[01:41:34] 
[01:41:34] !llvm.module.flags = !{!0}
[01:41:34] 
[01:41:34] !0 = !{i32 2, !"RtLibUseGOT", i32 1}
[01:41:34] 
[01:41:34] [[[ end stdout ]]]
[01:41:34] found call instruction when one wasn't expected
[01:41:34] Makefile:4: recipe for target 'all' failed
[01:41:34] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/inline-always-many-cgu'
[01:41:34] ------------------------------------------
[01:41:34] stderr:
[01:41:34] ------------------------------------------
[01:41:34] ------------------------------------------
[01:41:34] warning: ignoring emit path because multiple .ll files were produced
[01:41:34] 
[01:41:34] make[1]: *** [all] Error 1
[01:41:34] ------------------------------------------
[01:41:34] 
[01:41:34] thread '[run-make] run-make-fulldeps/inline-always-many-cgu' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3324:9
[01:41:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:41:34] test result: FAILED. 191 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:41:34] 
[01:41:34] 
[01:41:34] 
[01:41:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:41:34] 
[01:41:34] 
[01:41:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:34] Build completed unsuccessfully in 0:54:54
[01:41:34] Build completed unsuccessfully in 0:54:54
[01:41:34] Makefile:58: recipe for target 'check' failed
[01:41:34] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01bec35c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 25 19:42:57 UTC 2018
---
travis_time:end:03323db1:start=1543174983141873234,finish=1543174983149887054,duration=8013820
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f711d4d
$ ln -s . checkout && for CORE in obj/cores/core.*;
