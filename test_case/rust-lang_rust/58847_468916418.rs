plain
travis_time:end:0079a2e0:start=1551523585453596620,finish=1551523665999878472,duration=80546281852
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
[01:15:00] 
[01:15:00] running 119 tests
[01:15:25] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:15:29] i......iii.i.....ii
[01:15:29] 
[01:15:29]  finished in 29.586
[01:15:29] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:42:00] 
[01:42:00] running 195 tests
[01:42:26] ..................i...i.......................................F.........................i........... 100/195
[01:43:11] ...................................................i......................................i....
[01:43:11] failures:
[01:43:11] 
[01:43:11] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:43:11] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:43:11] 
[01:43:11] error: make failed
[01:43:11] status: exit code: 2
[01:43:11] command: "make"
[01:43:11] stdout:
[01:43:11] ------------------------------------------
[01:43:11] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:43:11] /bin/echo || exit 0 # This test requires /bin/echo to exist
[01:43:11] 
[01:43:11] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
[01:43:11]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
[01:43:11] Makefile:4: recipe for target 'all' failed
[01:43:11] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:43:11] ------------------------------------------
[01:43:11] stderr:
[01:43:11] ------------------------------------------
[01:43:11] ------------------------------------------
[01:43:11] warning: ignoring --out-dir flag due to -o flag
[01:43:11] 
[01:43:11] error[E0432]: unresolved import `rustc_codegen_utils::codegen_backend::MetadataOnlyCodegenBackend`
[01:43:11]   --> the_backend.rs:16:60
[01:43:11]    |
[01:43:11] 16 | use rustc_codegen_utils::codegen_backend::{CodegenBackend, MetadataOnlyCodegenBackend};
[01:43:11]    |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MetadataOnlyCodegenBackend` in `codegen_backend`
[01:43:11] error: aborting due to previous error
[01:43:11] 
[01:43:11] For more information about this error, try `rustc --explain E0432`.
[01:43:11] For more information about this error, try `rustc --explain E0432`.
[01:43:11] make[1]: *** [all] Error 1
[01:43:11] ------------------------------------------
[01:43:11] 
[01:43:11] thread '[run-make] run-make-fulldeps/hotplug_codegen_backend' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:43:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:43:11] test result: FAILED. 189 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:43:11] 
[01:43:11] 
[01:43:11] 
[01:43:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:43:11] 
[01:43:11] 
[01:43:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:11] Build completed unsuccessfully in 0:39:56
[01:43:11] Build completed unsuccessfully in 0:39:56
[01:43:11] make: *** [check] Error 1
[01:43:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1216583c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  2 12:31:06 UTC 2019
---
travis_time:end:08a25ba4:start=1551529868228305917,finish=1551529868290210775,duration=61904858
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09791d6c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04bdea67
$ dmesg | grep -i kill
