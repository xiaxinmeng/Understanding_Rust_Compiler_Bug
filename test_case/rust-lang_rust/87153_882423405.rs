plain
failures:

---- [debuginfo-cdb] debuginfo\msvc-pretty-enums.rs stdout ----

error: line not found in debugger output:     [+0x000] Ok               [Type: enum$<core::result::Result<u32, enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\msvc-pretty-enums.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff6`95160000 00007ff6`95169000   a.exe   
ModLoad: 00007ff9`1da90000 00007ff9`1dc7d000   ntdll.dll
ModLoad: 00007ff9`1af30000 00007ff9`1afe3000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`1a1b0000 00007ff9`1a445000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`17cc0000 00007ff9`17d4c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`19d50000 00007ff9`19e4a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`0d990000 00007ff9`0d9ab000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff8`ecdb0000 00007ff8`ed25c000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-f021d642ed65bd1b.dll
ModLoad: 00007ff9`1d1b0000 00007ff9`1d21d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`1d220000 00007ff9`1d33f000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`1cc10000 00007ff9`1ccb4000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`1d810000 00007ff9`1d8ae000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`1ad30000 00007ff9`1adce000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`19970000 00007ff9`19999000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`19b10000 00007ff9`19b34000   C:\Windows\System32\profapi.dll
ModLoad: 00007ff9`19420000 00007ff9`1942c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`1a100000 00007ff9`1a182000   C:\Windows\System32\bcryptPrimitives.dll
(1f58.1024): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`1db6260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Mon Jul 19 10:08:42.217 2021 (UTC + 0:00)
System Uptime: 0 days 4:54:09.123
Process Uptime: 0 days 0:00:00.575
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\msvc-pretty-enums.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe'  Debugger Process 0xF74 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.19041.685, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `msvc-pretty-enums.rs:121`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!msvc_pretty_enums::main+0x85:
00007ff6`95162f55 e846000000      call    a!msvc_pretty_enums::zzz (00007ff6`95162fa0)
0:000>  dx -r2 a,!
a,!              [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Some]
        [+0x000] __0              : Low (0x2) [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : 0x2 [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Discriminant$]
0:000>  dx -r2 b,!
b,!              [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Some]
        [+0x000] __0              : 0x11 [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : None (0x11) [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Discriminant$]
0:000>  dx -r2 c,!
c,!              [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [+0x000] dataful_variant  [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
        [+0x000] my_data          : 0x11 [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : Tag1 (0x11) [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Discriminant$]
0:000>  dx -r2 d,!
d,!              [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [+0x000] dataful_variant  [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
        [+0x000] my_data          : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : 0x10 [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Discriminant$]
0:000>  dx -r2 e,!
e,!              [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [+0x000] dataful_variant  [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
        [+0x000] my_data          : 0x13 [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : Tag2 (0x13) [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Discriminant$]
0:000>  dx -r2 f,!
f,!              [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Some]
        [+0x000] __0              : 0x7ff6951644a0 : 0x1 [Type: unsigned int *]
    [+0x000] discriminant     : 0x7ff6951644a0 [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Discriminant$]
0:000>  dx -r2 g,!
g,!              [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Some]
        [+0x000] __0              : 0x0 [Type: unsigned int *]
    [+0x000] discriminant     : None (0x0) [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Discriminant$]
0:000>  dx -r2 h,!
h,!              : Some [Type: enum$<core::option::Option<u32> >]
    [+0x000] variant0         [Type: enum$<core::option::Option<u32> >::None]
    [+0x000] variant1         [Type: enum$<core::option::Option<u32> >::Some]
        [+0x004] __0              : 0xc [Type: unsigned int]
    [+0x000] discriminant     : Some (0x1) [Type: core::option::Option]
0:000>  dx h
h                : Some [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : Some
    [+0x004] __0              : 0xc [Type: unsigned int]
0:000>  dx -r2 i,!
i,!              : None [Type: enum$<core::option::Option<u32> >]
    [+0x000] variant0         [Type: enum$<core::option::Option<u32> >::None]
    [+0x000] variant1         [Type: enum$<core::option::Option<u32> >::Some]
        [+0x004] __0              : 0x1ee [Type: unsigned int]
    [+0x000] discriminant     : None (0x0) [Type: core::option::Option]
0:000>  dx i
i                : None [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : None
0:000>  dx j
j                : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
0:000>  dx -r2 k,!
k,!              [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>::Some]
        [+0x000] __0              [Type: alloc::string::String]
    [+0x000] discriminant     : 0x1ee3b3a0590 [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>::Discriminant$]
0:000>  dx -r2 l,!
l,!              : $T2 [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [+0x000] Ok               [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
        [+0x000] __0              : 0x2a [Type: unsigned int]
0:000> 
l,!              : $T2 [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [+0x000] Ok               [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
        [+0x000] __0              : 0x2a [Type: unsigned int]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\winrt.natvis'
------------------------------------------
stderr:
------------------------------------------

---
test result: FAILED. 110 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 31.02s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.6\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.6\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"



make: *** [Makefile:72: ci-subset-1] Error 1
