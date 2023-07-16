 bash
$ time RUST_LOG=rustc::metadata::loader make -j1 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z print-link-args -Z print-llvm-passes -C debug-assertions=y' RUST_BACKTRACE=1
cfg: version 1.5.0-dev (04e497c00 2015-10-24)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: enabling debuginfo (CFG_ENABLE_DEBUGINFO)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
...
"cc" "-Wl,--as-needed" "-m64" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib" "x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/rustc_llvm-bb943c5a.0.o" "-o" "x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.so" "x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/rustc_llvm-bb943c5a.metadata.o" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "std-bb943c5a" "-L" "x86_64-unknown-linux-gnu/rt" "-L" "/usr/lib64" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/.rust/lib/x86_64-unknown-linux-gnu" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/lib/x86_64-unknown-linux-gnu" "-Wl,-Bstatic" "-Wl,--whole-archive" "-l" "rustllvm" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMInterpreter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMCJIT" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMExecutionEngine" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMRuntimeDyld" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMAsmParser" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMLinker" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMipo" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMVectorize" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMProfileData" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Disassembler" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86AsmParser" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86CodeGen" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMSelectionDAG" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMAsmPrinter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMCodeGen" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMTarget" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMScalarOpts" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMInstCombine" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMInstrumentation" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMTransformUtils" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMBitWriter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMAnalysis" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Desc" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMObject" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMCParser" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMBitReader" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMCDisassembler" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Info" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86AsmPrinter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMC" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Utils" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMCore" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMSupport" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-l" "rt" "-l" "dl" "-l" "curses" "-l" "pthread" "-l" "z" "-l" "m" "-l" "stdc++" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "rt" "-l" "c" "-l" "m" "-shared" "-Wl,-rpath,$ORIGIN/" "-Wl,-rpath,/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "compiler-rt"
  time: 0.106; rss: 84MB    running linker
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-m64" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib" "x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/rustc_llvm-bb943c5a.0.o" "-o" "x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.so" "x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/rustc_llvm-bb943c5a.metadata.o" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "std-bb943c5a" "-L" "x86_64-unknown-linux-gnu/rt" "-L" "/usr/lib64" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/.rust/lib/x86_64-unknown-linux-gnu" "-L" "/home/zazdxscf/build/1nonpkgs/rust/rust/lib/x86_64-unknown-linux-gnu" "-Wl,-Bstatic" "-Wl,--whole-archive" "-l" "rustllvm" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMInterpreter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMCJIT" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMExecutionEngine" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMRuntimeDyld" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMAsmParser" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMLinker" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMipo" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMVectorize" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMProfileData" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Disassembler" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86AsmParser" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86CodeGen" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMSelectionDAG" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMAsmPrinter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMCodeGen" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMTarget" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMScalarOpts" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMInstCombine" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMInstrumentation" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMTransformUtils" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMBitWriter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMAnalysis" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Desc" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMObject" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMCParser" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMBitReader" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMCDisassembler" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Info" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86AsmPrinter" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMMC" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMX86Utils" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMCore" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "LLVMSupport" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-l" "rt" "-l" "dl" "-l" "curses" "-l" "pthread" "-l" "z" "-l" "m" "-l" "stdc++" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "rt" "-l" "c" "-l" "m" "-shared" "-Wl,-rpath,$ORIGIN/" "-Wl,-rpath,/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "compiler-rt"
note: /usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMInterpreter
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMMCJIT
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMExecutionEngine
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMRuntimeDyld
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMAsmParser
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMLinker
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMipo
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMVectorize
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMProfileData
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86Disassembler
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86AsmParser
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86CodeGen
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMSelectionDAG
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMAsmPrinter
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMCodeGen
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMTarget
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMScalarOpts
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMInstCombine
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMInstrumentation
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMTransformUtils
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMBitWriter
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMAnalysis
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86Desc
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMObject
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMMCParser
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMBitReader
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMMCDisassembler
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86Info
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86AsmPrinter
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMMC
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMX86Utils
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMCore
/usr/lib/gcc/x86_64-pc-linux-gnu/5.2.0/../../../../x86_64-pc-linux-gnu/bin/ld: cannot find -lLLVMSupport
collect2: error: ld returned 1 exit status

error: aborting due to previous error
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_llvm' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_llvm] Error 101

real    6m30.758s
user    6m12.400s
sys 0m14.680s

