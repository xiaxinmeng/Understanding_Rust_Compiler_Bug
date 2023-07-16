plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMwBAN2ANl4S
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
[  0%] Built target RTLSanCommon.osx
[ 14%] Built target RTSanitizerCommonLibc.osx
Consolidate compiler generated dependencies of target RTSanitizerCommonCoverage.osx
[ 71%] Built target RTSanitizerCommon.osx
make[3]: *** No rule to make target `lib/sanitizer_common/K��)�o��M�Q�'��/���X�X�=%���/^K�', needed by `lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/sanitizer_symbolizer_posix_libcdep.cpp.o'.  Stop.
Consolidate compiler generated dependencies of target RTSanitizerCommonSymbolizer.osx
make[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/all] Error 2
[ 71%] Built target RTSanitizerCommonCoverage.osx
make[2]: *** Waiting for unfinished jobs....
Consolidate compiler generated dependencies of target RTInterception.osx
make[1]: *** [lib/lsan/CMakeFiles/clang_rt.lsan_osx_dynamic.dir/rule] Error 2
[ 71%] Built target RTInterception.osx
make: *** [clang_rt.lsan_osx_dynamic] Error 2
thread 'main' panicked at '
Build completed unsuccessfully in 0:42:58
command did not execute successfully, got: exit status: 2

