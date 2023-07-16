plain
2020-02-12T01:44:29.7392981Z NOTE: compiletest thinks it is using GDB version 7010001
2020-02-12T01:44:29.7393136Z 
2020-02-12T01:44:29.7393228Z error: gdb failed to execute
2020-02-12T01:44:29.7393299Z status: exit code: 0xc0000044
2020-02-12T01:44:29.7400847Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\mingw32\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.8\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program " "D:\\a\\1\\s\\mingw32\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\closure-in-generic-function.gdb\\closure-in-generic-function.debugger.script"
2020-02-12T01:44:29.7403071Z ------------------------------------------
2020-02-12T01:44:29.7403144Z 
2020-02-12T01:44:29.7403212Z ------------------------------------------
2020-02-12T01:44:29.7403317Z stderr:
---
2020-02-12T01:44:29.7404357Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:348:22
2020-02-12T01:44:29.7404481Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-12T01:44:29.7411014Z 
2020-02-12T01:44:29.7411312Z 
2020-02-12T01:44:29.7412473Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\debuginfo" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "debuginfo" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw32\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-12T01:44:29.7413474Z 
2020-02-12T01:44:29.7413512Z 
2020-02-12T01:44:29.8364064Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2020-02-12T01:44:29.8364492Z Build completed unsuccessfully in 1:36:29
2020-02-12T01:44:29.8364492Z Build completed unsuccessfully in 1:36:29
2020-02-12T01:44:29.8742491Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2020-02-12T01:44:29.9355114Z   local time: Wed Feb 12 01:44:29 CUT 2020
2020-02-12T01:44:30.3592237Z   network time: Wed, 12 Feb 2020 01:44:30 GMT
2020-02-12T01:44:30.3638777Z == end clock drift check ==
2020-02-12T01:44:30.5117905Z 
2020-02-12T01:44:30.5117905Z 
2020-02-12T01:44:30.8259353Z ##[error]Bash exited with code '2'.
2020-02-12T01:44:30.8812735Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-12T01:44:30.9472597Z ==============================================================================
2020-02-12T01:44:30.9472733Z Task         : Get sources
2020-02-12T01:44:30.9472820Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
