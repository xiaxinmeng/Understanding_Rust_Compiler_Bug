plain
test [debuginfo-gdb] src/test\debuginfo\cross-crate-type-uniquing.rs ... ok

failures:

---- [debuginfo-gdb] src/test\debuginfo\thread-names.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7010001

error: line not found in debugger output:   1    Thread [...] [...] "main" [...]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\lib\rustlib\x86_64-pc-windows-gnu\lib;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.17.10\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo\\thread-names.gdb\\thread-names.debugger.script"
--- stdout -------------------------------
GNU gdb (GDB) 7.10.1
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-w64-mingw32".
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x404c54: file src/test\debuginfo/thread-names.rs, line 44.
[New Thread 7712.0x1a4c]
[New Thread 7712.0xbf8]
[New Thread 7712.0x1e64]
[New Thread 7712.0xf24]
[New Thread 7712.0x9f8]
[Switching to Thread 7712.0x9f8]

Breakpoint 1, thread_names::main::_$u7b$$u7b$closure$u7d$$u7d$::hd5f69bd5cd9d8878 () at src/test\debuginfo/thread-names.rs:44
44         zzz(); // #break
  Id   Target Id         Frame 
* 5    Thread 7712.0x9f8 thread_names::main::_$u7b$$u7b$closure$u7d$$u7d$::hd5f69bd5cd9d8878 () at src/test\debuginfo/thread-names.rs:44
  4    Thread 7712.0xf24 0x00007ff9fd9c3364 in ntdll!ZwWaitForWorkViaWorkerFactory () from C:\Windows\SYSTEM32\ntdll.dll
  3    Thread 7712.0x1e64 0x00007ff9fd9c3364 in ntdll!ZwWaitForWorkViaWorkerFactory () from C:\Windows\SYSTEM32\ntdll.dll
  2    Thread 7712.0xbf8 0x00007ff9fd9c3364 in ntdll!ZwWaitForWorkViaWorkerFactory () from C:\Windows\SYSTEM32\ntdll.dll
  1    Thread 7712.0x1a4c 0x00007ff9fd9bfa74 in ntdll!ZwWaitForSingleObject () from C:\Windows\SYSTEM32\ntdll.dll
A debugging session is active.

 Inferior 1 [process 7712] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
stderr: none




failures:
    [debuginfo-gdb] src/test\debuginfo\thread-names.rs

test result: FAILED. 82 passed; 1 failed; 55 ignored; 0 measured; 0 filtered out; finished in 9.74s

Build completed unsuccessfully in 0:40:18
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
