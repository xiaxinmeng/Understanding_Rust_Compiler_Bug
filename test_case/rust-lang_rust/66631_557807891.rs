plain
2019-11-23T15:37:26.3164760Z test [run-make] run-make-fulldeps/output-filename-overwrites-input ... ok
2019-11-23T15:37:26.5959740Z test [run-make] run-make-fulldeps/output-with-hyphens ... ok
2019-11-23T15:37:27.0449480Z test [run-make] run-make-fulldeps/lto-smoke ... ok
2019-11-23T15:37:27.2535190Z test [run-make] run-make-fulldeps/panic-impl-transitive ... ok
2019-11-23T15:37:28.2583460Z test [run-make] run-make-fulldeps/pgo-branch-weights ... FAILED
2019-11-23T15:37:28.8280930Z test [run-make] run-make-fulldeps/override-aliased-flags ... ok
2019-11-23T15:37:29.0901750Z test [run-make] run-make-fulldeps/pgo-gen-no-imp-symbols ... ok
2019-11-23T15:37:29.8320510Z test [run-make] run-make-fulldeps/mixing-formats ... ok
2019-11-23T15:37:30.1077170Z test [run-make] run-make-fulldeps/pgo-indirect-call-promotion ... ok
---
2019-11-23T15:37:51.1381640Z test [run-make] run-make-fulldeps/sysroot-crates-are-unstable ... ok
2019-11-23T15:37:51.1381870Z 
2019-11-23T15:37:51.1381970Z failures:
2019-11-23T15:37:51.1382050Z 
2019-11-23T15:37:51.1382780Z ---- [run-make] run-make-fulldeps/pgo-branch-weights stdout ----
2019-11-23T15:37:51.1382950Z error: make failed
2019-11-23T15:37:51.1383410Z status: exit code: 2
2019-11-23T15:37:51.1383490Z command: "make"
2019-11-23T15:37:51.1383580Z stdout:
2019-11-23T15:37:51.1383580Z stdout:
2019-11-23T15:37:51.1384350Z ------------------------------------------
2019-11-23T15:37:51.1385050Z # We don't compile `opaque` with either optimizations or instrumentation.
2019-11-23T15:37:51.1385850Z # We don't compile `opaque` with either optimizations or instrumentation.
2019-11-23T15:37:51.1387270Z DYLD_LIBRARY_PATH="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   opaque.rs
2019-11-23T15:37:51.1387630Z # Compile the test program with instrumentation
2019-11-23T15:37:51.1388450Z mkdir -p "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir
2019-11-23T15:37:51.1390120Z DYLD_LIBRARY_PATH="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   interesting.rs \
2019-11-23T15:37:51.1391300Z   -Cprofile-generate="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir -O -Ccodegen-units=1
2019-11-23T15:37:51.1393000Z DYLD_LIBRARY_PATH="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   main.rs -Cprofile-generate="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir -O
2019-11-23T15:37:51.1393480Z # The argument below generates to the expected branch weights
2019-11-23T15:37:51.1394710Z DYLD_LIBRARY_PATH="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib:" /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/main aaaaaaaaaaaa2bbbbbbbbbbbb2bbbbbbbbbbbbbbbbcc || exit 1
2019-11-23T15:37:51.1395660Z "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin"/llvm-profdata merge \
2019-11-23T15:37:51.1396500Z   -o "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir/merged.profdata \
2019-11-23T15:37:51.1397380Z   "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir
2019-11-23T15:37:51.1398840Z DYLD_LIBRARY_PATH="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   interesting.rs \
2019-11-23T15:37:51.1400200Z   -Cprofile-use="/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir/merged.profdata -O \
2019-11-23T15:37:51.1400900Z   -Ccodegen-units=1 --emit=llvm-ir
2019-11-23T15:37:51.1401900Z cat "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/interesting.ll | "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" filecheck-patterns.txt
2019-11-23T15:37:51.1402720Z ------------------------------------------
2019-11-23T15:37:51.1402850Z stderr:
2019-11-23T15:37:51.1403480Z ------------------------------------------
2019-11-23T15:37:51.1403480Z ------------------------------------------
2019-11-23T15:37:51.1404190Z filecheck-patterns.txt:5:8: error: CHECK: expected string not found in input
2019-11-23T15:37:51.1404330Z CHECK: define void @function_called_twice(i32 %c) {{.*}} !prof !29 {
2019-11-23T15:37:51.1404540Z <stdin>:1:1: note: scanning from here
2019-11-23T15:37:51.1404540Z <stdin>:1:1: note: scanning from here
2019-11-23T15:37:51.1405180Z ; ModuleID = 'interesting.3a1fbbbh-cgu.0'
2019-11-23T15:37:51.1405310Z ^
2019-11-23T15:37:51.1405390Z <stdin>:7:1: note: possible intended match here
2019-11-23T15:37:51.1405500Z define void @function_called_twice(i32 %c) unnamed_addr #0 !prof !28 {
2019-11-23T15:37:51.1405600Z ^
2019-11-23T15:37:51.1405880Z make: *** [all] Error 1
2019-11-23T15:37:51.1406780Z ------------------------------------------
2019-11-23T15:37:51.1406860Z 
2019-11-23T15:37:51.1406920Z 
2019-11-23T15:37:51.1406990Z 
2019-11-23T15:37:51.1406990Z 
2019-11-23T15:37:51.1407060Z failures:
2019-11-23T15:37:51.1407730Z     [run-make] run-make-fulldeps/pgo-branch-weights
2019-11-23T15:37:51.1408510Z test result: FAILED. 184 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-23T15:37:51.1408610Z 
2019-11-23T15:37:51.1409310Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-23T15:37:51.1409450Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-23T15:37:51.1409450Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-23T15:37:51.1409520Z 
2019-11-23T15:37:51.1409580Z 
2019-11-23T15:37:51.1416390Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/run-make-fulldeps" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-make" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "/Users/runner/runners/2.160.1/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang" "--cxx" "/Users/runner/runners/2.160.1/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/Users/runner/runners/2.160.1/work/1/s/src/llvm-project/llvm/include -I/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/include -std=c++11 -stdlib=libc++  -fno-exceptions -fno-rtti -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-23T15:37:51.1419010Z 
2019-11-23T15:37:51.1419060Z 
2019-11-23T15:37:51.1419220Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-11-23T15:37:51.1419340Z Build completed unsuccessfully in 1:36:09
2019-11-23T15:37:51.1419340Z Build completed unsuccessfully in 1:36:09
2019-11-23T15:37:51.1478850Z == clock drift check ==
2019-11-23T15:37:51.1534430Z   local time: Sat Nov 23 15:37:51 UTC 2019
2019-11-23T15:37:51.2447070Z   network time: Sat, 23 Nov 2019 15:37:51 GMT
2019-11-23T15:37:51.2449830Z == end clock drift check ==
2019-11-23T15:37:51.2500010Z 
2019-11-23T15:37:51.2637210Z ##[error]Bash exited with code '1'.
2019-11-23T15:37:51.2686240Z ##[section]Starting: Checkout
2019-11-23T15:37:51.2689210Z ==============================================================================
2019-11-23T15:37:51.2689320Z Task         : Get sources
2019-11-23T15:37:51.2689420Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
