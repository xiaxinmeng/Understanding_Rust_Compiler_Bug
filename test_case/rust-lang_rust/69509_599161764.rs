plain
2020-03-15T03:07:14.2427958Z ---- [ui] ui\issues\issue-40883.rs stdout ----
2020-03-15T03:07:14.2428211Z 
2020-03-15T03:07:14.2428853Z error: test run failed!
2020-03-15T03:07:14.2429609Z status: exit code: 101
2020-03-15T03:07:14.2437026Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.2\externals\git\cmd;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\issues\\issue-40883\\a.exe"
2020-03-15T03:07:14.2443226Z ------------------------------------------
2020-03-15T03:07:14.2443491Z 
2020-03-15T03:07:14.2443761Z ------------------------------------------
2020-03-15T03:07:14.2444063Z stderr:
2020-03-15T03:07:14.2444063Z stderr:
2020-03-15T03:07:14.2444332Z ------------------------------------------
2020-03-15T03:07:14.2444898Z thread 'main' panicked at 'used 1024 bytes of stack, but `struct Big` is only 384 bytes', D:\a\1\s\src/test\ui\issues\issue-40883.rs:83:9
2020-03-15T03:07:14.2445813Z 
2020-03-15T03:07:14.2446107Z ------------------------------------------
2020-03-15T03:07:14.2446344Z 
2020-03-15T03:07:14.2446537Z 
---
2020-03-15T03:07:14.2469910Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:348:22
2020-03-15T03:07:14.2470504Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-15T03:07:14.2504476Z 
2020-03-15T03:07:14.2504657Z 
2020-03-15T03:07:14.2507913Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--mode" "ui" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-15T03:07:14.2511512Z 
2020-03-15T03:07:14.2511774Z 
2020-03-15T03:07:14.3341278Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2020-03-15T03:07:14.3342043Z Build completed unsuccessfully in 2:13:21
2020-03-15T03:07:14.3342043Z Build completed unsuccessfully in 2:13:21
2020-03-15T03:07:14.4405924Z make: *** [Makefile:82: ci-subset-2] Error 1
2020-03-15T03:07:14.5036242Z   local time: Sun Mar 15 03:07:14 CUT 2020
2020-03-15T03:07:15.0844799Z   network time: Sun, 15 Mar 2020 03:07:15 GMT
2020-03-15T03:07:15.0862339Z == end clock drift check ==
2020-03-15T03:07:15.1785287Z 
2020-03-15T03:07:15.1785287Z 
2020-03-15T03:07:15.4646731Z ##[error]Bash exited with code '2'.
2020-03-15T03:07:15.5442690Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-15T03:07:15.6212209Z ==============================================================================
2020-03-15T03:07:15.6212580Z Task         : Get sources
2020-03-15T03:07:15.6212897Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
