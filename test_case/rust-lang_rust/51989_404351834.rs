
[00:23:48] [249/2241] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.obj
[00:23:48] FAILED: lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.obj 
[00:23:48] C:\projects\rust\sccache.exe g++.exe  -DGTEST_HAS_RTTI=0 -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib/IR -IC:/projects/rust/src/llvm/lib/IR -Iinclude -IC:/projects/rust/src/llvm/include -ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer -Werror=date-time -std=gnu++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment  -O2 -DNDEBUG    -fno-exceptions -fno-rtti -MD -MT lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.obj -MF lib\IR\CMakeFiles\LLVMCore.dir\Globals.cpp.obj.d -o lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.obj -c C:/projects/rust/src/llvm/lib/IR/Globals.cpp
[00:23:48] C:/projects/rust/src/llvm/lib/IR/Globals.cpp:37:35: error: static assertion failed: unexpected GlobalValue size growth
[00:23:48]  static_assert(sizeof(GlobalValue) ==
[00:23:48]                ~~~~~~~~~~~~~~~~~~~~^~
[00:23:48]                    sizeof(Constant) + 2 * sizeof(void *) + 2 * sizeof(unsigned),
[00:23:48]                    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
