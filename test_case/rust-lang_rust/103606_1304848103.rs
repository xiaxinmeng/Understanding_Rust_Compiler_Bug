
CMake Warning (dev) at /usr/share/cmake/Modules/GNUInstallDirs.cmake:239 (message):
  Unable to determine default CMAKE_INSTALL_LIBDIR directory because no
  target architecture is known.  Please enable at least one language before
  including GNUInstallDirs.
Call Stack (most recent call first):
  /home/keshlam/fork/rust-103606/src/llvm-project/llvm/cmake/modules/LLVMInstallSymlink.cmake:5 (include)
  tools/llvm-ar/cmake_install.cmake:56 (include)
  tools/cmake_install.cmake:49 (include)
  cmake_install.cmake:78 (include)
This warning is for project developers.  Use -Wno-dev to suppress it.
