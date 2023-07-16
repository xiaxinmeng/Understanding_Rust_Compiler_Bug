plain
---- [js-doc-test] rustdoc-js\struct-like-variant.rs stdout ----

error: rustdoc-js test failed!
status: exit code: 1
command: "C:\\Program Files\\nodejs\\node" "D:\\a\\rust\\rust\\src/tools/rustdoc-js/tester.js" "--doc-folder" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc-js\\struct-like-variant" "--crate-name" "struct_like_variant" "--test-file" "D:\\a\\rust\\rust\\src/test\\rustdoc-js\\struct-like-variant.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
internal/fs/utils.js:332
    throw err;


Error: ENOENT: no such file or directory, open 'D:\a\rust\rust\build\i686-pc-windows-msvc\test\rustdoc-js\struct-like-variant\search.js'
    at Object.openSync (fs.js:497:3)
    at Object.readFileSync (fs.js:393:35)
    at readFile (D:\a\rust\rust\src\tools\rustdoc-js\tester.js:181:15)
    at load_files (D:\a\rust\rust\src\tools\rustdoc-js\tester.js:391:20)
    at main (D:\a\rust\rust\src\tools\rustdoc-js\tester.js:471:27)
    at Object.<anonymous> (D:\a\rust\rust\src\tools\rustdoc-js\tester.js:491:14)
    at Module._compile (internal/modules/cjs/loader.js:1085:14)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:1114:10)
    at Module.load (internal/modules/cjs/loader.js:950:32)
    at Function.Module._load (internal/modules/cjs/loader.js:790:12) {
  errno: -4058,
  syscall: 'open',
  code: 'ENOENT',
  path: 'D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc-js\\struct-like-variant\\search.js'

------------------------------------------


---
test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.54s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\rustdoc-js" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc-js" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "rustdoc-js" "--mode" "js-doc-test" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.10.0\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.10.0\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:59:23
Build completed unsuccessfully in 0:59:23
make: *** [Makefile:72: ci-subset-1] Error 1
