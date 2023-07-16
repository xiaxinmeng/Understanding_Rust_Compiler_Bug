plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMbWJDsMqNFW

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[ 20%] Built target RTLSanCommon.osx
[ 40%] Built target RTSanitizerCommonSymbolizer.osx
Consolidate compiler generated dependencies of target RTSanitizerCommonLibc.osx
Consolidate compiler generated dependencies of target RTInterception.osx
lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.osx.dir/compiler_depend.make:1061: *** multiple target patterns.  Stop.
Consolidate compiler generated dependencies of target RTSanitizerCommon.osx
[ 50%] Built target RTInterception.osx
[ 50%] Built target RTInterception.osx
make[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.osx.dir/all] Error 2
make[2]: *** Waiting for unfinished jobs....
 finished in 77.127 seconds
make[1]: *** [lib/lsan/CMakeFiles/clang_rt.lsan_osx_dynamic.dir/rule] Error 2
Build completed unsuccessfully in 0:43:47
make: *** [clang_rt.lsan_osx_dynamic] Error 2
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
