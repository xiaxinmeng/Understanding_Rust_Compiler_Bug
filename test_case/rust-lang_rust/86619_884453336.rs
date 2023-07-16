plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMqUj3dBSPC6

hw.ncpu: 3
hw.byteorder: 1234
+ ./x.py --stage 2 test
---
[ 26%] Building X86GenCallingConv.inc...
[ 26%] Building X86GenDAGISel.inc...
[ 26%] Building CXX object tools/llvm-cxxmap/CMakeFiles/llvm-cxxmap.dir/llvm-cxxmap.cpp.o
error: Connection to server timed out
make[2]: *** [tools/llvm-cxxmap/CMakeFiles/llvm-cxxmap.dir/llvm-cxxmap.cpp.o] Error 2
make[1]: *** [tools/llvm-cxxmap/CMakeFiles/llvm-cxxmap.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[ 26%] Building X86GenDisassemblerTables.inc...
[ 26%] Building RISCVGenSearchableTables.inc...
[ 26%] Building X86GenEVEX2VEXTables.inc...
[ 26%] Building RISCVGenSubtargetInfo.inc...
---
[ 27%] Building X86GenRegisterBank.inc...
[ 27%] Building X86GenRegisterInfo.inc...
[ 27%] Building X86GenSubtargetInfo.inc...
[ 27%] Built target X86CommonTableGen
make: *** [all] Error 2
command did not execute successfully, got: exit status: 2
 finished in 2929.849 seconds

Build completed unsuccessfully in 1:48:21
Build completed unsuccessfully in 1:48:21
build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
