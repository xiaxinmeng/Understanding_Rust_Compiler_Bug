plain
2019-07-26T06:04:46.5421285Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T06:04:46.5421771Z 
2019-07-26T06:04:46.5421909Z   git checkout -b <new-branch-name>
2019-07-26T06:04:46.5421961Z 
2019-07-26T06:04:46.5422047Z HEAD is now at f907629f0 Auto merge of #63001 - Centril:rollup-up9baiv, r=Centril
2019-07-26T06:04:46.5763091Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T06:04:46.5881157Z ==============================================================================
2019-07-26T06:04:46.5881259Z Task         : Bash
2019-07-26T06:04:46.5881356Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T08:20:10.9585046Z test [ui] ui\issues\issue-49851\compiler-builtins-error.rs ... ok
2019-07-26T08:20:11.0781188Z test [ui] ui\issues\issue-49919.rs ... ok
2019-07-26T08:20:11.4092578Z test [ui] ui\issues\issue-49934.rs ... ok
2019-07-26T08:20:11.5015355Z test [ui] ui\issues\issue-50187.rs ... ok
2019-07-26T08:20:11.5429456Z test [ui] ui\issues\issue-50264-inner-deref-trait\option-as_deref.rs ... ok
2019-07-26T08:20:11.6296131Z test [ui] ui\issues\issue-50264-inner-deref-trait\option-as_deref_mut.rs ... ok
2019-07-26T08:20:11.6657637Z test [ui] ui\issues\issue-50264-inner-deref-trait\result-as_deref.rs ... ok
2019-07-26T08:20:11.7622246Z test [ui] ui\issues\issue-50264-inner-deref-trait\result-as_deref_err.rs ... ok
2019-07-26T08:20:11.7996797Z test [ui] ui\issues\issue-50264-inner-deref-trait\result-as_deref_mut.rs ... ok
2019-07-26T08:20:11.9241414Z test [ui] ui\issues\issue-50264-inner-deref-trait\result-as_deref_mut_err.rs ... ok
2019-07-26T08:20:11.9403291Z test [ui] ui\issues\issue-50264-inner-deref-trait\result-as_deref_mut_ok.rs ... ok
2019-07-26T08:20:12.0146562Z test [ui] ui\issues\issue-50403.rs ... ok
2019-07-26T08:20:12.0651891Z test [ui] ui\issues\issue-50264-inner-deref-trait\result-as_deref_ok.rs ... ok
2019-07-26T08:20:12.6493804Z test [ui] ui\issues\issue-50411.rs ... ok
2019-07-26T08:20:12.8096221Z test [ui] ui\issues\issue-50480.rs ... ok
2019-07-26T08:20:12.9084826Z test [ui] ui\issues\issue-50576.rs ... ok
2019-07-26T08:20:13.0852651Z test [ui] ui\issues\issue-50577.rs ... ok
---
2019-07-26T08:22:14.8210217Z test [ui] ui\parser\raw\raw-literal-underscore.rs ... ok
2019-07-26T08:22:14.8871025Z test [ui] ui\parser\recover-enum2.rs ... ok
2019-07-26T08:22:14.8987263Z test [ui] ui\parser\recover-enum.rs ... ok
2019-07-26T08:22:15.1126744Z test [ui] ui\parser\recover-from-bad-variant.rs ... ok
2019-07-26T08:22:15.1128192Z test [ui] ui\parser\recover-from-homoglyph.rs ... ok
2019-07-26T08:22:15.3228637Z test [ui] ui\parser\recover-missing-semi.rs ... ok
2019-07-26T08:22:15.4777742Z test [ui] ui\parser\recover-tuple-pat.rs ... ok
2019-07-26T08:22:15.4835756Z test [ui] ui\parser\recovered-struct-variant.rs ... ok
2019-07-26T08:22:15.5082124Z test [ui] ui\parser\recover-tuple.rs ... ok
---
2019-07-26T08:54:51.9840167Z ---- [run-pass] run-pass\backtrace-debuginfo.rs stdout ----
2019-07-26T08:54:51.9840717Z 
2019-07-26T08:54:51.9841166Z error: test run failed!
2019-07-26T08:54:51.9842963Z status: exit code: 101
2019-07-26T08:54:51.9844765Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\citools\msys64\mingw32\bin;D:\a\1\s\citools\msys64\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.154.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.38.0.13-8.0.212-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.3\x64\bin;C:\Go1.12.4\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\run-pass\\backtrace-debuginfo\\a.exe"
2019-07-26T08:54:51.9846754Z ------------------------------------------
2019-07-26T08:54:51.9847171Z ---------------------------------------
2019-07-26T08:54:51.9848747Z trace does not match position list
2019-07-26T08:54:51.9848747Z trace does not match position list
2019-07-26T08:54:51.9848894Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:109"]
2019-07-26T08:54:51.9849054Z --- stdout
2019-07-26T08:54:51.9849144Z backtrace-debuginfo.rs:109
2019-07-26T08:54:51.9849220Z backtrace-debuginfo.rs:173
2019-07-26T08:54:51.9849285Z 
2019-07-26T08:54:51.9849285Z 
2019-07-26T08:54:51.9849633Z --- stderr
2019-07-26T08:54:51.9849738Z test case 0
2019-07-26T08:54:51.9849872Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:70:5
2019-07-26T08:54:51.9849999Z stack backtrace:
2019-07-26T08:54:51.9850086Z    0: 0x6e3d9ac0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9850201Z    1: 0x6e3d96c0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9850385Z    2: 0x6e3da2f1 - std::panicking::rust_panic_with_hook::h880c2c5069c1fea9
2019-07-26T08:54:51.9850538Z ---------------------------------------
2019-07-26T08:54:51.9850683Z trace does not match position list
2019-07-26T08:54:51.9850683Z trace does not match position list
2019-07-26T08:54:51.9850802Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110"]
2019-07-26T08:54:51.9851008Z --- stdout
2019-07-26T08:54:51.9851129Z backtrace-debuginfo.rs:110
2019-07-26T08:54:51.9851268Z backtrace-debuginfo.rs:173
2019-07-26T08:54:51.9851330Z 
2019-07-26T08:54:51.9851330Z 
2019-07-26T08:54:51.9851466Z --- stderr
2019-07-26T08:54:51.9851534Z test case 5
2019-07-26T08:54:51.9851825Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:85:5
2019-07-26T08:54:51.9851918Z stack backtrace:
2019-07-26T08:54:51.9852024Z    0: 0x6e3d9ac0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9852120Z    1: 0x6e3d96c0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9852239Z    2: 0x6e3da2f1 - std::panicking::rust_panic_with_hook::h880c2c5069c1fea9
2019-07-26T08:54:51.9852369Z ---------------------------------------
2019-07-26T08:54:51.9852548Z trace does not match position list
2019-07-26T08:54:51.9852548Z trace does not match position list
2019-07-26T08:54:51.9852667Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:93"]
2019-07-26T08:54:51.9852829Z --- stdout
2019-07-26T08:54:51.9852915Z backtrace-debuginfo.rs:93
2019-07-26T08:54:51.9853045Z backtrace-debuginfo.rs:110
2019-07-26T08:54:51.9853175Z backtrace-debuginfo.rs:173
2019-07-26T08:54:51.9853175Z backtrace-debuginfo.rs:173
2019-07-26T08:54:51.9853234Z 
2019-07-26T08:54:51.9853314Z --- stderr
2019-07-26T08:54:51.9853378Z test case 6
2019-07-26T08:54:51.9853486Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:91:9
2019-07-26T08:54:51.9853578Z stack backtrace:
2019-07-26T08:54:51.9853680Z    0: 0x6e3d9ac0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9853775Z    1: 0x6e3d96c0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9853891Z    2: 0x6e3da2f1 - std::panicking::rust_panic_with_hook::h880c2c5069c1fea9
2019-07-26T08:54:51.9854106Z ---------------------------------------
2019-07-26T08:54:51.9854188Z trace does not match position list
2019-07-26T08:54:51.9854188Z trace does not match position list
2019-07-26T08:54:51.9854322Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:95", "backtrace-debuginfo-aux.rs:5"]
2019-07-26T08:54:51.9854485Z --- stdout
2019-07-26T08:54:51.9854552Z backtrace-debuginfo-aux.rs:5
2019-07-26T08:54:51.9854658Z backtrace-debuginfo.rs:95
2019-07-26T08:54:51.9854733Z backtrace-debuginfo.rs:110
2019-07-26T08:54:51.9854733Z backtrace-debuginfo.rs:110
2019-07-26T08:54:51.9854872Z backtrace-debuginfo.rs:173
2019-07-26T08:54:51.9854932Z 
2019-07-26T08:54:51.9855000Z --- stderr
2019-07-26T08:54:51.9855083Z test case 7
2019-07-26T08:54:51.9855175Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:96:9
2019-07-26T08:54:51.9855291Z stack backtrace:
2019-07-26T08:54:51.9855376Z    0: 0x6e3d9ac0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9855558Z    1: 0x6e3d96c0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9855661Z    2: 0x6e3da2f1 - std::panicking::rust_panic_with_hook::h880c2c5069c1fea9
2019-07-26T08:54:51.9855807Z ---------------------------------------
2019-07-26T08:54:51.9855903Z trace does not match position list
2019-07-26T08:54:51.9855903Z trace does not match position list
2019-07-26T08:54:51.9856059Z still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:98", "backtrace-debuginfo-aux.rs:12"]
2019-07-26T08:54:51.9856249Z --- stdout
2019-07-26T08:54:51.9856334Z backtrace-debuginfo-aux.rs:12
2019-07-26T08:54:51.9856410Z backtrace-debuginfo.rs:98
2019-07-26T08:54:51.9856501Z backtrace-debuginfo.rs:110
2019-07-26T08:54:51.9856501Z backtrace-debuginfo.rs:110
2019-07-26T08:54:51.9856575Z backtrace-debuginfo.rs:173
2019-07-26T08:54:51.9856671Z 
2019-07-26T08:54:51.9856793Z --- stderr
2019-07-26T08:54:51.9856891Z test case 8
2019-07-26T08:54:51.9857013Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:99:9
2019-07-26T08:54:51.9857105Z stack backtrace:
2019-07-26T08:54:51.9857250Z    0: 0x6e3d9ac0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9857403Z    1: 0x6e3d96c0 - std::panicking::take_hook::h134ad8756ef11463
2019-07-26T08:54:51.9857572Z    2: 0x6e3da2f1 - std::panicking::rust_panic_with_hook::h880c2c5069c1fea9
2019-07-26T08:54:51.9857688Z 
2019-07-26T08:54:51.9857855Z ------------------------------------------
2019-07-26T08:54:51.9857995Z stderr:
2019-07-26T08:54:51.9858092Z ------------------------------------------
2019-07-26T08:54:51.9858092Z ------------------------------------------
2019-07-26T08:54:51.9858219Z thread 'main' panicked at 'found some errors', D:\a\1\s\src/test\run-pass\backtrace-debuginfo.rs:163:9
2019-07-26T08:54:51.9858389Z 
2019-07-26T08:54:51.9858476Z ------------------------------------------
2019-07-26T08:54:51.9858525Z 
2019-07-26T08:54:51.9858672Z 
---
2019-07-26T08:54:51.9859302Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:535:22
2019-07-26T08:54:51.9859702Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-26T08:54:51.9866933Z 
2019-07-26T08:54:51.9867106Z 
2019-07-26T08:54:51.9868006Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\run-pass" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\run-pass" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "run-pass" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\citools\\msys64\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-26T08:54:51.9868625Z 
2019-07-26T08:54:51.9868663Z 
2019-07-26T08:54:52.0719233Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/run-pass src/test/compile-fail src/test/run-pass-fulldeps src/tools/linkchecker
2019-07-26T08:54:52.0719814Z Build completed unsuccessfully in 2:37:10
2019-07-26T08:54:52.0719814Z Build completed unsuccessfully in 2:37:10
2019-07-26T08:54:52.0987141Z make: *** [Makefile:86: ci-subset-2] Error 1
2019-07-26T08:54:52.4902745Z ##[error]Bash exited with code '2'.
2019-07-26T08:54:52.5582131Z ##[section]Starting: Upload CPU usage statistics
2019-07-26T08:54:52.6333545Z ==============================================================================
2019-07-26T08:54:52.6333645Z Task         : Bash
2019-07-26T08:54:52.6333734Z Description  : Run a Bash script on macOS, Linux, or Windows
