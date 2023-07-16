plain
travis_time:end:0107bf04:start=1553473603141631803,finish=1553473675836634531,duration=72695002728
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
[01:52:20] 
[01:52:20] running 9 tests
[01:52:20] iiiiiiiii
[01:52:20] 
[01:52:20]  finished in 0.149
[01:52:20] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:52:35] 
[01:52:35] running 120 tests
[01:52:59] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:53:04] .i......iii.i.....ii
[01:53:04] 
[01:53:04]  finished in 29.187
[01:53:04] travis_fold:end:test_debuginfo

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:18:46] 
[02:18:46] running 195 tests
[02:19:14] ..................i...i................................................................i............ 100/195
[02:19:59] ...........................F.......................i......................................i....
[02:19:59] 
[02:19:59] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[02:19:59] 
[02:19:59] error: make failed
[02:19:59] error: make failed
[02:19:59] status: exit code: 2
[02:19:59] command: "make"
[02:19:59] stdout:
[02:19:59] ------------------------------------------
[02:19:59] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[02:19:59] /bin/echo || exit 0 # This test requires /bin/echo to exist
[02:19:59] 
[02:19:59] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
[02:19:59]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
[02:19:59] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  some_crate.rs --crate-name some_crate --crate-type lib -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/some_crate \
[02:19:59]  -Z codegen-backend=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib -Z unstable-options
[02:19:59] Makefile:4: recipe for target 'all' failed
[02:19:59] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[02:19:59] ------------------------------------------
[02:19:59] stderr:
[02:19:59] ------------------------------------------
[02:19:59] ------------------------------------------
[02:19:59] warning: ignoring --out-dir flag due to -o flag
[02:19:59] 
[02:19:59] Segmentation fault (core dumped)
[02:19:59] make[1]: *** [all] Error 139
[02:19:59] ------------------------------------------
[02:19:59] 
[02:19:59] thread '[run-make] run-make-fulldeps/hotplug_codegen_backend' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[02:19:59] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[02:19:59] 
[02:19:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[02:19:59] 
[02:19:59] 
[02:19:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:19:59] 
[02:19:59] 
[02:19:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:19:59] Build completed unsuccessfully in 0:38:59
[02:19:59] Build completed unsuccessfully in 0:38:59
[02:19:59] make: *** [check] Error 1
[02:19:59] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1efb6e90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 02:48:04 UTC 2019
---
travis_time:end:15c29be0:start=1553482085791658772,finish=1553482085797583925,duration=5925153
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008039c3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.1276.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 1298]
[New LWP 1276]
[New LWP 1304]
[New LWP 1302]
[New LWP 1306]
[New LWP 1300]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fd8d92305ca in ?? ()
[Current thread is 1 (LWP 1298)]
#0  0x00007fd8d92305ca in ?? ()
#1  0x00007fd8dc5d70f0 in ?? ()
#2  0xf9535cb417ea8c00 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.24279.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 24293]
[New LWP 24279]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
travis_fold:start:after_failure.6
travis_time:start:2284d789
$ dmesg | grep -i kill
travis_time:end:2284d789:start=1553482088236585715,finish=1553482088247947968,duration=11362253
