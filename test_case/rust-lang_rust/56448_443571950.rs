plain
travis_time:end:11985f2a:start=1543799952537175337,finish=1543799953630844992,duration=1093669655
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:07]    Compiling crossbeam-deque v0.2.0
[00:05:09]    Compiling syn v0.15.21
[00:05:10]    Compiling rustc-rayon v0.1.1
[00:05:26]    Compiling synstructure v0.10.1
[00:05:33]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[00:05:39]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:39]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:44]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:04]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
---
[00:22:05]    Compiling syn v0.15.21
[00:22:06]    Compiling crossbeam-deque v0.2.0
[00:22:09]    Compiling rustc-rayon v0.1.1
[00:22:22]    Compiling synstructure v0.10.1
[00:22:30]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[00:22:35]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:22:35]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:22:40]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:24:10]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:53] 
[00:54:53] running 119 tests
[00:54:56] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:54:56] .ii.i.....iiii.....
[00:54:56] 
[00:54:56]  finished in 3.385
[00:54:56] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:11] 
[00:55:11] running 118 tests
[00:55:34] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:55:38] ......iii.i.....ii
[00:55:38] 
[00:55:38]  finished in 27.169
[00:55:38] travis_fold:end:test_debuginfo

---
[01:17:25] travis_fold:end:test_stage1-rustc_lint

[01:17:25] travis_time:end:test_stage1-rustc_lint:start=1543804607377675117,finish=1543804607648785095,duration=271109978

[01:17:25] travis_fold:start:test_stage1-rustc_local_drop_derive
travis_time:start:test_stage1-rustc_local_drop_derive
Testing rustc_local_drop_derive stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:25]    Compiling rustc_local_drop_derive v0.1.0 (/checkout/src/librustc_local_drop_derive)
[01:17:25]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/rustc_local_drop_derive-2c2003ab05d45dcc
[01:17:25] 
[01:17:25] running 0 tests
[01:17:25] 
---
[01:17:25] 
[01:17:25]  finished in 0.533
[01:17:25] travis_fold:end:test_stage1-rustc_local_drop_derive

