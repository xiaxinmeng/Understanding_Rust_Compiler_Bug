plain
-- Build files have been written to: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[1/2794] Generating VCSRevision.h
[2/2794] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\Demangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/Demangle.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\Demangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\Demangle.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\Demangle.cpp:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\cstddef:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
[3/2794] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangleNodes.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangleNodes.cpp:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangleNodes.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/StringView.h:17:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\algorithm:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
[4/2794] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangle.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangle.cpp:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangle.h:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangleNodes.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/StringView.h:17:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\algorithm:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
[5/2794] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\ItaniumDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Demangle -ID:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\ItaniumDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\ItaniumDemangle.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\ItaniumDemangle.cpp:13:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\cstddef:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
[6/2794] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ARMTargetParser.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/ARMTargetParser.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/ARMTargetParser.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2 -UNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\ARMTargetParser.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Support\ARMTargetParser.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\ARMTargetParser.cpp:14:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/ARMTargetParser.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/SmallVector.h:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/iterator_range.h:21:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\utility:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
[7/2794] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\AArch64TargetParser.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AArch64TargetParser.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AArch64TargetParser.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2 -UNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\AArch64TargetParser.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AArch64TargetParser.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AArch64TargetParser.cpp:14:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/AArch64TargetParser.h:17:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/SmallVector.h:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/iterator_range.h:21:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\utility:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
[8/2794] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ABIBreak.cpp.obj
[9/2794] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\AMDGPUMetadata.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/AMDGPUMetadata.cpp.obj 
D:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_HAS_EXCEPTIONS=0 -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib\Support -ID:\a\rust\rust\src\llvm-project\llvm\lib\Support -Iinclude -ID:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=x86_64-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Zc:strictStrings /Oi /Zc:rvalueCast /Brepro /bigobj /W4  -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wstring-conversion /Gw /MT /O2 /Ob2 -UNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\AMDGPUMetadata.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c -- D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AMDGPUMetadata.cpp
In file included from D:\a\rust\rust\src\llvm-project\llvm\lib\Support\AMDGPUMetadata.cpp:15:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/AMDGPUMetadata.h:18:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/StringRef.h:12:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/STLExtras.h:19:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/Optional.h:18:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/Hashing.h:47:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/DataTypes.h:16:
In file included from D:\a\rust\rust\src\llvm-project\llvm\include\llvm-c/DataTypes.h:53:
In file included from C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\cstddef:9:
C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\include\yvals_core.h(537,2): error: STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
#error STL1000: Unexpected compiler version, expected Clang 11.0.0 or newer.
1 error generated.
1 error generated.
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
 finished in 55.211 seconds
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap --stage 2 test src/tools/cargotest src/tools/cargo
Build completed unsuccessfully in 0:02:26
