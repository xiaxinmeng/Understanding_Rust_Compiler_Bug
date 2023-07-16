
commit ea7b20d8f279ea8c63b9a4b8e9129fce0e3c2b5d
Author: comex <comexk@gmail.com>
Date:   Tue Jan 28 00:05:33 2014 -0500

    Add appropriate LLVM module flags for debug info.

    Set "Dwarf Version" to 2 on OS X to avoid toolchain incompatibility, and
    set "Debug Info Version" to prevent debug info from being stripped from
    bitcode.

    Fixes #11352.
