plain
travis_time:end:2a701e38:start=1554475773678273235,finish=1554475878697332313,duration=105019059078
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
####################################################################      94.7%
######################################################################## 100.0%
[00:03:11] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:11]     Updating crates.io index
[00:03:26]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:03:27]   Downloaded toml v0.4.10
[00:03:27]   Downloaded serde_json v1.0.33
[00:03:27]   Downloaded serde v1.0.82
[00:03:27]   Downloaded cc v1.0.28
---
tidy check
[00:05:23] * 569 error codes
[00:05:23] * highest error code: E0725
[00:05:23] * 252 features
[00:05:24] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[00:05:24] travis_time:end:tidy:start=1554476211336352454,finish=1554476213402327590,duration=2065975136

[00:05:24] Build completed successfully in 0:00:48
---
[00:06:26]    Compiling libc v0.2.51
[00:06:26]    Compiling termcolor v1.0.4
[00:06:26]    Compiling getopts v0.2.17
[00:06:26]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:06:28]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:06:39]     Finished release [optimized] target(s) in 13.75s
[00:06:39] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:06:39] travis_fold:end:stage0-test

---
[00:27:34]    Compiling libc v0.2.51
[00:27:34]    Compiling getopts v0.2.17
[00:27:34]    Compiling termcolor v1.0.4
[00:27:34]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:27:37]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:27:50]     Finished release [optimized] target(s) in 16.54s
[00:27:50] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:27:50] travis_fold:end:stage1-test

---
[01:03:24]     Checking getopts v0.2.17
[01:03:24]     Checking termcolor v1.0.4
[01:03:24]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:03:24]     Checking libc v0.2.51
[01:03:26]     Checking libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[01:03:31]     Finished release [optimized] target(s) in 7.13s
[01:03:31] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[01:03:32]  Documenting proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:03:36]     Finished release [optimized] target(s) in 4.32s
---
tidy check
[01:06:14] * 569 error codes
[01:06:14] * highest error code: E0725
[01:06:14] * 252 features
[01:06:15] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[01:06:15] travis_time:end:tidy:start=1554479862061286436,finish=1554479864275367708,duration=2214081272

[01:06:15] travis_fold:start:stage0-std
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:22] 
[01:19:22] running 9 tests
[01:19:22] iiiiiiiii
[01:19:22] 
[01:19:22]  finished in 0.172
[01:19:22] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:40] 
[01:19:40] running 121 tests
[01:20:10] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:15] i.i......iii.i.....ii
[01:20:15] 
[01:20:15]  finished in 35.676
[01:20:15] travis_fold:end:test_debuginfo

---
[01:50:36] error: failed to write bytecode: Read-only file system (os error 30)
[01:50:36] 
[01:50:36] error: aborting due to previous error
[01:50:36] 
[01:50:36] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1552: worker thread panicked', src/librustc/util/bug.rs:37:26
[01:50:36] 
[01:50:36] 
[01:50:36] verifying if getopts is an unstable crate
[01:50:36] verifying if getopts is an unstable crate
[01:50:36] crate getopts "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-95c33802fcaebdd9.rlib" is not unstable
[01:50:36] error: failed to write bytecode: Read-only file system (os error 30)
[01:50:36] error: aborting due to previous error
[01:50:36] 
[01:50:36] 
[01:50:36] 
---
[01:50:36] verifying if remove_dir_all is an unstable crate
[01:50:36] verifying if unicode_width is an unstable crate
[01:50:36] verifying if rand_pcg is an unstable crate
[01:50:36] verifying if libtest is an unstable crate
[01:50:36] crate libtest "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibtest-3c875ea7808ea8d5.rlib" is not unstable
[01:50:36] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:50:36] error: aborting due to previous error
[01:50:36] 
[01:50:36] 
[01:50:36] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1552: worker thread panicked', src/librustc/util/bug.rs:37:26
[01:50:36] 
[01:50:36] 
[01:50:36] verifying if serialize is an unstable crate
[01:50:36] verifying if rustc_lsan is an unstable crate
---
[01:50:36] 
[01:50:36] ------------------------------------------
[01:50:36] stderr:
[01:50:36] ------------------------------------------
[01:50:36] make[1]: *** [all] Error 1
[01:50:36] ------------------------------------------
[01:50:36] 
[01:50:36] thread '[run-make] run-make-fulldeps/sysroot-crates-are-unstable' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:50:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:50:36] test result: FAILED. 189 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:50:36] 
[01:50:36] 
[01:50:36] 
[01:50:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:50:36] 
[01:50:36] 
[01:50:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:50:36] Build completed unsuccessfully in 0:44:25
[01:50:36] Build completed unsuccessfully in 0:44:25
[01:50:36] Makefile:48: recipe for target 'check' failed
[01:50:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:140663d0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr  5 16:42:05 UTC 2019
---
travis_time:end:1ffb5cdd:start=1554482527403515339,finish=1554482527478980182,duration=75464843
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:087db06c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0692f3c8
$ dmesg | grep -i kill
