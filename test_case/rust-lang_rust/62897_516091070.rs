plain
2019-07-29T14:50:20.3007452Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T14:50:20.3007600Z 
2019-07-29T14:50:20.3007743Z   git checkout -b <new-branch-name>
2019-07-29T14:50:20.3007863Z 
2019-07-29T14:50:20.3008042Z HEAD is now at a385138fe Auto merge of #62897 - alexcrichton:fix-i686-msvc-tests, r=pietroalbini
2019-07-29T14:50:20.3401336Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T14:50:20.3511699Z ==============================================================================
2019-07-29T14:50:20.3511775Z Task         : Bash
2019-07-29T14:50:20.3511848Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T14:55:57.8317243Z installing mingw-w64-i686-rhash...
2019-07-29T14:55:57.8923673Z installing mingw-w64-i686-cmake...
2019-07-29T14:56:06.3490311Z Optional dependencies for mingw-w64-i686-cmake
2019-07-29T14:56:06.3490577Z     mingw-w64-i686-qt5: CMake Qt GUI
2019-07-29T14:56:06.3490680Z     mingw-w64-i686-emacs: for cmake emacs mode
2019-07-29T14:56:12.9584117Z D:\a\1\s\citools\msys64\usr\bin\rev.exe
2019-07-29T14:56:13.3124735Z Usage: rev [options] [file ...]
2019-07-29T14:56:13.3124845Z 
2019-07-29T14:56:13.3125069Z Reverse lines characterwise.
---
2019-07-29T17:39:48.5864404Z ---- [ui] ui\backtrace.rs stdout ----
2019-07-29T17:39:48.5864533Z 
2019-07-29T17:39:48.5864677Z error: test run failed!
2019-07-29T17:39:48.5865005Z status: exit code: 101
2019-07-29T17:39:48.5866877Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\citools\msys64\mingw32\bin;D:\a\1\s\citools\msys64\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.154.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.38.0.13-8.0.212-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.3\x64\bin;C:\Go1.12.4\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\backtrace\\a.exe"
2019-07-29T17:39:48.5868497Z ------------------------------------------
2019-07-29T17:39:48.5868618Z 
2019-07-29T17:39:48.5868864Z ------------------------------------------
2019-07-29T17:39:48.5869049Z stderr:
2019-07-29T17:39:48.5869049Z stderr:
2019-07-29T17:39:48.5869174Z ------------------------------------------
2019-07-29T17:39:48.5869353Z thread 'main' panicked at 'bad output: thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace.rs:17:9
2019-07-29T17:39:48.5869648Z    0: std::panicking::take_hook
2019-07-29T17:39:48.5869794Z    1: std::panicking::take_hook
2019-07-29T17:39:48.5869925Z    2: std::panicking::rust_panic_with_hook
2019-07-29T17:39:48.5869925Z    2: std::panicking::rust_panic_with_hook
2019-07-29T17:39:48.5870082Z ', D:\a\1\s\src/test\ui\backtrace.rs:67:5
2019-07-29T17:39:48.5870380Z 
2019-07-29T17:39:48.5870514Z ------------------------------------------
2019-07-29T17:39:48.5870624Z 
2019-07-29T17:39:48.5870740Z 
---
2019-07-29T17:39:48.5899282Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:533:22
2019-07-29T17:39:48.5899430Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T17:39:48.5933201Z 
2019-07-29T17:39:48.5933325Z 
2019-07-29T17:39:48.5934006Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "ui" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\citools\\msys64\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T17:39:48.5934968Z 
2019-07-29T17:39:48.5935003Z 
2019-07-29T17:39:48.6883391Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2019-07-29T17:39:48.6883544Z Build completed unsuccessfully in 2:37:06
2019-07-29T17:39:48.6883544Z Build completed unsuccessfully in 2:37:06
2019-07-29T17:39:48.7219202Z make: *** [Makefile:82: ci-subset-2] Error 1
2019-07-29T17:39:49.2032126Z ##[error]Bash exited with code '2'.
2019-07-29T17:39:49.2562786Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T17:39:49.3360476Z ==============================================================================
2019-07-29T17:39:49.3360584Z Task         : Bash
2019-07-29T17:39:49.3360674Z Description  : Run a Bash script on macOS, Linux, or Windows
