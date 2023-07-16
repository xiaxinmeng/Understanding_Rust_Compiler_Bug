plain
travis_time:end:25458a90:start=1554452944580814977,finish=1554453021813824607,duration=77233009630
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:11] 
######################################################################## 100.0%
[00:01:11] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:11]     Updating crates.io index
[00:01:26]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:01:27]   Downloaded toml v0.4.10
[00:01:27]   Downloaded cmake v0.1.33
[00:01:27]   Downloaded serde_derive v1.0.81
[00:01:27]   Downloaded cc v1.0.28
---
tidy check
[00:03:21] * 569 error codes
[00:03:21] * highest error code: E0725
[00:03:22] * 252 features
[00:03:23] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[00:03:23] travis_time:end:tidy:start=1554453233027337074,finish=1554453235051631246,duration=2024294172

[00:03:23] Build completed successfully in 0:00:46
---
[00:04:22]    Compiling libc v0.2.51
[00:04:22]    Compiling getopts v0.2.17
[00:04:22]    Compiling termcolor v1.0.4
[00:04:22]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:04:24]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:04:34]     Finished release [optimized] target(s) in 12.74s
[00:04:34] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:34] travis_fold:end:stage0-test

---
[00:25:10]    Compiling libc v0.2.51
[00:25:10]    Compiling termcolor v1.0.4
[00:25:10]    Compiling getopts v0.2.17
[00:25:10]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:25:12]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:25:25]     Finished release [optimized] target(s) in 16.03s
[00:25:25] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:25:25] travis_fold:end:stage1-test

---
[01:00:27]     Checking termcolor v1.0.4
[01:00:27]     Checking getopts v0.2.17
[01:00:27]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:00:27]     Checking libc v0.2.51
[01:00:28]     Checking libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[01:00:33]     Finished release [optimized] target(s) in 6.94s
[01:00:33] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[01:00:34]  Documenting proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:00:38]     Finished release [optimized] target(s) in 4.30s
---
tidy check
[01:03:16] * 569 error codes
[01:03:16] * highest error code: E0725
[01:03:16] * 252 features
[01:03:17] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[01:03:17] travis_time:end:tidy:start=1554456826912236893,finish=1554456829179734564,duration=2267497671

[01:03:17] travis_fold:start:stage0-std
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:07] 
[01:16:07] running 9 tests
[01:16:07] iiiiiiiii
[01:16:07] 
[01:16:07]  finished in 0.165
[01:16:07] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:25] 
[01:16:25] running 121 tests
[01:16:54] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:17:00] i.i......iii.i.....ii
[01:17:00] 
[01:17:00]  finished in 34.963
[01:17:00] travis_fold:end:test_debuginfo

---
[01:47:25] command: "make"
[01:47:25] stdout:
[01:47:25] ------------------------------------------
[01:47:25] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/sysroot-crates-are-unstable'
[01:47:25] python2.7 test.py
[01:47:25] verifying if atty is an unstable crate
[01:47:25] verifying if rustc_hash is an unstable crate
[01:47:25] verifying if rustc_apfloat is an unstable crate
[01:47:25] verifying if termcolor is an unstable crate
[01:47:25] crate termcolor "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtermcolor-a06e3f972506e90c.rlib" is not unstable
[01:47:25] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:47:25] error: aborting due to previous error
[01:47:25] 
[01:47:25] 
[01:47:25] 
[01:47:25] 
[01:47:25] verifying if getopts is an unstable crate
[01:47:25] crate getopts "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-95c33802fcaebdd9.rlib" is not unstable
[01:47:25] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:47:25] error: aborting due to previous error
[01:47:25] 
[01:47:25] 
[01:47:25] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1552: worker thread panicked', src/librustc/util/bug.rs:37:26
[01:47:25] 
[01:47:25] 
[01:47:25] verifying if rand_xorshift is an unstable crate
[01:47:25] verifying if unreachable is an unstable crate
---
[01:47:25] verifying if log_settings is an unstable crate
[01:47:25] verifying if parking_lot is an unstable crate
[01:47:25] verifying if alloc is an unstable crate
[01:47:25] verifying if polonius_engine is an unstable crate
[01:47:25] verifying if datafrog is an unstable crate
[01:47:25] verifying if crossbeam_epoch is an unstable crate
[01:47:25] verifying if libc is an unstable crate
[01:47:25] crate libc "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-669981f0fa39ea2c.rlib" is not unstable
[01:47:25] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:47:25] error: aborting due to previous error
[01:47:25] 
[01:47:25] 
[01:47:25] 
---
[01:47:25] 
[01:47:25] ------------------------------------------
[01:47:25] stderr:
[01:47:25] ------------------------------------------
[01:47:25] make[1]: *** [all] Error 1
[01:47:25] ------------------------------------------
[01:47:25] 
[01:47:25] thread '[run-make] run-make-fulldeps/sysroot-crates-are-unstable' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:47:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:47:25] test result: FAILED. 189 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:47:25] 
[01:47:25] 
[01:47:25] 
[01:47:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:47:25] 
[01:47:25] 
[01:47:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:25] Build completed unsuccessfully in 0:44:13
[01:47:25] Build completed unsuccessfully in 0:44:13
[01:47:25] Makefile:48: recipe for target 'check' failed
[01:47:25] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17ba06ba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr  5 10:17:57 UTC 2019
