
% ./build/x86_64-unknown-linux-gnu/llvm/bin/clang -c issue65844.c -mno-sse2
clang-9: /home/pnkfelix/Dev/Mozilla/rust.git/src/llvm-project/llvm/lib/Target/X86/X86FloatingPoint.cpp:317: unsigned int getFPReg(const llvm::MachineOperand&): Assertion `Reg >= X86::FP0 && Reg <= X86\
::FP6 && "Expected FP register!"' failed.
Stack dump:
[...]
clang-9: note: diagnostic msg: PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace, preprocessed source, and associated run script.
clang-9: note: diagnostic msg:
********************

PLEASE ATTACH THE FOLLOWING FILES TO THE BUG REPORT:
Preprocessed source(s) and associated run script(s) are located at:
clang-9: note: diagnostic msg: /tmp/issue65844-3b4b26.c
clang-9: note: diagnostic msg: /tmp/issue65844-3b4b26.sh
clang-9: note: diagnostic msg:

********************
[ERROR#1] %
