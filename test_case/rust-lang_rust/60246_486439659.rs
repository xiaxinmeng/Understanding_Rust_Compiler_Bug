plain
travis_time:end:001763f4:start=1556135562249149308,finish=1556135563013292812,duration=764143504
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:24] 
[01:22:24] running 9 tests
[01:22:24] iiiiiiiii
[01:22:24] 
[01:22:24]  finished in 0.151
[01:22:24] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:40] 
[01:22:40] running 121 tests
[01:23:06] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:10] i.i......iii.i.....ii
[01:23:10] 
[01:23:10]  finished in 30.681
[01:23:10] travis_fold:end:test_debuginfo

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:52:08] 
[01:52:08] running 197 tests
[01:52:37] ..................i...i................................................................i............ 100/197
[01:53:23] ...................................F.................i......................................i....
[01:53:23] failures:
[01:53:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:53:23] 
[01:53:23] ---- [run-make] run-make-fulldeps/pretty-print-path-suffix stdout ----
[01:53:23] ---- [run-make] run-make-fulldeps/pretty-print-path-suffix stdout ----
[01:53:23] 
[01:53:23] error: make failed
[01:53:23] status: exit code: 2
[01:53:23] command: "make"
[01:53:23] stdout:
[01:53:23] ------------------------------------------
[01:53:23] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/pretty-print-path-suffix'
[01:53:23] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix/foo.out -Z unpretty=hir=foo input.rs
[01:53:23] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix/nest_foo.out -Z unpretty=hir=nest::foo input.rs
[01:53:23] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix/foo_method.out -Z unpretty=hir=foo_method input.rs
[01:53:23] diff -u /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix/foo.out foo.pp
[01:53:23] --- /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix/foo.out 2019-04-24 21:45:41.689211650 +0000
[01:53:23] +++ foo.pp 2019-04-24 19:52:41.805211650 +0000
[01:53:23] @@ -0,0 +1,5 @@
[01:53:23] +
[01:53:23] +pub fn foo() -> i32 { 45 } /* foo */
[01:53:23] +
[01:53:23] +
[01:53:23] +pub fn foo() -> &'static str { "i am a foo." } /* nest::foo */
[01:53:23] Makefile:4: recipe for target 'all' failed
[01:53:23] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/pretty-print-path-suffix'
[01:53:23] ------------------------------------------
[01:53:23] stderr:
[01:53:23] ------------------------------------------
[01:53:23] ------------------------------------------
[01:53:23] warning: ignoring --out-dir flag due to -o flag
[01:53:23] 
[01:53:23] warning: ignoring --out-dir flag due to -o flag
[01:53:23] 
[01:53:23] warning: ignoring --out-dir flag due to -o flag
[01:53:23] 
[01:53:23] make[1]: *** [all] Error 1
[01:53:23] ------------------------------------------
[01:53:23] 
[01:53:23] 
[01:53:23] 
[01:53:23] 
[01:53:23] failures:
[01:53:23]     [run-make] run-make-fulldeps/pretty-print-path-suffix
[01:53:23] 
[01:53:23] test result: FAILED. 191 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[01:53:23] 
[01:53:23] 
[01:53:23] 
[01:53:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:53:23] 
[01:53:23] 
[01:53:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:53:23] Build completed unsuccessfully in 0:42:50
[01:53:23] Build completed unsuccessfully in 0:42:50
[01:53:23] make: *** [check] Error 1
[01:53:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1004690c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 21:46:16 UTC 2019