[01:17:25] travis_time:end:test_stage1-rustc_local_drop_derive:start=1543804607650839183,finish=1543804608184314098,duration=533474915
[01:17:25] travis_fold:start:test_stage1-rustc_metadata
travis_time:start:test_stage1-rustc_metadata
Testing rustc_metadata stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:25]    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
---
[01:23:17] command: "make"
[01:23:17] stdout:
[01:23:17] ------------------------------------------
[01:23:17] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/sysroot-crates-are-unstable'
[01:23:17] python2.7 test.py
[01:23:17] verifying if byteorder is an unstable crate
[01:23:17] verifying if backtrace is an unstable crate
[01:23:17] verifying if nodrop is an unstable crate
[01:23:17] verifying if smallvec is an unstable crate
[01:23:17] verifying if lazy_static is an unstable crate
[01:23:17] verifying if version_check is an unstable crate
[01:23:17] crate version_check "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libversion_check-2f2b80688e9c3135.rlib" is not unstable
[01:23:17] error: failed to write bytecode: Read-only file system (os error 30)
[01:23:17] error: aborting due to previous error
[01:23:17] 
[01:23:17] 
[01:23:17] 
[01:23:17] 
[01:23:17] verifying if cc is an unstable crate
[01:23:17] crate cc "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcc-99fd269482e631e8.rlib" is not unstable
[01:23:17] error: failed to write bytecode: Read-only file system (os error 30)
[01:23:17] 
[01:23:17] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:23:17] error: aborting due to 2 previous errors
[01:23:17] 
[01:23:17] 
[01:23:17] error: aborting due to previous error
[01:23:17] error: aborting due to previous error
[01:23:17] 
[01:23:17] For more information about this error, try `rustc --explain E0514`.
[01:23:17] 
[01:23:17] 
[01:23:17] verifying if parking_lot is an unstable crate
[01:23:17] verifying if tempfile is an unstable crate
[01:23:17] verifying if libc is an unstable crate
[01:23:17] verifying if lazy_static is an unstable crate
[01:23:17] verifying if term is an unstable crate
[01:23:17] verifying if rustc_lsan is an unstable crate
[01:23:17] verifying if unicode_xid is an unstable crate
[01:23:17] crate unicode_xid "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_xid-2e9145dd84b1091f.rlib" is not unstable
[01:23:17] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1412: worker thread panicked', src/librustc/util/bug.rs:47:26
[01:23:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:23:17] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:23:17] error: aborting due to previous error
[01:23:17] 
[01:23:17] 
[01:23:17] 
---
[01:23:17] verifying if serialize is an unstable crate
[01:23:17] verifying if miniz_sys is an unstable crate
[01:23:17] verifying if either is an unstable crate
[01:23:17] verifying if synstructure is an unstable crate
[01:23:17] crate synstructure "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsynstructure-c55ea77c48c98da4.rlib" is not unstable
[01:23:17] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:23:17] 
[01:23:17] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1412: worker thread panicked', src/librustc/util/bug.rs:47:26
[01:23:17] error: aborting due to previous error
[01:23:17] 
[01:23:17] 
[01:23:17] 
[01:23:17] 
[01:23:17] verifying if getopts is an unstable crate
[01:23:17] verifying if rustc_tsan is an unstable crate
[01:23:17] verifying if panic_unwind is an unstable crate
[01:23:17] verifying if jobserver is an unstable crate
[01:23:17] verifying if stable_deref_trait is an unstable crate
[01:23:17] verifying if atty is an unstable crate
[01:23:17] verifying if lock_api is an unstable crate
[01:23:17] verifying if cmake is an unstable crate
[01:23:17] crate cmake "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcmake-afb19a8695d09bc4.rlib" is not unstable
[01:23:17] error[E0514]: found crate `cmake` compiled by an incompatible version of rustc
[01:23:17]  --> <anon>:1:1
[01:23:17] 1 | extern crate cmake;
[01:23:17]   | ^^^^^^^^^^^^^^^^^^^
[01:23:17]   |
[01:23:17]   |
[01:23:17]   = help: please recompile that crate using this compiler (rustc 1.32.0-dev)
[01:23:17]   = note: the following crate versions were found:
[01:23:17]           crate `cmake` compiled by rustc 1.31.0-beta.1 (2824a67b0 2018-10-29): /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcmake-afb19a8695d09bc4.rlib
[01:23:17] error: aborting due to previous error
[01:23:17] 
[01:23:17] For more information about this error, try `rustc --explain E0514`.
[01:23:17] 
[01:23:17] 
[01:23:17] 
[01:23:17] verifying if bitflags is an unstable crate
[01:23:17] verifying if chalk_macros is an unstable crate
[01:23:17] verifying if unwind is an unstable crate
[01:23:17] verifying if syn is an unstable crate
[01:23:17] crate syn "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyn-fd7d5d1b02b75881.rlib" is not unstable
[01:23:17] error: could not write output to rust_out.3b0f5964mhfddkur.rcgu.o: Read-only file system
[01:23:17] error: aborting due to previous error
[01:23:17] 
[01:23:17] 
[01:23:17] thread '<unnamed>' panicked at 'src/librustc_codegen_ssa/back/write.rs:1412: worker thread panicked', src/librustc/util/bug.rs:47:26
[01:23:17] 
[01:23:17] 
[01:23:17] verifying if cfg_if is an unstable crate
[01:23:17] verifying if env_logger is an unstable crate
[01:23:17] verifying if env_logger is an unstable crate
[01:23:17] verifying if alloc is an unstable crate
[01:23:17] verifying if rand is an unstable crate
[01:23:17] verifying if rustc_asan is an unstable crate
[01:23:17] verifyr lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:17] 
[01:23:17] 
[01:23:1travis_time:end:1440d945:start=1543799962477996023,finish=1543804960366604965,duration=4997888608942
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
---
travis_time:end:37f46ce7:start=1543804962509687740,finish=1543804962516083398,duration=6395658
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:283af1a8
$ ln -s . checkout && f
