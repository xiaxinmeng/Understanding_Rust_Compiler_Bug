plain
---- [run-make] tests/run-make-fulldeps/obtain-borrowck stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make-fulldeps/obtain-borrowck" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck  driver.rs -o ""/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck"/driver"
--- stderr -------------------------------
error[E0432]: unresolved import `rustc_middle::ty::query::query_values`
  --> driver.rs:27:30
Build completed unsuccessfully in 0:30:45
Build completed unsuccessfully in 0:30:45
   |
27 | use rustc_middle::ty::query::query_values::mir_borrowck;
   |                              ^^^^^^^^^^^^ could not find `query_values` in `query`
error[E0603]: struct `ExternProviders` is private
  --> driver.rs:28:31
   |
   |
28 | use rustc_middle::ty::query::{ExternProviders, Providers};
   |                               ^^^^^^^^^^^^^^^ private struct
note: the struct `ExternProviders` is defined here
  --> /checkout/compiler/rustc_middle/src/ty/query.rs:7:21
   |
   |
7  |     DynamicQueries, ExternProviders, Providers, QueryArenas, QueryCaches, QueryEngine, QueryStates,

error[E0603]: struct `Providers` is private
  --> driver.rs:28:48
   |
   |
28 | use rustc_middle::ty::query::{ExternProviders, Providers};
   |                                                ^^^^^^^^^ private struct
note: the struct `Providers` is defined here
  --> /checkout/compiler/rustc_middle/src/ty/query.rs:7:38
   |
   |
7  |     DynamicQueries, ExternProviders, Providers, QueryArenas, QueryCaches, QueryEngine, QueryStates,

warning: ignoring --out-dir flag due to -o flag

error: aborting due to 3 previous errors; 1 warning emitted
