plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMaatLaNvkNO

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
-- Build files have been written to: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/sanitizers/build
running: "cmake" "--build" "." "--target" "clang_rt.lsan_osx_dynamic" "--config" "Release" "--" "-j" "3"
Consolidate compiler generated dependencies of target RTSanitizerCommonCoverage.osx
Consolidate compiler generated dependencies of target RTLSanCommon.osx
make[3]: *** No rule to make target `/�h�jF�%"��ն��S���3Ѧ�~�K�-x��=8Gu��'L�F���6�ƫǜ�8y*ٿ�$#�N��x%W��Uo7��o55_��]wG𰚺Np���Np�', needed by `lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/sanitizer_symbolizer_mac.cpp.o'.  Stop.
[  0%] Built target RTSanitizerCommonCoverage.osx
[  0%] Built target RTSanitizerCommonCoverage.osx
make[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/all] Error 2
[ 11%] Built target RTLSanCommon.osx
make[2]: *** Waiting for unfinished jobs....
Consolidate compiler generated dependencies of target RTSanitizerCommonLibc.osx
make[1]: *** [lib/lsan/CMakeFiles/clang_rt.lsan_osx_dynamic.dir/rule] Error 2
[ 11%] Built target RTSanitizerCommonLibc.osx
make: *** [clang_rt.lsan_osx_dynamic] Error 2
thread 'main' panicked at '
Build completed unsuccessfully in 0:43:29
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
