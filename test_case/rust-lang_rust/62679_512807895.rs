plain
2019-07-18T12:18:01.0202751Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T12:18:01.0203247Z 
2019-07-18T12:18:01.0203541Z   git checkout -b <new-branch-name>
2019-07-18T12:18:01.0203697Z 
2019-07-18T12:18:01.0203882Z HEAD is now at e8e0c666b Auto merge of #62679 - Xanewok:after-expansion, r=Zoxc
2019-07-18T12:18:01.0569364Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T12:18:01.0691851Z ==============================================================================
2019-07-18T12:18:01.0691925Z Task         : Bash
2019-07-18T12:18:01.0691998Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T13:07:24.6996374Z [551/2435] Linking CXX executable bin\llvm-tblgen.exe
2019-07-18T13:07:24.7449352Z [552/2435] Building Options.inc...
2019-07-18T13:07:24.7618049Z [553/2435] Building Options.inc...
2019-07-18T13:07:25.0875071Z [554/2435] Building CXX object lib\ToolDrivers\llvm-lib\CMakeFiles\LLVMLibDriver.dir\LibDriver.cpp.obj
2019-07-18T13:07:25.0875847Z FAILED: lib/ToolDrivers/llvm-lib/CMakeFiles/LLVMLibDriver.dir/LibDriver.cpp.obj 
2019-07-18T13:07:25.0876541Z D:\a\1\s\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_DEBUG_POINTER_IMPL="" -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\ToolDrivers\llvm-lib -ID:\a\1\s\src\llvm-project\llvm\lib\ToolDrivers\llvm-lib -Iinclude -ID:\a\1\s\src\llvm-project\llvm\include /nologo /MT /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /MT /O2 /Ob2   -UNDEBUG  /EHs-c- /GR- /showIncludes /Folib\ToolDrivers\llvm-lib\CMakeFiles\LLVMLibDriver.dir\LibDriver.cpp.obj /Fdlib\ToolDrivers\llvm-lib\CMakeFiles\LLVMLibDriver.dir\LLVMLibDriver.pdb -c D:\a\1\s\src\llvm-project\llvm\lib\ToolDrivers\llvm-lib\LibDriver.cpp
2019-07-18T13:07:25.0877227Z clang-cl.exe: warning: argument unused during compilation: '-mno-incremental-linker-compatible' [-Wunused-command-line-argument]
2019-07-18T13:07:25.0877455Z In file included from D:\a\1\s\src\llvm-project\llvm\lib\ToolDrivers\llvm-lib\LibDriver.cpp:18:
2019-07-18T13:07:25.0877686Z In file included from D:\a\1\s\src\llvm-project\llvm\include\llvm/Bitcode/BitcodeReader.h:19:
2019-07-18T13:07:25.0878009Z In file included from D:\a\1\s\src\llvm-project\llvm\include\llvm/IR/ModuleSummaryIndex.h:27:
2019-07-18T13:07:25.0878155Z In file included from D:\a\1\s\src\llvm-project\llvm\include\llvm/IR/Module.h:23:
2019-07-18T13:07:25.0878301Z D:\a\1\s\src\llvm-project\llvm\include\llvm/IR/Attributes.h(73,14):  fatal error: 'llvm/IR/Attributes.inc' file not found
2019-07-18T13:07:25.0878387Z     #include "llvm/IR/Attributes.inc"
2019-07-18T13:07:25.0878720Z 1 error generated.
2019-07-18T13:07:25.1652973Z [555/2435] Building CXX object lib\ToolDrivers\llvm-dlltool\CMakeFiles\LLVMDlltoolDriver.dir\DlltoolDriver.cpp.obj
2019-07-18T13:07:25.1652973Z [555/2435] Building CXX object lib\ToolDrivers\llvm-dlltool\CMakeFiles\LLVMDlltoolDriver.dir\DlltoolDriver.cpp.obj
2019-07-18T13:07:25.1655529Z ninja: build stopped: subcommand failed.
2019-07-18T13:07:25.1760604Z command did not execute successfully, got: exit code: 1
2019-07-18T13:07:25.1760645Z 
2019-07-18T13:07:25.1760645Z 
2019-07-18T13:07:25.1760718Z build script failed, must exit now', C:\Program Files\Rust\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.38\src\lib.rs:813:5
2019-07-18T13:07:25.1765696Z  finished in 175.807
2019-07-18T13:07:25.1804949Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/tools/cargotest src/tools/cargo
2019-07-18T13:07:25.1805066Z Build completed unsuccessfully in 0:37:38
2019-07-18T13:07:25.1805066Z Build completed unsuccessfully in 0:37:38
2019-07-18T13:07:25.2576867Z ##[error]Bash exited with code '1'.
2019-07-18T13:07:25.3017895Z ##[section]Starting: Upload CPU usage statistics
2019-07-18T13:07:25.3119827Z ==============================================================================
2019-07-18T13:07:25.3119902Z Task         : Bash
2019-07-18T13:07:25.3119975Z Description  : Run a Bash script on macOS, Linux, or Windows
