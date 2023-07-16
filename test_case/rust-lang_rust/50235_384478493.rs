plain
[00:04:51]    Compiling lazy_static v1.0.0
[00:04:51]    Compiling scopeguard v0.3.3
[00:04:52]    Compiling memoffset v0.2.1
[00:04:52]    Compiling libc v0.2.40
[00:04:52]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#52633662)
[00:04:52]    Compiling stable_deref_trait v1.0.0
[00:04:52]    Compiling smallvec v0.6.0
[00:04:52]    Compiling either v1.5.0
[00:04:52]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:05:21]    Compiling rls-data v0.15.0
[00:05:21]    Compiling parking_lot v0.5.4
[00:05:24]    Compiling flate2 v1.0.1
[00:05:26]    Compiling backtrace v0.3.6
[00:05:27]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#52633662)
[00:05:32]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:32]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:35]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:59]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
---
[00:22:03]    Compiling memoffset v0.2.1
[00:22:03]    Compiling scopeguard v0.3.3
[00:22:03]    Compiling lazy_static v1.0.0
[00:22:03]    Compiling libc v0.2.40
[00:22:03]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#52633662)
[00:22:03]    Compiling scoped-tls v0.1.1
[00:22:03]    Compiling smallvec v0.6.0
[00:22:03]    Compiling either v1.5.0
[00:22:03]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:22:25]    Compiling rls-data v0.15.0
[00:22:27]    Compiling crossbeam-deque v0.2.0
[00:22:27]    Compiling parking_lot v0.5.4
[00:22:27]    Compiling flate2 v1.0.1
[00:22:30]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#52633662)
[00:22:34]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:22:36]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:22:36]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:22:39]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[00:41:26]    Compiling ena v0.9.2
[00:41:26]    Compiling crossbeam-epoch v0.3.1
[00:41:27]    Compiling crossbeam-deque v0.2.0
[00:41:27]    Compiling parking_lot_core v0.2.13
[00:41:27]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#52633662)
[00:41:28]    Compiling parking_lot v0.5.4
[00:41:29]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#52633662)
[00:41:35]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:41:36]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:41:38]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[00:42:02]  Documenting proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:42:02]  Documenting proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:42:04] warning: [cfg] cannot be resolved, ignoring it...
[00:42:04] 
[00:42:04] warning: [rayon::prelude] cannot be resolved, ignoring it...
[00:42:04] warning: [Experimental] cannot be resolved, ignoring it...
[00:42:04] 
[00:42:04] 
[00:42:04] warning: [plumbing] cannot be resolved, ignoring it...
[00:42:05] warning: [Garbage] cannot be resolved, ignoring it...
[00:42:05] 
[00:42:05] warning: [flat_map] cannot be resolved, ignoring it...
[00:42:05] 
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:52] 
[01:28:52] running 182 tests
[01:29:35] .............................................................F......................................
[01:30:25] .................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:31:43] failures:
[01:31:43] 
[01:31:43] ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
[01:31:43]  
[01:31:43]  
[01:31:43] error: make failed
[01:31:43] status: exit code: 2
[01:31:43] command: "make"
[01:31:43] stdout:
[01:31:43] ------------------------------------------
[01:31:43] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/issue-19371'
[01:31:43] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371.stage2-x86_64-unknown-linux-gnu  foo.rs
[01:31:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:31:43] Makefile:8: recipe for target 'all' failed
[01:31:43] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/issue-19371'
[01:31:43] ------------------------------------------
[01:31:43] stderr:
[01:31:43] ------------------------------------------
[01:31:43] error[E0432]: unresolved import `rustc_data_structures`
[01:31:43] error[E0432]: unresolved import `rustc_data_structures`
[01:31:43]   --> foo.rs:29:5
[01:31:43]    |
[01:31:43] 29 | use rustc_data_structures::sync;
[01:31:43]    |     ^^^^^^^^^^^^^^^^^^^^^ Maybe a missing `extern crate rustc_data_structures;`?
[01:31:43] error: aborting due to previous error
[01:31:43] 
[01:31:43] For more information about this error, try `rustc --explain E0432`.
[01:31:43] For more information about this error, try `rustc --explain E0432`.
[01:31:43] make[1]: *** [all] Error 101
[01:31:43] ------------------------------------------
[01:31:43] 
[01:31:43] 
[01:31:43] thread '[run-make] run-make-fulldeps/issue-19371' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[01:31:43] 
[01:31:43] 
[01:31:43] failures:
[01:31:43]     [run-make] run-make-fulldeps/issue-19371
[01:31:43]     [run-make] run-make-fulldeps/issue-19371
[01:31:43] 
[01:31:43] test result: FAILED. 181 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:31:43] 
[01:31:43] 
[01:31:43] 
[01:31:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler travis_time:start:04f70dcc
Thu Apr 26 00:53:14 UTC 2018
Thu, 26 Apr 2018 00:53:14 GMT
travis_time:end:04f70dcc:start=1524703994790565159,finish=1524703994858866947,duration=68301788

