plain
2019-07-24T22:00:39.7155013Z [551/2435] Linking CXX executable bin\llvm-tblgen.exe
2019-07-24T22:00:39.8882652Z [552/2435] Building Options.inc...
2019-07-24T22:00:39.8981288Z [553/2435] Building Options.inc...
2019-07-24T22:00:40.2638159Z [554/2435] Building CXX object lib\ToolDrivers\llvm-lib\CMakeFiles\LLVMLibDriver.dir\LibDriver.cpp.obj
2019-07-24T22:00:40.2638607Z FAILED: lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj 
2019-07-24T22:00:40.2639133Z D:\a\1\s\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_DEBUG_POINTER_IMPL="" -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\ToolDrivers\llvm-lib -ID:\a\1\s\src\llvm-project\llvm\lib\ToolDrivers\llvm-lib -Iinclude -ID:\a\1\s\src\llvm-project\llvm\include /nologo /MT /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /MT /O2 /Ob2   -UNDEBUG  /EHs-c- /GR- /showIncludes /Folib\ToolDrivers\llvm-lib\CMakeFiles\LLVMLibDriver.dir\LibDriver.cpp.obj /Fdlib\ToolDrivers\llvm-lib\CMakeFiles\LLVMLibDriver.dir\LLVMLibDriver.pdb -c D:\a\1\s\src\llvm-project\llvm\lib\ToolDrivers\llvm-lib\LibDriver.cpp
2019-07-24T22:00:40.2639628Z clang-cl.exe: warning: argument unused during compilation: '-mno-incremental-linker-compatible' [-Wunused-command-line-argument]
2019-07-24T22:00:40.2639780Z In file included from D:\a\1\s\src\llvm-project\llvm\lib\ToolDrivers\llvm-lib\LibDriver.cpp:18:
2019-07-24T22:00:40.2639922Z In file included from D:\a\1\s\src\llvm-project\llvm\include\llvm/Bitcode/BitcodeReader.h:19:
2019-07-24T22:00:40.2640040Z In file included from D:\a\1\s\src\llvm-project\llvm\include\llvm/IR/ModuleSummaryIndex.h:27:
2019-07-24T22:00:40.2640189Z In file included from D:\a\1\s\src\llvm-project\llvm\include\llvm/IR/Module.h:23:
2019-07-24T22:00:40.2640317Z D:\a\1\s\src\llvm-project\llvm\include\llvm/IR/Attributes.h(73,14):  fatal error: 'llvm/IR/Attributes.inc' file not found
2019-07-24T22:00:40.2640438Z     #include "llvm/IR/Attributes.inc"
2019-07-24T22:00:40.2640620Z 1 error generated.
2019-07-24T22:00:40.3378436Z [555/2435] Building CXX object lib\ToolDrivers\llvm-dlltool\CMakeFiles\LLVMDlltoolDriver.dir\DlltoolDriver.cpp.obj
2019-07-24T22:00:40.3378436Z [555/2435] Building CXX object lib\ToolDrivers\llvm-dlltool\CMakeFiles\LLVMDlltoolDriver.dir\DlltoolDriver.cpp.obj
2019-07-24T22:00:40.3378636Z ninja: build stopped: subcommand failed.
2019-07-24T22:00:40.3476204Z command did not execute successfully, got: exit code: 1
2019-07-24T22:00:40.3476374Z 
2019-07-24T22:00:40.3476374Z 
2019-07-24T22:00:40.3476751Z build script failed, must exit now', C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.38\src\lib.rs:813:5
2019-07-24T22:00:40.3488962Z  finished in 187.706
2019-07-24T22:00:40.3488962Z  finished in 187.706
2019-07-24T22:00:40.3538479Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/run-pass --exclude src/test/compile-fail --exclude src/test/run-pass-fulldeps --exclude src/tools/linkchecker
2019-07-24T22:00:40.3538662Z Build completed unsuccessfully in 0:49:10
2019-07-24T22:00:40.3747306Z make: *** [Makefile:84: ci-subset-1] Error 1
2019-07-24T22:00:40.4031445Z ##[error]Bash exited with code '2'.
2019-07-24T22:00:40.4292437Z ##[section]Starting: Upload CPU usage statistics
2019-07-24T22:00:40.4419146Z ==============================================================================
2019-07-24T22:00:40.4419556Z Task         : Bash
2019-07-24T22:00:40.4419670Z Description  : Run a Bash script on macOS, Linux, or Windows
