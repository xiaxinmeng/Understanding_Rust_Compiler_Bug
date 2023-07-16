

---- [run-make] run-make\issue-33329 stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[1]: Entering directory '/c/bot/slave/auto-win-msvc-64-opt-rustbuild/build/src/test/run-make/issue-33329'
C:\bot\slave\auto-win-msvc-64-opt-rustbuild\build\obj\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --target x86_64_unknown-linux-musl main.rs 2>&1 | \
    grep "error: Error loading target specification: Could not find specification for target"
Makefile:2: recipe for target 'all' failed
make[1]: Leaving directory '/c/bot/slave/auto-win-msvc-64-opt-rustbuild/build/src/test/run-make/issue-33329'

------------------------------------------
stderr:
------------------------------------------
make[1]: *** [all] Error 1

------------------------------------------

thread '[run-make] run-make\issue-33329' panicked at 'explicit panic', C:\bot\slave\auto-win-msvc-64-opt-rustbuild\build\src\tools\compiletest\src\runtest.rs:2243
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-make] run-make\issue-33329

test result: FAILED. 129 passed; 1 failed; 0 ignored; 0 measured



command did not execute successfully: "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\compiletest.exe" "--compile-lib-path" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--src-base" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\src/test\\run-make" "--build-base" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\test\\run-make" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--mode" "run-make" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\llvm\\build\\Release/bin\\FileCheck.exe" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=C:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\rust-test-helpers" "--docck-python" "python" "--lldb-python" "python" "--gdb-version" "GNU gdb (GDB) 7.8" "--cc" "C:\\Program Files (x86)\\Microsoft Visual Studio 12.0\\VC/bin\\amd64\\cl.exe" "--cxx" "C:\\Program Files (x86)\\Microsoft Visual Studio 12.0\\VC/bin\\amd64\\cl.exe" "--cflags" "/nologo /MD" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter codegen core debuginfocodeview debuginfodwarf debuginfopdb engine executionengine instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser native nativecodegen objcarcopts object option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag support symbolize tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils" "--llvm-cxxflags" "-IC:\\bot\\slave\\auto-win-msvc-64-opt-rustbuild\\build\\obj\\build\\x86_64-pc-windows-msvc\\llvm/include  /nologo /MD /W4 -wd4141 -wd4146 -wd4180 -wd4244 -wd4258 -wd4267 -wd4291 -wd4345 -wd4351 -wd4355 -wd4456 -wd4457 -wd4458 -wd4459 -wd4503 -wd4624 -wd4722 -wd4800 -wd4100 -wd4127 -wd4512 -wd4505 -wd4610 -wd4510 -wd4702 -wd4245 -wd4706 -wd4310 -wd4701 -wd4703 -wd4389 -wd4611 -wd4805 -wd4204 -wd4577 -wd4091 -wd4592 -wd4324 -w14062 -we4238 /Zc:inline /MD /O2 /Ob2 /D NDEBUG  /EHs-c- /GR- /MP -D_DEBUG_POINTER_IMPL= -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
thread 'main' panicked at 'Some tests failed', C:\bot\slave\auto-win-msvc-64-opt-rustbuild\build\src\tools\compiletest\src\main.rs:293
expected success, got: exit code: 101
