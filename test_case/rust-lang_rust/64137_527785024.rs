plain
2019-09-04T04:25:56.5290017Z do so (now or later) by using -b with the checkout command again. Example:
2019-09-04T04:25:56.5290985Z 
2019-09-04T04:25:56.5292040Z   git checkout -b <new-branch-name>
2019-09-04T04:25:56.5292747Z 
2019-09-04T04:25:56.5293147Z HEAD is now at 4e99d1bbc Auto merge of #64137 - Centril:rollup-4am55q3, r=Centril
2019-09-04T04:25:56.5654719Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-09-04T04:25:56.5767126Z ==============================================================================
2019-09-04T04:25:56.5767249Z Task         : Bash
2019-09-04T04:25:56.5767325Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-09-04T07:44:30.9144540Z ---- [ui] ui\backtrace-debuginfo.rs stdout ----
2019-09-04T07:44:30.9144581Z 
2019-09-04T07:44:30.9144647Z error: test run failed!
2019-09-04T07:44:30.9144704Z status: exit code: 101
2019-09-04T07:44:30.9145580Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\mingw32\bin;D:\a\1\s\citools\msys64\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.155.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui\\backtrace-debuginfo\\a.exe"
2019-09-04T07:44:30.9146903Z ------------------------------------------
2019-09-04T07:44:30.9147016Z ---------------------------------------
2019-09-04T07:44:30.9147172Z trace does not match position list
2019-09-04T07:44:30.9147172Z trace does not match position list
2019-09-04T07:44:30.9147334Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126"]
2019-09-04T07:44:30.9147545Z --- stdout
2019-09-04T07:44:30.9147725Z backtrace-debuginfo-aux.rs:6
2019-09-04T07:44:30.9147803Z backtrace-debuginfo.rs:111
2019-09-04T07:44:30.9147876Z backtrace-debuginfo.rs:126
2019-09-04T07:44:30.9147876Z backtrace-debuginfo.rs:126
2019-09-04T07:44:30.9147931Z backtrace-debuginfo.rs:189
2019-09-04T07:44:30.9147990Z 
2019-09-04T07:44:30.9148038Z --- stderr
2019-09-04T07:44:30.9148285Z test case 7
2019-09-04T07:44:30.9148603Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:112:9
2019-09-04T07:44:30.9148710Z stack backtrace:
2019-09-04T07:44:30.9148788Z    0: 0x70665ab9 - mingw_set_invalid_parameter_handler
2019-09-04T07:44:30.9148891Z    1: 0x706657d4 - mingw_set_invalid_parameter_handler
2019-09-04T07:44:30.9148975Z    2: 0x7066619d - mingw_set_invalid_parameter_handler
2019-09-04T07:44:30.9149098Z    3:   0x42179f - std::panicking::begin_panic::hff02dbaf8ddf02f7
2019-09-04T07:44:30.9149189Z                        at D:\a\1\s\src\libstd/panicking.rs:411
2019-09-04T07:44:30.9149315Z    4:   0x40ee4f - backtrace_debuginfo::inner_inlined::{{closure}}::h62023ed1a359e296
2019-09-04T07:44:30.9149421Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:112
2019-09-04T07:44:30.9149541Z    5:   0x40ebdb - backtrace_debuginfo::aux::callback::h5834211fa2110840
2019-09-04T07:44:30.9149635Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo-aux.rs:6
2019-09-04T07:44:30.9149746Z    6:   0x417e64 - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
2019-09-04T07:44:30.9149839Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:111
2019-09-04T07:44:30.9149956Z    7:   0x99c4c8 - mingw_set_invalid_parameter_handler
2019-09-04T07:44:30.9150043Z    8:   0x4124ac - core::str::<impl str>::parse::h4c96c3e5477cebd4
2019-09-04T07:44:30.9150155Z                        at D:\a\1\s\src\libcore\str/mod.rs:3969
2019-09-04T07:44:30.9150268Z 
2019-09-04T07:44:30.9150332Z ------------------------------------------
2019-09-04T07:44:30.9150419Z stderr:
2019-09-04T07:44:30.9150489Z ------------------------------------------
2019-09-04T07:44:30.9150489Z ------------------------------------------
2019-09-04T07:44:30.9150607Z thread 'main' panicked at 'found some errors', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:179:9
2019-09-04T07:44:30.9150763Z 
2019-09-04T07:44:30.9150843Z ------------------------------------------
2019-09-04T07:44:30.9150889Z 
2019-09-04T07:44:30.9150943Z 
---
2019-09-04T07:44:30.9179744Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:536:22
2019-09-04T07:44:30.9179909Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-04T07:44:30.9210036Z 
2019-09-04T07:44:30.9210450Z 
2019-09-04T07:44:30.9211148Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "ui" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-04T07:44:30.9211792Z 
2019-09-04T07:44:30.9211857Z 
2019-09-04T07:44:31.0140446Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-09-04T07:44:31.0141783Z Build completed unsuccessfully in 3:07:15
2019-09-04T07:44:31.0141783Z Build completed unsuccessfully in 3:07:15
2019-09-04T07:44:31.0399754Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-09-04T07:44:31.1059317Z   local time: Wed Sep  4 07:44:31 CUT 2019
2019-09-04T07:44:31.5980344Z   network time: Wed, 04 Sep 2019 07:44:31 GMT
2019-09-04T07:44:31.5981497Z == end clock drift check ==
2019-09-04T07:44:31.5981497Z == end clock drift check ==
2019-09-04T07:44:32.0389524Z ##[error]Bash exited with code '2'.
2019-09-04T07:44:32.0892287Z ##[section]Starting: Upload CPU usage statistics
2019-09-04T07:44:32.1667195Z ==============================================================================
2019-09-04T07:44:32.1667284Z Task         : Bash
2019-09-04T07:44:32.1667363Z Description  : Run a Bash script on macOS, Linux, or Windows
