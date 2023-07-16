
[00:49:28] [ 91%] Linking CXX executable ../../bin/llvm-ar
[00:49:28] CMakeFiles/llvm-ar.dir/llvm-ar.cpp.o: In function `llvm::cl::opt<(anonymous namespace)::Format, false, llvm::cl::parser<(anonymous namespace)::Format> >::printOptionInfo(unsigned int) const':
[00:49:28] llvm-ar.cpp:(.text._ZNK4llvm2cl3optIN12_GLOBAL__N_16FormatELb0ENS0_6parserIS3_EEE15printOptionInfoEj+0x1d): undefined reference to `llvm::cl::generic_parser_base::printOptionInfo(llvm::cl::Option const&, unsigned int) const'
[00:49:28] CMakeFiles/llvm-ar.dir/llvm-ar.cpp.o: In function `llvm::cl::opt<(anonymous namespace)::Format, false, llvm::cl::parser<(anonymous namespace)::Format> >::getOptionWidth() const':
[00:49:28] llvm-ar.cpp:(.text._ZNK4llvm2cl3optIN12_GLOBAL__N_16FormatELb0ENS0_6parserIS3_EEE14getOptionWidthEv+0x19): undefined reference to `llvm::cl::generic_parser_base::getOptionWidth(llvm::cl::Option const&) const'
...
[00:49:30] llvm-config.cpp:(.text.startup.main+0x3049): undefined reference to `llvm::raw_ostream::write(char const*, unsigned int)'
[00:49:30] llvm-config.cpp:(.text.startup.main+0x305a): undefined reference to `llvm::raw_ostream::write(char const*, unsigned int)'
[00:49:30] llvm-config.cpp:(.text.startup.main+0x3077): undefined reference to `llvm::raw_ostream::write(char const*, unsigned int)'
[00:49:30] llvm-config.cpp:(.text.startup.main+0x3179): undefined reference to `llvm::raw_ostream::write(char const*, unsigned int)'
[00:49:30] collect2: error: ld returned 1 exit status
[00:49:30] tools/llvm-config/CMakeFiles/llvm-config.dir/build.make:96: recipe for target 'bin/llvm-config' failed
[00:49:30] make[2]: *** [bin/llvm-config] Error 1
[00:49:30] CMakeFiles/Makefile2:11805: recipe for target 'tools/llvm-config/CMakeFiles/llvm-config.dir/all' failed
[00:49:30] make[1]: *** [tools/llvm-config/CMakeFiles/llvm-config.dir/all] Error 2
[00:49:30] Makefile:149: recipe for target 'all' failed
[00:49:30] make: *** [all] Error 2
[00:49:30] thread 'main' panicked at '
[00:49:30] command did not execute successfully, got: exit code: 2
