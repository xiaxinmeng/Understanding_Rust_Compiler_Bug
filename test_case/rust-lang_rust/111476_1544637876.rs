plain
---- [run-make] tests/run-make/coverage-llvmir stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/coverage-llvmir" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC" CXX="c++ -ffunction-sections -fdata-sections -fPIC" HOST_RPATH_DIR="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-aarch64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="aarch64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/coverage-llvmir/coverage-llvmir" "make"
--- stdout -------------------------------
# Compile the test program with non-experimental coverage instrumentation, and generate LLVM IR
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/coverage-llvmir/coverage-llvmir:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/coverage-llvmir/coverage-llvmir -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/coverage-llvmir/coverage-llvmir  ../coverage-llvmir/testprog.rs \
  -Cinstrument-coverage \
  --emit=llvm-ir
cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make/coverage-llvmir/coverage-llvmir"/testprog.ll | \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" ../coverage-llvmir/filecheck.testprog.txt -check-prefixes=CHECK -DDEFINE_INTERNAL='define internal' -DCOMDAT_IF_SUPPORTED=', comdat' -DINSTR_PROF_DATA='__llvm_prf_data' -DINSTR_PROF_NAME='__llvm_prf_names' -DINSTR_PROF_CNTS='__llvm_prf_cnts' -DINSTR_PROF_VALS='__llvm_prf_vals' -DINSTR_PROF_VNODES='__llvm_prf_vnds' -DINSTR_PROF_COVMAP='__llvm_covmap' -DINSTR_PROF_COVFUN='__llvm_covfun' -DINSTR_PROF_ORDERFILE='__llvm_orderfile'
--- stderr -------------------------------
--- stderr -------------------------------
../coverage-llvmir/filecheck.testprog.txt:39:8: error: CHECK: expected string not found in input
CHECK: %pgocount = load i64, {{i64\*|ptr}}
Build completed unsuccessfully in 0:46:21
<stdin>:380:7: note: scanning from here
start:
      ^
      ^
<stdin>:400:51: note: possible intended match here
 %10 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_7, i64 0, i64 0

Input file: <stdin>
Input file: <stdin>
Check file: ../coverage-llvmir/filecheck.testprog.txt

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          375: } 
          376:  
          377: ; testprog::will_be_called 
          378: ; Function Attrs: nonlazybind uwtable 
          379: define internal { ptr, i64 } @_RNvCs2vQph7PjJMo_8testprog14will_be_called() unnamed_addr #1 { 
          380: start: 
check:39'0           X error: no match found
          381:  %0 = alloca { ptr, ptr }, align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          382:  %_7 = alloca [1 x { ptr, ptr }], align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          383:  %_3 = alloca %"core::fmt::Arguments<'_>", align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          384:  %_1 = alloca { ptr, i64 }, align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          385:  %1 = atomicrmw add ptr @__profc__RNvCs2vQph7PjJMo_8testprog14will_be_called, i64 1 monotonic, align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
          395:  %7 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          396:  %8 = insertvalue { ptr, ptr } poison, ptr %5, 0 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          397:  %9 = insertvalue { ptr, ptr } %8, ptr %7, 1 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          398:  %_8.0 = extractvalue { ptr, ptr } %9, 0 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          399:  %_8.1 = extractvalue { ptr, ptr } %9, 1 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          400:  %10 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_7, i64 0, i64 0 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'1                                                       ?                      possible intended match
          401:  %11 = getelementptr inbounds { ptr, ptr }, ptr %10, i32 0, i32 0 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          402:  store ptr %_8.0, ptr %11, align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          403:  %12 = getelementptr inbounds { ptr, ptr }, ptr %10, i32 0, i32 1 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          404:  store ptr %_8.1, ptr %12, align 8 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          405: ; call <core::fmt::Arguments>::new_v1 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
>>>>>>
make: *** [Makefile:60: test_llvm_ir] Error 1



failures:
