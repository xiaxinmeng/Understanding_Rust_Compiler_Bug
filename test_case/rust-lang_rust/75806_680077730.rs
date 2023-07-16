plain
-- Build files have been written to: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[1/2684] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ABIBreak.cpp.obj
[2/2684] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\Demangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/Demangle.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG  /EHs-c- /GR- -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\Demangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\Demangle.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\Demangle.cpp:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\cstddef:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[3/2684] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG  /EHs-c- /GR- -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangleNodes.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangleNodes.cpp:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangleNodes.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/StringView.h:17:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\algorithm:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[4/2684] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG  /EHs-c- /GR- -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangle.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangle.cpp:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangle.h:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangleNodes.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/StringView.h:17:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\algorithm:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[5/2684] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\ItaniumDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG  /EHs-c- /GR- -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\ItaniumDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\ItaniumDemangle.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\ItaniumDemangle.cpp:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\cstddef:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[6/2684] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ARMTargetParser.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/ARMTargetParser.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/ARMTargetParser.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\ARMTargetParser.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Support\ARMTargetParser.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\ARMTargetParser.cpp:14:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/ARMTargetParser.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/SmallVector.h:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/iterator_range.h:21:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\iterator:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[7/2684] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\AArch64TargetParser.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AArch64TargetParser.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AArch64TargetParser.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\AArch64TargetParser.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AArch64TargetParser.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AArch64TargetParser.cpp:14:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/AArch64TargetParser.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/SmallVector.h:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/iterator_range.h:21:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\iterator:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[8/2684] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\APFloat.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/APFloat.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/APFloat.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\APFloat.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Support\APFloat.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\APFloat.cpp:14:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/APFloat.h:19:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/APInt.h:18:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/Compiler.h:21:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\new:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
[9/2684] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\AMDGPUMetadata.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wno-noexcept-type -Wno-comment /Gw /MT /O2 /Ob2 /DNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\AMDGPUMetadata.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AMDGPUMetadata.cpp
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
clang-cl: warning: argument unused during compilation: '-Brepro' [-Wunused-command-line-argument]
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AMDGPUMetadata.cpp:15:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/Twine.h:12:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/SmallVector.h:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/iterator_range.h:21:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\iterator:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.27.29110\include\yvals_core.h(494,2): error: STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 10.0.0 or newer.
1 error generated.
1 error generated.
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
 finished in 42.748
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap --stage 2 test src/tools/cargotest src/tools/cargo
Build completed unsuccessfully in 0:02:06
== clock drift check ==
== clock drift check ==
  local time: Tue Aug 25 14:58:04 CUT 2020
  network time: Tue, 25 Aug 2020 14:58:04 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (7060) (node)
Terminate orphan process: pid (1292) (python)
Terminate orphan process: pid (1188) (sccache)
