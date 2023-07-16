plain
travis_time:end:0255f9b0:start=1543863384381938093,finish=1543863472704537416,duration=88322599323
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
[00:55:53] 
[00:55:53] running 120 tests
[00:55:56] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii...............i..ii 100/120
[00:55:56] ..ii.i.....iiii.....
[00:55:56] 
[00:55:56]  finished in 3.326
[00:55:56] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:10] 
[00:56:10] running 118 tests
[00:56:33] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:37] ......iii.i.....ii
[00:56:37] 
[00:56:37]  finished in 26.939
[00:56:37] travis_fold:end:test_debuginfo

---
[01:24:24] verifying if humantime is an unstable crate
[01:24:24] verifying if test is an unstable crate
[01:24:24] verifying if termcolor is an unstable crate
[01:24:24] verifying if parking_lot_core is an unstable crate
[01:24:24] verifying if rustc_hash is an unstable crate
[01:24:24] verifying if backtrace_sys is an unstable crate
[01:24:24] verifying if compiler_builtins is an unstable crate
[01:24:24] verifying if cc is an unstable crate
[01:24:24] verifying if quote is an unstable crate
[01:24:24] verifying if rand_core is an unstable crate
[01:24:24] verifying if memoffset is an unstable crate
[01:24:24] verifying if ena is an unstable crate
[01:24:24] verifying if proc_macro2 is an unstable crate
[01:24:24] verifying if num_cpus is an unstable crate
[01:24:24] verifying if crossbeam_utils is an unstable crate
[01:24:24] verifying if rustc_demangle is an unstable crate
[01:24:24] verifying if scoped_tls is an unstable crate
[01:24:24] verifying if crossbeam_deque is an unstable crate
[01:24:24] verifying if datafrog is an unstable crate
[01:24:24] verifying if rustc_serialize is an unstable crate
[01:24:24] verifying if rustc_msan is an unstable crate
[01:24:24] verifying if libc is an unstable crate
[01:24:24] verifying if panic_abort is an unstable crate
[01:24:24] verifying if log_settings is an unstable crate
[01:24:24] verifying if rls_data is an unstable crate
[01:24:24] verifying if memmap is an unstable crate
[01:24:24] verifying if build_helper is an unstable crate
[01:24:24] verifying if build_helper is an unstable crate
[01:24:24] crate build_helper "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbuild_helper-fc70adc6582f8056.rlib" is not unstable
[01:24:24] error[E0514]: found crate `build_helper` compiled by an incompatible version of rustc
[01:24:24]  --> <anon>:1:1
[01:24:24] 1 | extern crate build_helper;
[01:24:24]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:24:24]   |
[01:24:24]   |
[01:24:24]   = help: please recompile that crate using this compiler (rustc 1.32.0-dev)
[01:24:24]   = note: the following crate versions were found:
[01:24:24]      g if panic_unwind is an unstable crate
[01:24:24] verifying if jobserver is an unstable crate
[01:24:24] verifying if atty is an unstable crate
[01:24:24] verifying if atty is an unstable crate
[01:24:24] verifying if lock_api is an unstable crate
[01:24:24] verifying if cmake is an unstable crate
[01:24:24] crate cmake "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcmake-afb19a8695d09bc4.rlib" is not unstable
[01:24:24] error[E0514]: found crate `cmake` compiled by an incompatible version of rustc
[01:24:24]  --> <anon>:1:1
[01:24:24] 1 | extern crate cmake;
[01:24:24]   | ^^^^^^^^^^^^^^^^^^^
[01:24:24]   |
[01:24:24]   |
[01:24:24]   = help: please recompile that crate using this compiler (rustc 1.32.0-dev)
[01:24:24]   = note: the following crate versions were found:
[01:24:24]           crate `cmake` compiled by rustc 1.31.0-beta.1 (2824a67b0 2018-10-29): /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcmake-afb19a8695d09bc4.rlib
[01:24:24] error: aborting due to previous error
[01:24:24] 
[01:24:24] For more information about this error, try `rustc --explain E0514`.
[01:24:24] 
---
[01:24:24] 
[01:24:24] ------------------------------------------
[01:24:24] stderr:
[01:24:24] ------------------------------------------
[01:24:24] make[1]: *** [all] Error 1
[01:24:24] ------------------------------------------
[01:24:24] 
[01:24:24] thread '[run-make] run-make-fulldeps/sysroot-crates-are-unstable' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:24:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:24:24] test result: FAILED. 191 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:24:24] 
[01:24:24] 
[01:24:24] 
[01:24:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:24:24] 
[01:24:24] 
[01:24:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:24] Build completed unsuccessfully in 0:39:06
[01:24:24] Build completed unsuccessfully in 0:39:06
[01:24:24] Makefile:58: recipe for target 'check' failed
[01:24:24] make: *** [check] Error 1
travis_time:end:0e907c58:start=1543868546047406012,finish=1543868546347785603,duration=300379591
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
