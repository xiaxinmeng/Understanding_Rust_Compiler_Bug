plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
running 81 tests
iiF.i...i.i..ii........iiiiii..................ii.i...iiiiiii.iiii...............
failures:

---- [run-make] tests/run-make/issue-107205 stdout ----
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: make failed
status: exit status: 2
status: exit status: 2
command: cd "/checkout/tests/run-make/issue-107205" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-13/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-13/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-107205/issue-107205" "make"
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc main.rs
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
error: couldn't create a temp dir: Read-only file system (os error 30) at path "/checkout/tests/run-make/issue-107205/rmetaU0co17"
error: aborting due to previous error

make: *** [Makefile:2: all] Error 1
------------------------------------------
