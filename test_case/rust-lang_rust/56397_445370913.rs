plain
travis_time:end:033b6e2c:start=1544213093234157703,finish=1544213148633480583,duration=55399322880
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:03] 
[00:53:03] running 120 tests
[00:53:06] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:53:07] ..ii.i.....iiii.....
[00:53:07] 
[00:53:07]  finished in 3.598
[00:53:07] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:21] 
[00:53:21] running 118 tests
[00:53:44] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:53:48] ......iii.i.....ii
[00:53:48] 
[00:53:48]  finished in 26.928
[00:53:48] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:27] 
[01:18:27] running 194 tests
[01:18:50] .......................................................................................F............ 100/194
[01:19:48] .............................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:22:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:22:31] failures:
[01:22:31] 
[01:22:31] ---- [run-make] run-make-fulldeps/libs-search-path stdout ----
---
[01:22:31] 
[01:22:31] ------------------------------------------
[01:22:31] stderr:
[01:22:31] ------------------------------------------
[01:22:31] make[1]: *** No targets.  Stop.
[01:22:31] ------------------------------------------
[01:22:31] 
[01:22:31] 
[01:22:31] thread '[run-make] run-make-fulldeps/libs-search-path' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[01:22:31] 
[01:22:31] 
[01:22:31] failures:
[01:22:31]     [run-make] run-make-fulldeps/libs-search-path
[01:22:31]     [run-make] run-make-fulldeps/libs-search-path
[01:22:31] 
[01:22:31] test result: FAILED. 193 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:22:31] 
[01:22:31] 
[01:22:31] 
[01:22:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gngondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "nknown-linux-gnu
58684 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps
56896 ./src/llvm/test/MC
56648 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
56108 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build
---
travis_time:end:084cf5eb:start=1544218109914566994,finish=1544218109921618017,duration=7051023
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08b962db
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 
