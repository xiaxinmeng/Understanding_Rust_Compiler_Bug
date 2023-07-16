
clang++ --target=arm-pc-windows-msvc -D_ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE -v -o test.exe clang_except_test.cpp
clang version 6.0.0 (tags/RELEASE_600/final)
Target: arm-pc-windows-msvc
Thread model: posix
InstalledDir: C:\Program Files\LLVM\bin
 "C:\\Program Files\\LLVM\\bin\\clang++.exe" -cc1 -triple thumbv7-pc-windows-msvc19.11.25547 -emit-obj -mrelax-all -mincremental-linker-compatible -disable-free -disable-llvm-verifier -discard-value-names -main-file-name clang_except_test.cpp -mrelocation-model static -mthread-model posix -mdisable-fp-elim -fmath-errno -masm-verbose -mconstructor-aliases -target-cpu cortex-a9 -target-feature +strict-align -target-abi aapcs -mfloat-abi hard -fallow-half-arguments-and-returns -dwarf-column-info -debugger-tuning=gdb -v -resource-dir "C:\\Program Files\\LLVM\\lib\\clang\\6.0.0" -D _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE -internal-isystem "C:\\Program Files\\LLVM\\lib\\clang\\6.0.0\\include" -internal-isystem "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.11.25503\\include" -internal-isystem "C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.17134.0\\ucrt" -internal-isystem "C:\\Program Files (x86)\\Windows Kits\\10\\include\\10.0.17134.0\\shared" -internal-isystem "C:\\Program Files (x86)\\Windows Kits\\10\\include\\10.0.17134.0\\um" -internal-isystem "C:\\Program Files (x86)\\Windows Kits\\10\\include\\10.0.17134.0\\winrt" -fdeprecated-macro -fdebug-compilation-dir "d:\\work\\rust" -ferror-limit 19 -fmessage-length 80 -backend-option -arm-restrict-it -fno-use-cxa-atexit -fms-extensions -fms-compatibility -fms-compatibility-version=19.11.25547 -std=c++14 -fdelayed-template-parsing -fobjc-runtime=gcc -fcxx-exceptions -fexceptions -fseh-exceptions -fdiagnostics-show-option -fcolor-diagnostics -o "C:\\Users\\jordanrh\\AppData\\Local\\Temp\\clang_except_test-8cb11c.o" -x c++ clang_except_test.cpp
clang -cc1 version 6.0.0 based upon LLVM 6.0.0 default target x86_64-pc-windows-msvc
#include "..." search starts here:
#include <...> search starts here:
 C:\Program Files\LLVM\lib\clang\6.0.0\include
 C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.11.25503\include
 C:\Program Files (x86)\Windows Kits\10\Include\10.0.17134.0\ucrt
 C:\Program Files (x86)\Windows Kits\10\include\10.0.17134.0\shared
 C:\Program Files (x86)\Windows Kits\10\include\10.0.17134.0\um
 C:\Program Files (x86)\Windows Kits\10\include\10.0.17134.0\winrt
End of search list.
fatal error: error in backend: Funclet EH is not implemented for this target
clang++.exe: error: clang frontend command failed with exit code 70 (use -v to see invocation)
clang version 6.0.0 (tags/RELEASE_600/final)
Target: arm-pc-windows-msvc
Thread model: posix
InstalledDir: C:\Program Files\LLVM\bin
clang++.exe: note: diagnostic msg: PLEASE submit a bug report to http://llvm.org/bugs/ and include the crash backtrace, preprocessed source, and associated run script.
clang++.exe: note: diagnostic msg:
********************

PLEASE ATTACH THE FOLLOWING FILES TO THE BUG REPORT:
Preprocessed source(s) and associated run script(s) are located at:
clang++.exe: note: diagnostic msg: C:\Users\jordanrh\AppData\Local\Temp\clang_except_test-c25954.cpp
clang++.exe: note: diagnostic msg: C:\Users\jordanrh\AppData\Local\Temp\clang_except_test-c25954.sh
clang++.exe: note: diagnostic msg:

********************
make: *** [Makefile:4: test.exe] Error 70
