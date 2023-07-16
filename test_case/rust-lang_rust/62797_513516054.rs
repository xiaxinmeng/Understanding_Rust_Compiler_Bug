plain
2019-07-21T03:23:16.9767994Z ---- [run-pass] run-pass\backtrace-debuginfo.rs stdout ----
2019-07-21T03:23:16.9768050Z 
2019-07-21T03:23:16.9768133Z error: test run failed!
2019-07-21T03:23:16.9768209Z status: exit code: 101
2019-07-21T03:23:16.9769955Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\citools\msys64\mingw32\bin;D:\a\1\s\citools\msys64\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.154.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.38.0.13-8.0.212-win_x64\bin;C:\npm\prefix;C:\Program Files\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.3\x64\bin;C:\Go1.12.4\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\Scripts;C:\hostedtoolcache\windows\Python\3.6.8;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\run-pass\\backtrace-debuginfo\\a.exe"
2019-07-21T03:23:16.9771298Z ------------------------------------------
2019-07-21T03:23:16.9772776Z ---------------------------------------
2019-07-21T03:23:16.9772875Z trace does not match position list
2019-07-21T03:23:16.9772875Z trace does not match position list
2019-07-21T03:23:16.9773221Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:109", "backtrace-debuginfo.rs:72", "backtrace-debuginfo-aux.rs:5"]
2019-07-21T03:23:16.9773450Z --- stdout
2019-07-21T03:23:16.9773520Z backtrace-debuginfo-aux.rs:5
2019-07-21T03:23:16.9773814Z backtrace-debuginfo.rs:72
2019-07-21T03:23:16.9773890Z backtrace-debuginfo.rs:109
2019-07-21T03:23:16.9773890Z backtrace-debuginfo.rs:109
2019-07-21T03:23:16.9773983Z backtrace-debuginfo.rs:173
2019-07-21T03:23:16.9774031Z 
2019-07-21T03:23:16.9774095Z --- stderr
2019-07-21T03:23:16.9774180Z test case 2
2019-07-21T03:23:16.9774275Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:73:9
2019-07-21T03:23:16.9774397Z stack backtrace:
2019-07-21T03:23:16.9774502Z    0: 0x71809730 - std::panicking::take_hook::hfbb9fe42674c6a04
2019-07-21T03:23:16.9774601Z    1: 0x71809330 - std::panicking::take_hook::hfbb9fe42674c6a04
2019-07-21T03:23:16.9774717Z    2: 0x71809f61 - std::panicking::rust_panic_with_hook::h49a19dfc0991b7ac
2019-07-21T03:23:16.9774856Z ---------------------------------------
2019-07-21T03:23:16.9774953Z trace does not match position list
2019-07-21T03:23:16.9774953Z trace does not match position list
2019-07-21T03:23:16.9775048Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110"]
2019-07-21T03:23:16.9775200Z --- stdout
2019-07-21T03:23:16.9775283Z backtrace-debuginfo.rs:110
2019-07-21T03:23:16.9775359Z backtrace-debuginfo.rs:173
2019-07-21T03:23:16.9775406Z 
2019-07-21T03:23:16.9775406Z 
2019-07-21T03:23:16.9775491Z --- stderr
2019-07-21T03:23:16.9775555Z test case 4
2019-07-21T03:23:16.9775666Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:84:5
2019-07-21T03:23:16.9775758Z stack backtrace:
2019-07-21T03:23:16.9775859Z    0: 0x71809730 - std::panicking::take_hook::hfbb9fe42674c6a04
2019-07-21T03:23:16.9775971Z    1: 0x71809330 - std::panicking::take_hook::hfbb9fe42674c6a04
2019-07-21T03:23:16.9776079Z    2: 0x71809f61 - std::panicking::rust_panic_with_hook::h49a19dfc0991b7ac
2019-07-21T03:23:16.9776223Z ---------------------------------------
2019-07-21T03:23:16.9776300Z trace does not match position list
2019-07-21T03:23:16.9776300Z trace does not match position list
2019-07-21T03:23:16.9776434Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:98", "backtrace-debuginfo-aux.rs:12"]
2019-07-21T03:23:16.9776605Z --- stdout
2019-07-21T03:23:16.9776672Z backtrace-debuginfo-aux.rs:12
2019-07-21T03:23:16.9776764Z backtrace-debuginfo.rs:98
2019-07-21T03:23:16.9776838Z backtrace-debuginfo.rs:110
2019-07-21T03:23:16.9776838Z backtrace-debuginfo.rs:110
2019-07-21T03:23:16.9776932Z backtrace-debuginfo.rs:173
2019-07-21T03:23:16.9776979Z 
2019-07-21T03:23:16.9777057Z --- stderr
2019-07-21T03:23:16.9777123Z test case 8
2019-07-21T03:23:16.9777232Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:99:9
2019-07-21T03:23:16.9777323Z stack backtrace:
2019-07-21T03:23:16.9777434Z    0: 0x71809730 - std::panicking::take_hook::hfbb9fe42674c6a04
2019-07-21T03:23:16.9777531Z    1: 0x71809330 - std::panicking::take_hook::hfbb9fe42674c6a04
2019-07-21T03:23:16.9778734Z    2: 0x71809f61 - std::panicking::rust_panic_with_hook::h49a19dfc0991b7ac
2019-07-21T03:23:16.9778919Z 
2019-07-21T03:23:16.9793452Z ------------------------------------------
2019-07-21T03:23:16.9795236Z stderr:
2019-07-21T03:23:16.9795609Z ------------------------------------------
2019-07-21T03:23:16.9795609Z ------------------------------------------
2019-07-21T03:23:16.9795984Z thread 'main' panicked at 'found some errors', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:163:9
2019-07-21T03:23:16.9796569Z 
2019-07-21T03:23:16.9796880Z ------------------------------------------
2019-07-21T03:23:16.9796990Z 
2019-07-21T03:23:16.9797081Z 
---
2019-07-21T03:23:16.9798088Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:535:22
2019-07-21T03:23:16.9798296Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-21T03:23:16.9798357Z 
2019-07-21T03:23:16.9798414Z 
2019-07-21T03:23:16.9799135Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\run-pass" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\run-pass" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "run-pass" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\citools\\msys64\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-21T03:23:16.9800112Z 
2019-07-21T03:23:16.9800176Z 
2019-07-21T03:23:17.0979952Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/run-pass src/test/compile-fail src/test/run-pass-fulldeps src/tools/linkchecker
2019-07-21T03:23:17.0980436Z Build completed unsuccessfully in 2:43:08
2019-07-21T03:23:17.0980436Z Build completed unsuccessfully in 2:43:08
2019-07-21T03:23:17.1305148Z make: *** [Makefile:86: ci-subset-2] Error 1
2019-07-21T03:23:17.4646537Z ##[error]Bash exited with code '2'.
2019-07-21T03:23:17.5265430Z ##[section]Starting: Upload CPU usage statistics
2019-07-21T03:23:17.6300670Z ==============================================================================
2019-07-21T03:23:17.6300795Z Task         : Bash
2019-07-21T03:23:17.6300884Z Description  : Run a Bash script on macOS, Linux, or Windows
