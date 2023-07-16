plain
travis_time:end:1d5bce8b:start=1543831341297730946,finish=1543831396049658308,duration=54751927362
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:55]    Compiling crossbeam-deque v0.2.0
[00:04:55]    Compiling syn v0.15.21
[00:04:58]    Compiling rustc-rayon v0.1.1
[00:05:11]    Compiling synstructure v0.10.1
[00:05:18]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[00:05:24]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:25]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:29]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:47]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
---
[00:21:13]    Compiling flate2 v1.0.3
[00:21:13]    Compiling crossbeam-deque v0.2.0
[00:21:17]    Compiling rustc-rayon v0.1.1
[00:21:29]    Compiling synstructure v0.10.1
[00:21:36]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[00:21:41]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:21:42]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:21:46]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:23:12]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:19] 
[00:53:19] running 119 tests
[00:53:21] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:53:22] .ii.i.....iiii.....
[00:53:22] 
[00:53:22]  finished in 3.194
[00:53:22] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:35] 
[00:53:35] running 118 tests
[00:53:58] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:54:01] ......iii.i.....ii
[00:54:01] 
[00:54:01]  finished in 26.240
[00:54:01] travis_fold:end:test_debuginfo

---
[01:14:33] travis_fold:end:test_stage1-rustc_lint

[01:14:33] travis_time:end:test_stage1-rustc_lint:start=1543835877680908835,finish=1543835877937200137,duration=256291302

[01:14:33] travis_fold:start:test_stage1-rustc_local_drop_derive
travis_time:start:test_stage1-rustc_local_drop_derive
Testing rustc_local_drop_derive stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:33]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[01:14:33]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/rustc_local_drop_derive-2c2003ab05d45dcc
[01:14:33] 
[01:14:33] running 0 tests
[01:14:33] 
---
[01:14:34] 
[01:14:34]  finished in 0.509
[01:14:34] travis_fold:end:test_stage1-rustc_local_drop_derive

[01:14:34] travis_time:end:test_stage1-rustc_local_drop_derive:start=1543835877939104787,finish=1543835878448616685,duration=509511898
[01:14:34] travis_fold:start:test_stage1-rustc_metadata
travis_time:start:test_stage1-rustc_metadata
Testing rustc_metadata stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:34]    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
---
[01:17:56] running 192 tests
[01:18:19] .................................................................................................... 100/192
[01:19:17] ..........................................................................................Ftest [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
d_core is an unstable crate
[01:20:03] verifying if memoffset is an unstable crate
[01:20:03] verifying if ena is an unstable crate
[01:20:03] verifying if proc_macro2 is an unstable crate
[01:20:03] verifying if num_cpus is an unstable crate
[01:20:03] verifying if crossbeam_utils is an unstable crate
[01:20:03] verifying if rustc_demangle is an unstable crate
[01:20:03] verifying if scoped_tls is an unstable crate
[01:20:03] verifying if crossbeam_deque is an unstable crate
[01:20:03] verifying if datafrog is an unstable crate
[01:20:03] verifying if rustc_serialize is an unstable crate
[01:20:03] verifying if rustc_msan is an unstable crate
[01:20:03] verifying if libc is an unstable crate
[01:20:03] verifying if panic_abort is an unstable crate
[01:20:03] verifying if log_settings is an unstable crate
[01:20:03] verifying if rls_data is an unstable crate
[01:20:03] verifying if memmap is an unstable crate
[01:20:03] verifying if build_helper is an unstable crate
[01:20:03] verifying if build_helper is an unstable crate
[01:20:03] crate build_helper "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbuild_helper-fc70adc6582f8056.rlib" is not unstable
[01:20:03] error[E0514]: found crate `build_helper` compiled by an incompatible version of rustc
[01:20:03]  --> <anon>:1:1
[01:20:03] 1 | extern crate build_helper;
[01:20:03]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:20:03]   |
[01:20:03]   |
[01:20:03]   = help: please recompile that crate using this compiler (rustc 1.32.0-dev)
[01:20:03]   = note: the following crate versions were found:
[01:20:03]       verifying if panic_unwind is an unstable crate
[01:20:03] verifying if jobserver is an unstable crate
[01:20:03] verifying if atty is an unstable crate
[01:20:03] verifying if atty is an unstable crate
[01:20:03] verifying if lock_api is an unstable crate
[01:20:03] verifying if cmake is an unstable crate
[01:20:03] crate cmake "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcmake-afb19a8695d09bc4.rlib" is not unstable
[01:20:03] error[E0514]: found crate `cmake` compiled by an incompatible version of rustc
[01:20:03]  --> <anon>:1:1
[01:20:03] 1 | extern crate cmake;
[01:20:03]   | ^^^^^^^^^^^^^^^^^^^
[01:20:03]   |
[01:20:03]   |
[01:20:03]   = help: please recompile that crate using this compiler (rustc 1.32.0-dev)
[01:20:03]   = note: the following crate versions were found:
[01:20:03]           crate `cmake` compiled by rustc 1.31.0-beta.1 (2824a67b0 2018-10-29): /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcmake-afb19a8695d09bc4.rlib
[01:20:03] error: aborting due to previous error
[01:20:03] 
[01:20:03] For more information about this error, try `rustc --explain E0514`.
[01:20:03] 
[01:20:03] 
[01:20:03] 
[01:20:03] verifying if bitflags is an unstable crate
[01:20:03] verifying if chalk_macros is an unstable crate
[01:20:03] verifying if unwind is an unstable crate
[01:20:03] verifying if syn is an unstable crate
[01:20:03] verifying if cfg_if is an unstable crate
[01:20:03] verifying if env_logger is an unstable crate
[01:20:03] verifying if alloc is an unstable crate
[01:20:nknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:03] 
[01:20:03] 
[01:20:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:03] Build completed unsuccessfully in 0:36:50
[01:20:03] Build completed unsuccessfully in 0:36:50
[01:20:03] make: *** [check] Error 1
[01:20:03] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e3c2b08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 11:23:28 UTC 2018
