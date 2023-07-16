plain
travis_time:end:239673c7:start=1544115499386530081,finish=1544115500441897697,duration=1055367616
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
[00:53:12] 
[00:53:12] running 120 tests
[00:53:15] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii...............i..ii 100/120
[00:53:15] ..ii.i.....iiii.....
[00:53:15] 
[00:53:15]  finished in 3.241
[00:53:15] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:29] 
[00:53:29] running 118 tests
[00:53:51] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:53:54] ......iii.i.....ii
[00:53:54] 
[00:53:54]  finished in 26.069
[00:53:54] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:54] 
[01:17:54] running 193 tests
[01:18:17] ..............................................................F..................................... 100/193
[01:19:15] ............................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:21:55] failures:
[01:21:55] 
[01:21:55] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:21:55] 
[01:21:55] 
[01:21:55] error: make failed
[01:21:55] status: exit code: 2
[01:21:55] command: "make"
[01:21:55] stdout:
[01:21:55] ------------------------------------------
[01:21:55] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:21:55] /bin/echo || exit 0 # This test requires /bin/echo to exist
[01:21:55] 
[01:21:55] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
[01:21:55]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
[01:21:55] Makefile:4: recipe for target 'all' failed
[01:21:55] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:21:55] ------------------------------------------
[01:21:55] stderr:
[01:21:55] ------------------------------------------
[01:21:55] ------------------------------------------
[01:21:55] warning: ignoring --out-dir flag due to -o flag
[01:21:55] 
[01:21:55] error[E0599]: no function or associated item named `new` found for type `rustc_codegen_utils::codegen_backend::MetadataOnlyCodegenBackend` in the current scope
[01:21:55]   --> the_backend.rs:81:25
[01:21:55]    |
[01:21:55] 81 |     Box::new(TheBackend(MetadataOnlyCodegenBackend::new()))
[01:21:55]    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `rustc_codegen_utils::codegen_backend::MetadataOnlyCodegenBackend`
[01:21:55] error: aborting due to previous error
[01:21:55] 
[01:21:55] For more information about this error, try `rustc --explain E0599`.
[01:21:55] For more information about this error, try `rustc --explain E0599`.
[01:21:55] make[1]: *** [all] Error 1
[01:21:55] ------------------------------------------
[01:21:55] 
[01:21:55] thread '[run-make] run-make-fulldeps/hotplug_codegen_backend' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[01:21:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:21:55] 
[01:21:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:21:55] 
[01:21:55] 
[01:21:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAtstrap-1plb86h2refwc
134768 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f7ccxknqgy-1et5jhg-bfczq5obtrde
123704 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
115352 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
---
travis_time:end:12689c55:start=1544120426969093428,finish=1544120426975306343,duration=6212915
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:066b781c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d849476
travis_time:start:0d849476
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03ed4963
$ dmesg | grep -i kill
