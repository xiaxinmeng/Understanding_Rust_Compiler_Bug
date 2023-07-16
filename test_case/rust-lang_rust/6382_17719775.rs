
compile: rustllvm/i686-pc-mingw32/RustWrapper.o
cc1plus.exe: warnings being treated as errors
In file included from C:/MinGW/msys/1.0/home/Thad/rust/src/rustllvm/RustWrapper.
cpp:43:0:
C:/MinGW/msys/1.0/home/Thad/rust/src/llvm/include/llvm/ExecutionEngine/JITMemory
Manager.h: In member function 'virtual bool llvm::JITMemoryManager::CheckInvaria
nts(std::string&)':
C:/MinGW/msys/1.0/home/Thad/rust/src/llvm/include/llvm/ExecutionEngine/JITMemory
Manager.h:139:3: error: visibility attribute not supported in this configuration
; ignored
make: *** [rustllvm/i686-pc-mingw32/RustWrapper.o] Error 1
