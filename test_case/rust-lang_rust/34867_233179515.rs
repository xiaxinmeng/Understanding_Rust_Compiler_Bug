
CMake Error at /usr/lib/llvm-3.8/build/share/llvm/cmake/LLVMExports.cmake:1034 (message):
  The imported target "LLVMSupport" references the file

     "/usr/lib/llvm-3.8/build/lib/libLLVMSupport.a"

  but this file does not exist.  Possible reasons include:

  * The file was deleted, renamed, or moved to another location.

  * An install or uninstall procedure did not complete successfully.

  * The installation package was faulty and contained

     "/usr/lib/llvm-3.8/build/share/llvm/cmake/LLVMExports.cmake"

  but not all the files it references.

Call Stack (most recent call first):
  /usr/lib/llvm-3.8/build/share/llvm/cmake/LLVMConfig.cmake:178 (include)
  CMakeLists.txt:113 (include)
