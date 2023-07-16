plain
travis_time:end:1c532bfe:start=1553281555602410990,finish=1553281628533568117,duration=72931157127
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
[01:20:39] 
[01:20:39] running 9 tests
[01:20:39] iiiiiiiii
[01:20:39] 
[01:20:39]  finished in 0.150
[01:20:39] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:54] 
[01:20:54] running 120 tests
[01:21:19] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:23] .i......iii.i.....ii
[01:21:23] 
[01:21:23]  finished in 28.590
[01:21:23] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:47:13] 
[01:47:13] running 196 tests
[01:47:40] .........F........i...i.................................................................i........... 100/196
[01:48:25] ....................................................i......................................i....
[01:48:25] failures:
[01:48:25] 
[01:48:25] ---- [run-make] run-make-fulldeps/c-link-to-rust-va-list-fn stdout ----
[01:48:25] ---- [run-make] run-make-fulldeps/c-link-to-rust-va-list-fn stdout ----
[01:48:25] 
[01:48:25] error: make failed
[01:48:25] status: exit code: 2
[01:48:25] command: "make"
[01:48:25] stdout:
[01:48:25] ------------------------------------------
[01:48:25] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/c-link-to-rust-va-list-fn'
[01:48:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/c-link-to-rust-va-list-fn/c-link-to-rust-va-list-fn:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/c-link-to-rust-va-list-fn/c-link-to-rust-va-list-fn -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/c-link-to-rust-va-list-fn/c-link-to-rust-va-list-fn  checkrust.rs
[01:48:25] Makefile:4: recipe for target 'all' failed
[01:48:25] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/c-link-to-rust-va-list-fn'
[01:48:25] ------------------------------------------
[01:48:25] stderr:
[01:48:25] ------------------------------------------
[01:48:25] error[E0599]: no method named `copy` found for type `std::ffi::VaList<'_>` in the current scope
[01:48:25] error[E0599]: no method named `copy` found for type `std::ffi::VaList<'_>` in the current scope
[01:48:25]   --> checkrust.rs:65:8
[01:48:25]    |
[01:48:25] 65 |     ap.copy(|mut ap| {
[01:48:25] 
[01:48:25] error: aborting due to previous error
[01:48:25] 
[01:48:25] For more information about this error, try `rustc --explain E0599`.
[01:48:25] For more information about this error, try `rustc --explain E0599`.
[01:48:25] make[1]: *** [all] Error 1
[01:48:25] ------------------------------------------
[01:48:25] 
[01:48:25] thread '[run-make] run-make-fulldeps/c-link-to-rust-va-list-fn' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:48:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:48:25] test result: FAILED. 190 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:48:25] 
[01:48:25] 
[01:48:25] 
[01:48:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:48:25] 
[01:48:25] 
[01:48:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:25] Build completed unsuccessfully in 0:39:19
[01:48:25] Build completed unsuccessfully in 0:39:19
[01:48:25] make: *** [check] Error 1
[01:48:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003cad14
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 22 20:55:43 UTC 2019
---
travis_time:end:09ccc969:start=1553288144585358979,finish=1553288144643799924,duration=58440945
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06f3cf2c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:259c936c
$ dmesg | grep -i kill
