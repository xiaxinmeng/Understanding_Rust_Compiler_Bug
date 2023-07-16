
[00:17:25] Scanning dependencies of target obj2yaml
[00:17:25] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/obj2yaml.cpp.o
[00:17:26] [ 98%] Linking CXX executable ../../bin/llvm-symbolizer
[00:17:26] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/coff2yaml.cpp.o
[00:17:26] clang: error: unable to execute command: Segmentation fault: 11
[00:17:26] clang: error: linker command failed due to signal (use -v to see invocation)
[00:17:26] make[3]: *** [bin/llvm-symbolizer] Error 254
[00:17:26] make[2]: *** [tools/llvm-symbolizer/CMakeFiles/llvm-symbolizer.dir/all] Error 2
[00:17:26] make[2]: *** Waiting for unfinished jobs....
[00:17:26] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/dwarf2yaml.cpp.o
[00:17:26] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/elf2yaml.cpp.o
[00:17:26] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/macho2yaml.cpp.o
[00:17:26] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/Error.cpp.o
[00:17:27] [ 98%] Linking CXX executable ../../bin/obj2yaml
[00:17:27] [ 98%] Built target obj2yaml
[00:17:27] make[1]: *** [all] Error 2
[00:17:27] thread 'main' panicked at '
[00:17:27] command did not execute successfully, got: exit code: 2
[00:17:27] 
[00:17:27] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.26/src/lib.rs:599:4
[00:17:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
