plain
2019-12-13T13:12:07.0298813Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-13T13:12:07.0299226Z 
2019-12-13T13:12:07.0299301Z   git checkout -b <new-branch-name>
2019-12-13T13:12:07.0299347Z 
2019-12-13T13:12:07.0299739Z HEAD is now at a8d1fbfe9 Auto merge of #67026 - Nadrieril:improve-usefulness-empty, r=varkor,Centril,estebank
2019-12-13T13:12:07.0663375Z ##[section]Starting: Setup environment
2019-12-13T13:12:07.0770814Z ==============================================================================
2019-12-13T13:12:07.0770904Z Task         : Bash
2019-12-13T13:12:07.0770993Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-13T13:12:08.7888992Z BUILD_SOURCEBRANCHNAME=auto
2019-12-13T13:12:08.7889067Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-13T13:12:08.7889171Z BUILD_SOURCEVERSION=a8d1fbfe9401db1c3d3debf889f0c915b92ad61c
2019-12-13T13:12:08.7889270Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-13T13:12:08.7889550Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67026 - Nadrieril:improve-usefulness-empty, r=varkor,Centril,estebank
2019-12-13T13:12:08.7889741Z CI_JOB_NAME=i686-mingw-2
2019-12-13T13:12:08.7889828Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-13T13:12:08.7889914Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-13T13:12:08.7890018Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-13T13:12:08.7907213Z TMP=/tmp
2019-12-13T13:12:08.7907312Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-12-13T13:12:08.7907396Z TOOLSTATE_PUBLISH=1
2019-12-13T13:12:08.7907499Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-12-13T13:12:08.7907647Z There was a completely separate check and diagnostics for the case of an empty match. This led to slightly different error messages and duplicated code.
2019-12-13T13:12:08.7907833Z This improves code reuse and generally clarifies what happens for empty matches. This also clarifies the action of the `exhaustive_patterns` feature, and ensures that this feature doesn't change diagnostics in places it doesn't need to.
2019-12-13T13:12:08.7908139Z USERDOMAIN=fv-az363
2019-12-13T13:12:08.7908237Z USERDOMAIN_ROAMINGPROFILE=fv-az363
2019-12-13T13:12:08.7908328Z USERNAME=VssAdministrator
2019-12-13T13:12:08.7908400Z USERPROFILE=C:\Users\VssAdministrator
---
2019-12-13T16:14:04.9822179Z ---- [ui] ui\backtrace-debuginfo.rs stdout ----
2019-12-13T16:14:04.9822245Z 
2019-12-13T16:14:04.9822306Z error: test run failed!
2019-12-13T16:14:04.9822394Z status: exit code: 101
2019-12-13T16:14:04.9824299Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\mingw32\bin;C:\Python27amd64;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.163.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files " "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui\\backtrace-debuginfo\\a.exe"
2019-12-13T16:14:04.9826012Z ------------------------------------------
2019-12-13T16:14:04.9826099Z ---------------------------------------
2019-12-13T16:14:04.9826356Z trace does not match position list
2019-12-13T16:14:04.9826356Z trace does not match position list
2019-12-13T16:14:04.9826493Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:114", "backtrace-debuginfo-aux.rs:13"]
2019-12-13T16:14:04.9826671Z --- stdout
2019-12-13T16:14:04.9826732Z backtrace-debuginfo-aux.rs:13
2019-12-13T16:14:04.9826820Z backtrace-debuginfo.rs:114
2019-12-13T16:14:04.9826887Z backtrace-debuginfo.rs:126
2019-12-13T16:14:04.9826887Z backtrace-debuginfo.rs:126
2019-12-13T16:14:04.9826972Z backtrace-debuginfo.rs:189
2019-12-13T16:14:04.9827014Z 
2019-12-13T16:14:04.9827087Z --- stderr
2019-12-13T16:14:04.9827147Z test case 8
2019-12-13T16:14:04.9827251Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:115:9
2019-12-13T16:14:04.9827348Z stack backtrace:
2019-12-13T16:14:04.9827450Z    0: 0x64655bcc - _set_invalid_parameter_handler
2019-12-13T16:14:04.9827825Z    1: 0x6461adb5 - _set_invalid_parameter_handler
2019-12-13T16:14:04.9827939Z    2:   0x80c0e0 - _set_invalid_parameter_handler
2019-12-13T16:14:04.9828039Z    3:   0x80c0e0 - _set_invalid_parameter_handler
2019-12-13T16:14:04.9828121Z 
2019-12-13T16:14:04.9828201Z ------------------------------------------
2019-12-13T16:14:04.9828510Z stderr:
2019-12-13T16:14:04.9828778Z ------------------------------------------
2019-12-13T16:14:04.9828778Z ------------------------------------------
2019-12-13T16:14:04.9829005Z thread 'main' panicked at 'found some errors', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:179:9
2019-12-13T16:14:04.9829366Z 
2019-12-13T16:14:04.9829518Z ------------------------------------------
2019-12-13T16:14:04.9829665Z 
2019-12-13T16:14:04.9829782Z 
---
2019-12-13T16:14:04.9853434Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:537:22
2019-12-13T16:14:04.9854000Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-13T16:14:04.9887800Z 
2019-12-13T16:14:04.9888037Z 
2019-12-13T16:14:04.9888821Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "ui" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-13T16:14:04.9889444Z 
2019-12-13T16:14:04.9889487Z 
2019-12-13T16:14:05.0489497Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-12-13T16:14:05.0490077Z Build completed unsuccessfully in 2:52:17
2019-12-13T16:14:05.0490077Z Build completed unsuccessfully in 2:52:17
2019-12-13T16:14:05.0923226Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-12-13T16:14:05.1550892Z   local time: Fri Dec 13 16:14:05 CUT 2019
2019-12-13T16:14:05.6140302Z   network time: Fri, 13 Dec 2019 16:14:05 GMT
2019-12-13T16:14:05.6152122Z == end clock drift check ==
2019-12-13T16:14:05.7263933Z 
2019-12-13T16:14:05.7263933Z 
2019-12-13T16:14:05.9575518Z ##[error]Bash exited with code '2'.
2019-12-13T16:14:06.0055181Z ##[section]Starting: Checkout
2019-12-13T16:14:06.0783896Z ==============================================================================
2019-12-13T16:14:06.0784026Z Task         : Get sources
2019-12-13T16:14:06.0784118Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
