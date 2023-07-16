plain
2020-03-19T05:53:24.7231121Z ---- [ui] ui\issues\issue-70093.rs stdout ----
2020-03-19T05:53:24.7231302Z 
2020-03-19T05:53:24.7231537Z error: test compilation failed although it shouldn't!
2020-03-19T05:53:24.7231782Z status: exit code: 1
2020-03-19T05:53:24.7238764Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw32\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\2.7.17\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.2\externals\git\cmd;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Go1.14\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\hostedtoolcache\windows\Python\3.7.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.6\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL " "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\issues\\issue-70093.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\issues\\issue-70093\\a.exe" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-Zlink-native-libraries=no" "-Cdefault-linker-libraries=yes" "-L" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\issues\\issue-70093\\auxiliary"
2020-03-19T05:53:24.7245915Z ------------------------------------------
2020-03-19T05:53:24.7246077Z 
2020-03-19T05:53:24.7246289Z ------------------------------------------
2020-03-19T05:53:24.7246483Z stderr:
2020-03-19T05:53:24.7246483Z stderr:
2020-03-19T05:53:24.7246698Z ------------------------------------------
2020-03-19T05:53:24.7246989Z error: linking with `link.exe` failed: exit code: 1120
2020-03-19T05:53:24.7247360Z    |
2020-03-19T05:53:24.7250286Z    = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x86\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LARGEADDRESSAWARE" "/SAFESEH" "/LIBPATH:C:\\MORE_SPACE\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\issues\\issue-70093\\a.issue_70093.7rcbfp3g-cgu.0.rcgu.o" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\issues\\issue-70093\\a.issue_70093.7rcbfp3g-cgu.1.rcgu.o" "/OUT:D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\issues\\issue-70093\\a.exe" "/OPT:REF,ICF" "/DEBUG" "/LIBPATH:C:\\MORE_SPACE\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "std-0f54896ef7f9d739.dll.lib" "C:\\MORE_SPACE\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib\\libcompiler_builtins-6ac82cfc559709b1.rlib"
2020-03-19T05:53:24.7252647Z    = note: LINK : error LNK2001: unresolved external symbol _mainCRTStartup
2020-03-19T05:53:24.7253222Z            D:\a\1\s\build\i686-pc-windows-msvc\test\ui\issues\issue-70093\a.exe : fatal error LNK1120: 1 unresolved externals
2020-03-19T05:53:24.7253945Z 
2020-03-19T05:53:24.7254164Z error: aborting due to previous error
2020-03-19T05:53:24.7254336Z 
2020-03-19T05:53:24.7254459Z 
---
2020-03-19T05:53:24.7269048Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:348:22
2020-03-19T05:53:24.7269542Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T05:53:24.7295795Z 
2020-03-19T05:53:24.7296092Z 
2020-03-19T05:53:24.7300523Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "ui" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T05:53:24.7304268Z 
2020-03-19T05:53:24.7304398Z 
2020-03-19T05:53:26.2097812Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2020-03-19T05:53:26.2195549Z Build completed unsuccessfully in 1:56:42
2020-03-19T05:53:26.2195549Z Build completed unsuccessfully in 1:56:42
2020-03-19T05:53:26.2196038Z make: *** [Makefile:82: ci-subset-2] Error 1
2020-03-19T05:53:26.2196803Z   local time: Thu Mar 19 05:53:24 CUT 2020
2020-03-19T05:53:26.2197311Z   network time: Thu, 19 Mar 2020 05:53:25 GMT
2020-03-19T05:53:26.2197673Z == end clock drift check ==
2020-03-19T05:53:26.2198152Z 
2020-03-19T05:53:26.2198152Z 
2020-03-19T05:53:26.3680043Z ##[error]Bash exited with code '2'.
2020-03-19T05:53:26.4718633Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-19T05:53:27.2638388Z ==============================================================================
2020-03-19T05:53:27.2638858Z Task         : Get sources
2020-03-19T05:53:27.2639571Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
