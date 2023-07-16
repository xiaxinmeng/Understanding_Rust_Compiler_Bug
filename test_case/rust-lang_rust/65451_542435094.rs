plain
2019-10-15T22:39:21.2757280Z test [run-make] run-make-fulldeps/sysroot-crates-are-unstable ... ok
2019-10-15T22:39:21.2757900Z 
2019-10-15T22:39:21.2758090Z failures:
2019-10-15T22:39:21.2758150Z 
2019-10-15T22:39:21.2758920Z ---- [run-make] run-make-fulldeps/issue-64153 stdout ----
2019-10-15T22:39:21.2759490Z error: make failed
2019-10-15T22:39:21.2759640Z status: exit code: 2
2019-10-15T22:39:21.2759760Z command: "make"
2019-10-15T22:39:21.2759890Z stdout:
2019-10-15T22:39:21.2759890Z stdout:
2019-10-15T22:39:21.2760570Z ------------------------------------------
2019-10-15T22:39:21.2762730Z DYLD_LIBRARY_PATH="/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153:/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib:" '/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153 -L /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153  --crate-type rlib upstream.rs -o /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153/libupstream.rlib -Ccodegen-units=1
2019-10-15T22:39:21.2764900Z DYLD_LIBRARY_PATH="/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153:/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib:" '/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153 -L /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153  --crate-type staticlib downstream.rs -Clto -Ccodegen-units=1 -o /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153/libdownstream.a
2019-10-15T22:39:21.2765620Z # Dump all the symbols from the staticlib into `syms`
2019-10-15T22:39:21.2766700Z nm /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153/libdownstream.a > /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153/syms
2019-10-15T22:39:21.2767810Z # Count the global instances of `issue64153_test_function`. There'll be 2
2019-10-15T22:39:21.2768270Z # if the `upstream` object file got erronously included twice.
2019-10-15T22:39:21.2769350Z grep -c -e "[[:space:]]T[[:space:]]issue64153_test_function" /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153/syms > /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps/issue-64153/issue-64153/count
2019-10-15T22:39:21.2771100Z ------------------------------------------
2019-10-15T22:39:21.2771540Z stderr:
2019-10-15T22:39:21.2772310Z ------------------------------------------
2019-10-15T22:39:21.2772310Z ------------------------------------------
2019-10-15T22:39:21.2772960Z warning: ignoring --out-dir flag due to -o flag
2019-10-15T22:39:21.2773050Z 
2019-10-15T22:39:21.2773680Z warning: ignoring --out-dir flag due to -o flag
2019-10-15T22:39:21.2773770Z 
2019-10-15T22:39:21.2773850Z make: *** [all] Error 1
2019-10-15T22:39:21.2774520Z ------------------------------------------
2019-10-15T22:39:21.2774600Z 
2019-10-15T22:39:21.2774660Z 
2019-10-15T22:39:21.2774700Z 
---
2019-10-15T22:39:21.2778290Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-15T22:39:21.2778480Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-15T22:39:21.2778550Z 
2019-10-15T22:39:21.2778590Z 
2019-10-15T22:39:21.2785720Z command did not execute successfully: "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/vsts/agent/2.158.0/work/1/s/src/test/run-make-fulldeps" "--build-base" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-i686-apple-darwin" "--mode" "run-make" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "/Users/vsts/agent/2.158.0/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/vsts/agent/2.158.0/work/1/s/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/Users/vsts/agent/2.158.0/work/1/s/src/llvm-project/llvm/include -I/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/llvm/build/include -std=c++11 -stdlib=libc++  -fno-exceptions -fno-rtti -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-15T22:39:21.2788730Z 
2019-10-15T22:39:21.2788840Z 
2019-10-15T22:39:21.2789180Z failed to run: /Users/vsts/agent/2.158.0/work/1/s/build/bootstrap/debug/bootstrap test
2019-10-15T22:39:21.2789670Z Build completed unsuccessfully in 1:36:56
2019-10-15T22:39:21.2789670Z Build completed unsuccessfully in 1:36:56
2019-10-15T22:39:21.2854630Z == clock drift check ==
2019-10-15T22:39:21.2920490Z   local time: Tue Oct 15 22:39:21 UTC 2019
2019-10-15T22:39:21.3776980Z   network time: Tue, 15 Oct 2019 22:39:21 GMT
2019-10-15T22:39:21.3779910Z == end clock drift check ==
2019-10-15T22:39:21.3953830Z ##[error]Bash exited with code '1'.
2019-10-15T22:39:21.4000280Z ##[section]Starting: Upload CPU usage statistics
2019-10-15T22:39:21.4029330Z ==============================================================================
2019-10-15T22:39:21.4029470Z Task         : Bash
2019-10-15T22:39:21.4029580Z Description  : Run a Bash script on macOS, Linux, or Windows
