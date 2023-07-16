plain
travis_time:end:02179482:start=1558393394901951467,finish=1558393397101869866,duration=2199918399
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:32] 
[01:18:32] running 143 tests
[01:18:35] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:18:36] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:18:36] 
[01:18:36]  finished in 4.550
[01:18:36] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:39] 
[01:18:39] running 9 tests
[01:18:39] iiiiiiiii
[01:18:39] 
[01:18:39]  finished in 0.153
[01:18:39] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:54] 
[01:18:54] running 122 tests
[01:19:18] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:19:23] .i.i......iii.i.....ii
[01:19:23] 
[01:19:23]  finished in 29.257
[01:19:23] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:41:11] 
[01:41:11] running 199 tests
[01:41:36] ..................i...i.......................................F........................i............ 100/199
[01:42:20] ...........................iiii........i..........iiii.iii....................................i....
[01:42:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:42:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:20] 
[01:42:20] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:42:20] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:42:20] 
[01:42:20] error: make failed
[01:42:20] status: exit code: 2
[01:42:20] command: "make"
[01:42:20] stdout:
[01:42:20] ------------------------------------------
[01:42:20] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:42:20] /bin/echo || exit 0 # This test requires /bin/echo to exist
[01:42:20] 
[01:42:20] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
[01:42:20]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
[01:42:20] Makefile:4: recipe for target 'all' failed
[01:42:20] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:42:20] ------------------------------------------
[01:42:20] stderr:
[01:42:20] ------------------------------------------
[01:42:20] ------------------------------------------
[01:42:20] warning: ignoring --out-dir flag due to -o flag
[01:42:20] 
[01:42:20] error[E0277]: the trait bound `&std::collections::HashMap<std::string::String, std::option::Option<syntax::ast::Name>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>: std::default::Default` is not satisfied
[01:42:20]   --> the_backend.rs:51:13
[01:42:20] 51 |             Default::default() // Just a dummy
[01:42:20] 51 |             Default::default() // Just a dummy
[01:42:20]    |             ^^^^^^^^^^^^^^^^ the trait `std::default::Default` is not implemented for `&std::collections::HashMap<std::string::String, std::option::Option<syntax::ast::Name>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[01:42:20]    = help: the following implementations were found:
[01:42:20]              <std::collections::HashMap<K, V, S> as std::default::Default>
[01:42:20]    = note: required by `std::default::Default::default`
[01:42:20] 
[01:42:20] 
[01:42:20] error: aborting due to previous error
[01:42:20] 
[01:42:20] For more information about this error, try `rustc --explain E0277`.
[01:42:20] make[1]: *** [all] Error 1
[01:42:20] ------------------------------------------
[01:42:20] 
[01:42:20] 
[01:42:20] 
[01:42:20] 
[01:42:20] failures:
[01:42:20]     [run-make] run-make-fulldeps/hotplug_codegen_backend
[01:42:20] 
[01:42:20] test result: FAILED. 182 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out
[01:42:20] 
[01:42:20] 
[01:42:20] 
[01:42:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:20] 
[01:42:20] 
[01:42:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:20] Build completed unsuccessfully in 0:35:21
[01:42:20] Build completed unsuccessfully in 0:35:21
[01:42:20] make: *** [check] Error 1
[01:42:20] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:037736f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 21 00:45:50 UTC 2019
---
travis_time:end:098fb560:start=1558399552285861906,finish=1558399552290712170,duration=4850264
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15c973fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0446f916
travis_time:start:0446f916
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c9e58ac
$ dmesg | grep -i kill
