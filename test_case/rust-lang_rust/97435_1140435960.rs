plain
---- [ui] src/test\ui\runtime\backtrace-debuginfo.rs stdout ----

error: test run failed!
status: exit code: 101
command: PATH="D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.17.10\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\runtime\\backtrace-debuginfo\\a.exe"
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:183"]
--- stdout
backtrace-debuginfo-aux.rs:6
backtrace-debuginfo.rs:82
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:183

--- stderr
test case 2
thread 'main' panicked at 'explicit panic', D:\a\rust\rust\src/test\ui\runtime\backtrace-debuginfo.rs:83:9
stack backtrace:
   0: 0x71b94e4e - core::fmt::write::hb3de2e02f973e8b6
   1: 0x71b270f4 - std::io::Write::write_fmt::h638a5c919110df2c
   2: 0x71b367f1 - std::panicking::default_hook::{{closure}}::h8b9dcaf961af952c
   3: 0x71b3652f - std::panicking::default_hook::h85ebe8ff13fbfb91
   4: 0x71b36ed0 - std::panicking::rust_panic_with_hook::h75c2ad14e652bd23
   5:   0x4036b3 - std::panicking::begin_panic::{{closure}}::hb8bd53031aaff874
                       at D:\a\rust\rust\library\std\src\panicking.rs:617:9
   6:   0x4031b5 - std::sys_common::backtrace::__rust_end_short_backtrace::hca95f158e85e3dd1
                       at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:138:18
   7:   0x403612 - std::panicking::begin_panic::hb8b67146283bf752
                       at D:\a\rust\rust\library\std\src\panicking.rs:616:12
   8:   0x4171fe - backtrace_debuginfo::inner::{{closure}}::h59122acb3b1f10ce
                       at D:\a\rust\rust\src/test\ui\runtime\backtrace-debuginfo.rs:83:9
   9:   0x416beb - backtrace_debuginfo::aux::callback::hce098a2a42e4d40c
  10:   0x416fe0 - backtrace_debuginfo::inner::h68711b0a08793708
                       at D:\a\rust\rust\src/test\ui\runtime\backtrace-debuginfo.rs:82:29
                       at D:\a\rust\rust\src/test\ui\runtime\backtrace-debuginfo.rs:82:29
  11:   0x4175fe - backtrace_debuginfo::outer::h383486e0a6708dcb
                       at D:\a\rust\rust\src/test\ui\runtime\backtrace-debuginfo.rs:119:5
  12:   0x9af798 - <unknown>
  13:   0x405129 - core::ops::function::FnOnce::call_once::h03d8f82ea5ccd7c0
                       at D:\a\rust\rust\library\core\src\ops\function.rs:248:5
  14:   0x4031f1 - std::sys_common::backtrace::__rust_begin_short_backtrace::h17a07543e83addb6
                       at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:122:18
  15:   0x403273 - std::rt::lang_start::{{closure}}::h4431b2c2d1388e98
                       at D:\a\rust\rust\library\std\src\rt.rs:145:18
  16: 0x71b190f0 - std::rt::lang_start_internal::he2a6984270c0caf1
  17:   0x403250 - std::rt::lang_start::h6cebe39b335281f9
                       at D:\a\rust\rust\library\std\src\rt.rs:144:17
  18:   0x418ab3 - _main
  19:   0x4013e3 - ___tmainCRTStartup
  20: 0x77240419 - <unknown>
  21: 0x77c372fd - <unknown>
  22: 0x77c372cd - <unknown>
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'found some errors', D:\a\rust\rust\src/test\ui\runtime\backtrace-debuginfo.rs:173:9
------------------------------------------




failures:
    [ui] src/test\ui\runtime\backtrace-debuginfo.rs

test result: FAILED. 13095 passed; 1 failed; 193 ignored; 0 measured; 0 filtered out; finished in 541.61s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-gnu target=i686-pc-windows-gnu
Build completed unsuccessfully in 0:38:42
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
