plain
2019-11-14T22:07:33.5041033Z 
2019-11-14T22:07:33.5041279Z ------------------------------------------
2019-11-14T22:07:33.5041365Z stderr:
2019-11-14T22:07:33.5041589Z ------------------------------------------
2019-11-14T22:07:33.5041677Z make: python: Command not found
2019-11-14T22:07:33.5041751Z make: *** [all] Error 127
2019-11-14T22:07:33.5042044Z ------------------------------------------
2019-11-14T22:07:33.5042092Z 
2019-11-14T22:07:33.5042141Z 
2019-11-14T22:07:33.5042174Z 
---
2019-11-14T22:07:33.5047786Z ', src/tools/compiletest/src/main.rs:537:22
2019-11-14T22:07:33.5047911Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-14T22:07:33.5053894Z 
2019-11-14T22:07:33.5053969Z 
2019-11-14T22:07:33.5066136Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "run-make" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m32 -march=i686" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm-project/llvm/include -I/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/include -std=c++11   -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-14T22:07:33.5068730Z 
2019-11-14T22:07:33.5069846Z 
2019-11-14T22:07:33.5070433Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/bootstrap --exclude src/test/rustdoc-js --exclude src/tools/error_index_generator --exclude src/tools/linkchecker
2019-11-14T22:07:33.5070586Z Build completed unsuccessfully in 1:54:14
2019-11-14T22:07:33.5070586Z Build completed unsuccessfully in 1:54:14
2019-11-14T22:07:33.5117888Z == clock drift check ==
2019-11-14T22:07:33.5140781Z   local time: Thu Nov 14 22:07:33 UTC 2019
2019-11-14T22:07:33.7817243Z   network time: Thu, 14 Nov 2019 22:07:33 GMT
2019-11-14T22:07:33.7818420Z == end clock drift check ==
2019-11-14T22:07:44.4005130Z 
2019-11-14T22:07:44.4133680Z ##[error]Bash exited with code '1'.
2019-11-14T22:07:44.4170812Z ##[section]Starting: Checkout
2019-11-14T22:07:44.4173122Z ==============================================================================
2019-11-14T22:07:44.4173251Z Task         : Get sources
2019-11-14T22:07:44.4173371Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
