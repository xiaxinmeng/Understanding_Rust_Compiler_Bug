plain
---- [run-make] tests/run-make/extern-fn-explicit-align stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/extern-fn-explicit-align" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC" CXX="c++ -ffunction-sections -fdata-sections -fPIC" HOST_RPATH_DIR="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-aarch64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="aarch64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align" "make"
--- stdout -------------------------------
cc -ffunction-sections -fdata-sections -fPIC -v -c -o /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o test.c
ar crus /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.a /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align  test.rs
rm /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o
--- stderr -------------------------------
Build completed unsuccessfully in 0:45:53
Using built-in specs.
Using built-in specs.
COLLECT_GCC=cc
Target: aarch64-linux-gnu
Configured with: ../src/configure -v --with-pkgversion='Ubuntu 9.4.0-1ubuntu1~20.04.1' --with-bugurl=file:///usr/share/doc/gcc-9/README.Bugs --enable-languages=c,ada,c++,go,d,fortran,objc,obj-c++,gm2 --prefix=/usr --with-gcc-major-version-only --program-suffix=-9 --program-prefix=aarch64-linux-gnu- --enable-shared --enable-linker-build-id --libexecdir=/usr/lib --without-included-gettext --enable-threads=posix --libdir=/usr/lib --enable-nls --enable-clocale=gnu --enable-libstdcxx-debug --enable-libstdcxx-time=yes --with-default-libstdcxx-abi=new --enable-gnu-unique-object --disable-libquadmath --disable-libquadmath-support --enable-plugin --enable-default-pie --with-system-zlib --with-target-system-zlib=auto --enable-objc-gc=auto --enable-multiarch --enable-fix-cortex-a53-843419 --disable-werror --enable-checking=release --build=aarch64-linux-gnu --host=aarch64-linux-gnu --target=aarch64-linux-gnu
Thread model: posix
gcc version 9.4.0 (Ubuntu 9.4.0-1ubuntu1~20.04.1) 
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-fPIC' '-v' '-c' '-o' '/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o' '-mlittle-endian' '-mabi=lp64'
 /usr/lib/gcc/aarch64-linux-gnu/9/cc1 -quiet -v -imultiarch aarch64-linux-gnu test.c -quiet -dumpbase test.c -mlittle-endian -mabi=lp64 -auxbase-strip /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o -version -ffunction-sections -fdata-sections -fPIC -fasynchronous-unwind-tables -fstack-protector-strong -Wformat -Wformat-security -fstack-clash-protection -o /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/ccG3IfBf.s
GNU C17 (Ubuntu 9.4.0-1ubuntu1~20.04.1) version 9.4.0 (aarch64-linux-gnu)
 compiled by GNU C version 9.4.0, GMP version 6.2.0, MPFR version 4.0.2, MPC version 1.1.0, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
ignoring nonexistent directory "/usr/local/include/aarch64-linux-gnu"
ignoring nonexistent directory "/usr/lib/gcc/aarch64-linux-gnu/9/include-fixed"
ignoring nonexistent directory "/usr/lib/gcc/aarch64-linux-gnu/9/../../../../aarch64-linux-gnu/include"
#include "..." search starts here:
#include <...> search starts here:
 /usr/lib/gcc/aarch64-linux-gnu/9/include
 /usr/include/aarch64-linux-gnu
 /usr/include
End of search list.
End of search list.
GNU C17 (Ubuntu 9.4.0-1ubuntu1~20.04.1) version 9.4.0 (aarch64-linux-gnu)
 compiled by GNU C version 9.4.0, GMP version 6.2.0, MPFR version 4.0.2, MPC version 1.1.0, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: 6a3864a8c3fe8bbb972fb5dbcb1f67d4
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-fPIC' '-v' '-c' '-o' '/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o' '-mlittle-endian' '-mabi=lp64'
 as -v -EL -mabi=lp64 -o /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/ccG3IfBf.s
GNU assembler version 2.34 (aarch64-linux-gnu) using BFD version (GNU Binutils for Ubuntu) 2.34
COMPILER_PATH=/usr/lib/gcc/aarch64-linux-gnu/9/:/usr/lib/gcc/aarch64-linux-gnu/9/:/usr/lib/gcc/aarch64-linux-gnu/:/usr/lib/gcc/aarch64-linux-gnu/9/:/usr/lib/gcc/aarch64-linux-gnu/
LIBRARY_PATH=/usr/lib/gcc/aarch64-linux-gnu/9/:/usr/lib/gcc/aarch64-linux-gnu/9/../../../aarch64-linux-gnu/:/usr/lib/gcc/aarch64-linux-gnu/9/../../../../lib/:/lib/aarch64-linux-gnu/:/lib/../lib/:/usr/lib/aarch64-linux-gnu/:/usr/lib/../lib/:/usr/lib/gcc/aarch64-linux-gnu/9/../../../:/lib/:/usr/lib/
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-fPIC' '-v' '-c' '-o' '/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/extern-fn-explicit-align/extern-fn-explicit-align/libtest.o' '-mlittle-endian' '-mabi=lp64'
error[E0308]: mismatched types
  --> test.rs:58:13
   |
45 |         many_args(
45 |         many_args(
   |         --------- arguments to this function are incorrect
...
58 |             string.as_ptr(),
   |             ^^^^^^^^^^^^^^^ expected `*const i8`, found `*const u8`
   |
   = note: expected raw pointer `*const i8`
              found raw pointer `*const u8`
  --> test.rs:23:8
   |
23 |     fn many_args(
   |        ^^^^^^^^^
