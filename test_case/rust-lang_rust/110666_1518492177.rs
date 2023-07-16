plain
---- [ui] tests\ui\runtime\backtrace-debuginfo.rs stdout ----

error: test run failed!
status: exit code: 101
command: PATH="C:\a\rust\rust\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;C:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;C:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;C:\a\rust\rust\ninja;C:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.3\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.20.3\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\Program Files\OpenSSL\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\runtime\\backtrace-debuginfo\\a.exe"
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:184"]
--- stdout
backtrace-debuginfo-aux.rs:6
backtrace-debuginfo.rs:83
backtrace-debuginfo.rs:120
backtrace-debuginfo.rs:120
backtrace-debuginfo.rs:184

--- stderr
test case 2
thread 'main' panicked at 'explicit panic', fake-test-src-base\runtime\backtrace-debuginfo.rs:84:9
stack backtrace:
   0: 0x6644fa3f - core::fmt::write::h2b9eb53d2547fe27
   1: 0x663e4034 - std::io::Write::write_fmt::h6e9419f982a5b1f5
   2: 0x663ee3a0 - std::sys_common::backtrace::print::hc43a427a6b7b1661
   3: 0x663f10bf - std::panicking::default_hook::{{closure}}::h7bf1029117ec41b2
   4: 0x663f0e2f - std::panicking::default_hook::h569445ed98347b73
   5: 0x663f17bc - std::panicking::rust_panic_with_hook::hedae779c90bfd63d
   6:   0x3d30cd - std::panicking::begin_panic::{{closure}}::hfd1e9363b88299c5
                       at /rustc/FAKE_PREFIX\library\std\src/panicking.rs:611:9
   7:   0x3d2d22 - std::sys_common::backtrace::__rust_end_short_backtrace::hb7be814dc6e24d49
                       at /rustc/FAKE_PREFIX\library\std\src\sys_common/backtrace.rs:150:18
   8:   0x3d3050 - std::panicking::begin_panic::h63d3135886ecb4b8
                       at /rustc/FAKE_PREFIX\library\std\src/panicking.rs:610:12
   9:   0x3e4482 - backtrace_debuginfo::inner::{{closure}}::h91be12834cc6ccb2
                       at C:\a\rust\rust\fake-test-src-base\runtime\backtrace-debuginfo.rs:84:9
  10:   0x3e3fc1 - backtrace_debuginfo::aux::callback::hdf08c9083bd261d9
                       at C:\a\rust\rust\fake-test-src-base\runtime\backtrace-debuginfo-aux.rs:6:5
  11:   0x3e4297 - backtrace_debuginfo::inner::hf8248a822002144b
                       at C:\a\rust\rust\fake-test-src-base\runtime\backtrace-debuginfo.rs:83:29
  12:   0x3e485e - backtrace_debuginfo::outer::h21d58bbf1167d7dc
                       at C:\a\rust\rust\fake-test-src-base\runtime\backtrace-debuginfo.rs:120:5
  13:  0x159c638 - <unknown>
  14:   0x3d44b9 - core::ops::function::FnOnce::call_once::ha10d483f3fcd748f
                       at /rustc/FAKE_PREFIX\library\core\src\ops/function.rs:250:5
  15:   0x3d2d41 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd91813ca6d013a64
                       at /rustc/FAKE_PREFIX\library\std\src\sys_common/backtrace.rs:134:18
  16:   0x3d2db3 - std::rt::lang_start::{{closure}}::h8ed4690c349bb974
                       at /rustc/FAKE_PREFIX\library\std\src/rt.rs:166:18
  17: 0x663d716e - std::rt::lang_start_internal::h4a8815a1a90ca70e
  18:   0x3d2d8b - std::rt::lang_start::h5f059116e4069fb5
                       at /rustc/FAKE_PREFIX\library\std\src/rt.rs:165:17
  19:   0x3e5b7e - _main
  20:   0x3d1386 - ___tmainCRTStartup
  21: 0x75fe05b9 - <unknown>
  22: 0x7741779d - <unknown>
  23: 0x7741776d - <unknown>
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'found some errors', fake-test-src-base\runtime\backtrace-debuginfo.rs:174:9
------------------------------------------




failures:
    [ui] tests\ui\runtime\backtrace-debuginfo.rs

test result: FAILED. 14625 passed; 1 failed; 224 ignored; 0 measured; 0 filtered out; finished in 897.81s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-gnu target=i686-pc-windows-gnu
Build completed unsuccessfully in 0:54:57
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
