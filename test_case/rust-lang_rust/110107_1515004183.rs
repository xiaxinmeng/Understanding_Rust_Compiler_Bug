plain
---- [run-make] tests/run-make/const-prop-lint stdout ----

error: make failed
status: exit status: 2
command: cd "/Users/runner/work/rust/rust/tests/run-make/const-prop-lint" && AR="ar" CC="/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" CXX="/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang++ -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" HOST_RPATH_DIR="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" LD_LIB_PATH_ENVVAR="DYLD_LIBRARY_PATH" LLVM_BIN_DIR="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/FileCheck" NODE="/usr/local/bin/node" PYTHON="/usr/bin/python3" RUSTC="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" RUSTDOC="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-apple-darwin" RUST_DEMANGLER="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2-tools-bin/rust-demangler" S="/Users/runner/work/rust/rust" TARGET="x86_64-apple-darwin" TARGET_RPATH_DIR="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" TMPDIR="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/const-prop-lint/const-prop-lint" "make"
--- stdout -------------------------------
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/const-prop-lint/const-prop-lint:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-bootstrap-tools/x86_64-apple-darwin/release/deps:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/lib" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/const-prop-lint/const-prop-lint -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/const-prop-lint/const-prop-lint  input.rs; test $? -eq 1
ls *.o; test $? -eq 2
--- stderr -------------------------------
warning: unused variable: `x`
 --> input.rs:4:9
  |
  |
4 |     let x = 255u8 + 1;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

error: this arithmetic operation will overflow
  |
4 |     let x = 255u8 + 1;
4 |     let x = 255u8 + 1;
  |             ^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
note: the lint level is defined here
 --> input.rs:1:9
  |
  |
1 | #![deny(arithmetic_overflow)]

error: aborting due to previous error; 1 warning emitted


ls: *.o: No such file or directory
make: *** [all] Error 1



failures:
