plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMV7bBCJrzIB
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
Consolidate compiler generated dependencies of target RTSanitizerCommonCoverage.osx
[ 14%] Built target RTSanitizerCommonCoverage.osx
[ 71%] Built target RTSanitizerCommon.osx
Consolidate compiler generated dependencies of target RTSanitizerCommonSymbolizer.osx
lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/compiler_depend.make:976: *** missing separator.  Stop.
Consolidate compiler generated dependencies of target RTInterception.osx
make[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/all] Error 2
make[2]: *** Waiting for unfinished jobs....
make[1]: *** [lib/lsan/CMakeFiles/clang_rt.lsan_osx_dynamic.dir/rule] Error 2
[ 71%] Built target RTInterception.osx
make: *** [clang_rt.lsan_osx_dynamic] Error 2
thread 'main' panicked at '
Build completed unsuccessfully in 0:50:22
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
