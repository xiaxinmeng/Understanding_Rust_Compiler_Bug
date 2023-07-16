plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:917222de331afc95ef8d3a6300048017039b2b08)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

running 230 tests
F...........i.....ii....................................................................  88/230
.....i.........................i...................iiiiiii.......i...................iii 176/230
........F................F......i...ii................
failures:

---- [run-make] tests/run-make-fulldeps/atomic-lock-free stdout ----


error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make-fulldeps/atomic-lock-free" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=i686-unknown-linux-gnu atomic_lock_free.rs
--- stderr -------------------------------
error: requires `callable` lang_item
  --> atomic_lock_free.rs:20:5
   |
   |
20 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:25:5
   |
   |
25 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:29:5
   |
   |
29 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:33:5
   |
   |
33 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:37:5
   |
   |
37 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:41:5
   |
   |
41 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:45:5
   |
   |
45 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:49:5
   |
   |
49 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:61:5
   |
   |
61 |     atomic_xadd_seqcst(x, 1);

error: requires `callable` lang_item
  --> atomic_lock_free.rs:65:5
   |
   |
65 |     atomic_xadd_seqcst(x, 1);

error: aborting due to 10 previous errors

make: *** [Makefile:9: all] Error 1
make: *** [Makefile:9: all] Error 1
------------------------------------------


---- [run-make] tests/run-make-fulldeps/simd-ffi stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make-fulldeps/simd-ffi" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-linux-androideabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-linux-androideabi
--- stderr -------------------------------
--- stderr -------------------------------
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
warning: type `f32x4` should have an upper camel case name
  --> simd.rs:11:12
   |
11 | pub struct f32x4(f32, f32, f32, f32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `F32x4`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `i32x4` should have an upper camel case name
  --> simd.rs:24:12
24 | pub struct i32x4(i32, i32, i32, i32);
24 | pub struct i32x4(i32, i32, i32, i32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `I32x4`
error: requires `callable` lang_item
  --> simd.rs:19:14
   |
   |
19 |     unsafe { vsqrt(x) }

error: requires `callable` lang_item
  --> simd.rs:54:14
   |
   |
54 |     unsafe { integer(a, b) }

error: aborting due to 2 previous errors; 2 warnings emitted


make: *** [Makefile:47: arm-linux-androideabi] Error 1


---- [run-make] tests/run-make-fulldeps/target-specs stdout ----


error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make-fulldeps/target-specs" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-awesome-platform.json --crate-type=lib --emit=asm
--- stderr -------------------------------
--- stderr -------------------------------
warning: target json file contains unused fields: morestack
error: requires `callable` lang_item
  --> foo.rs:23:23
   |
   |
23 |     let _a = unsafe { _foo() };

error: aborting due to previous error

make: *** [Makefile:3: all] Error 1
