plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make/issue-10971-temps-dir'
touch /d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir/lib.rs
D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin\rustc.exe --crate-type=lib --temps-dir=/d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir/temp1 --out-dir=/d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir /d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir/lib.rs \
 & D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin\rustc.exe --crate-type=cdylib --temps-dir=/d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir/temp2 --out-dir=/d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir /d/a/rust/rust/build/i686-pc-windows-msvc/test/run-make/issue-10971-temps-dir/issue-10971-temps-dir/lib.rs
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make/issue-10971-temps-dir'
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
/bin/sh: line 1: D:arustrustbuildi686-pc-windows-msvcstage2binrustc.exe: command not found
/bin/sh: line 2: D:arustrustbuildi686-pc-windows-msvcstage2binrustc.exe: command not found
make[1]: *** [Makefile:6: all] Error 127
------------------------------------------




failures:
    [run-make] run-make\issue-10971-temps-dir

test result: FAILED. 13 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out; finished in 2.87s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\run-make" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\run-make" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "run-make" "--mode" "run-make" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin" "--cc" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30037\\bin\\HostX64\\x86\\cl.exe" "--cxx" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30037\\bin\\HostX64\\x86\\cl.exe" "--cflags" "-nologo -MT -Brepro" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:44:27
Build completed unsuccessfully in 0:44:27
make: *** [Makefile:72: ci-subset-1] Error 1
