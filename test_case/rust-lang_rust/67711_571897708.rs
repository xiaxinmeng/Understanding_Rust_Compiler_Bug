plain
2020-01-08T05:24:15.2860834Z status: exit code: 2
2020-01-08T05:24:15.2860918Z command: "make" "make"
2020-01-08T05:24:15.2861000Z stdout:
2020-01-08T05:24:15.2861407Z ------------------------------------------
2020-01-08T05:24:15.2862416Z c++ -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o foo.cpp
2020-01-08T05:24:15.2863002Z ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o
2020-01-08T05:24:15.2864159Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions  foo.rs -lfoo -lstdc++
2020-01-08T05:24:15.2871077Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/foo
2020-01-08T05:24:15.2871894Z DropCheck::drop
2020-01-08T05:24:15.2871970Z caught C++ exception
2020-01-08T05:24:15.2872061Z throwing rust panic
2020-01-08T05:24:15.2872133Z caught foreign exception in catch (...)
---
2020-01-08T05:24:15.2872872Z 
2020-01-08T05:24:15.2873309Z ------------------------------------------
2020-01-08T05:24:15.2873407Z stderr:
2020-01-08T05:24:15.2873684Z ------------------------------------------
2020-01-08T05:24:15.2874004Z ar: `u' modifier ignored since `D' is the default (see `U')
2020-01-08T05:24:15.2874303Z thread 'main' panicked at 'Box<Any>', foo.rs:43:9
2020-01-08T05:24:15.2874734Z thread 'main' panicked at 'Box<Any>', foo.rs:81:9
2020-01-08T05:24:15.2875117Z thread 'main' panicked at 'assertion failed: update_panic_count(0) == 0', src/libstd/panicking.rs:280:9
2020-01-08T05:24:15.2875247Z stack backtrace:
2020-01-08T05:24:15.2875247Z stack backtrace:
2020-01-08T05:24:15.2875638Z    0:     0x5573b16b2f24 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h897ccbc0cf5fa4b1
2020-01-08T05:24:15.2875994Z    1:     0x5573b16c77ec - core::fmt::write::h6cd612bcb450eae9
2020-01-08T05:24:15.2876319Z    2:     0x5573b16b0b65 - std::io::Write::write_fmt::hee34ccb6ecf82c22
2020-01-08T05:24:15.2876687Z    3:     0x5573b16b50d3 - std::panicking::default_hook::{{closure}}::hd45811bcd8b2d94e
2020-01-08T05:24:15.2877043Z    4:     0x5573b16b4cc8 - std::panicking::default_hook::hb5c6536ce69e528a
2020-01-08T05:24:15.2877390Z    5:     0x5573b16b5a01 - std::panicking::rust_panic_with_hook::h5a71b658679970c5
2020-01-08T05:24:15.2877763Z    6:     0x5573b16b575e - std::panicking::begin_panic::hfd4568567903b418
2020-01-08T05:24:15.2878100Z    7:     0x5573b16b63a4 - std::rt::lang_start_internal::h40579b8c4326241e
2020-01-08T05:24:15.2880610Z    8:     0x5573b16a8178 - std::rt::lang_start::h653edd866edf49c2
2020-01-08T05:24:15.2881087Z    9:     0x5573b16a90cb - main
2020-01-08T05:24:15.2881371Z   10:     0x7f8e4c3efb6b - __libc_start_main
2020-01-08T05:24:15.2882155Z   11:     0x5573b16a622a - _start
2020-01-08T05:24:15.2882649Z   12:                0x0 - <unknown>
2020-01-08T05:24:15.2882822Z Illegal instruction (core dumped)
2020-01-08T05:24:15.2882822Z Illegal instruction (core dumped)
2020-01-08T05:24:15.2882909Z make: *** [Makefile:4: all] Error 132
2020-01-08T05:24:15.2883204Z ------------------------------------------
2020-01-08T05:24:15.2883271Z 
2020-01-08T05:24:15.2883307Z 
2020-01-08T05:24:15.2883340Z 
---
2020-01-08T05:24:15.2885892Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-08T05:24:15.2886038Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-08T05:24:15.2886097Z 
2020-01-08T05:24:15.2886132Z 
2020-01-08T05:24:15.2893130Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm-project/llvm/include -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -std=c++11   -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_DEBUG -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-08T05:24:15.2895005Z 
2020-01-08T05:24:15.2895046Z 
2020-01-08T05:24:15.2895168Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-08T05:24:15.2895270Z Build completed unsuccessfully in 2:33:02
2020-01-08T05:24:15.2895270Z Build completed unsuccessfully in 2:33:02
2020-01-08T05:24:15.3007549Z == clock drift check ==
2020-01-08T05:24:15.3033093Z   local time: Wed Jan  8 05:24:15 UTC 2020
2020-01-08T05:24:15.8091029Z   network time: Wed, 08 Jan 2020 05:24:15 GMT
2020-01-08T05:24:15.8092500Z == end clock drift check ==
2020-01-08T05:24:16.2664081Z 
2020-01-08T05:24:16.2768840Z ##[error]Bash exited with code '1'.
2020-01-08T05:24:16.2817087Z ##[section]Starting: Checkout
2020-01-08T05:24:16.2819398Z ==============================================================================
2020-01-08T05:24:16.2819506Z Task         : Get sources
2020-01-08T05:24:16.2819609Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
