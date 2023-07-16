plain
failures:

---- [debuginfo-cdb] debuginfo\generic-struct.rs stdout ----

error: line not found in debugger output: int_int          [Type: generic_struct::AGenericStruct<i32, i32>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\generic-struct.cdb\\generic-struct.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\generic-struct.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\generic-struct.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`ac010000 00007ff7`ac017000   a.exe   
ModLoad: 00007ff9`3c9c0000 00007ff9`3cbad000   ntdll.dll
ModLoad: 00007ff9`39e70000 00007ff9`39f23000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`395f0000 00007ff9`39885000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`36bf0000 00007ff9`36c7c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`393f0000 00007ff9`394ea000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`2d6d0000 00007ff9`2d6eb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`0a740000 00007ff9`0abee000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-f021d642ed65bd1b.dll
ModLoad: 00007ff9`3a2a0000 00007ff9`3a30d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`3a390000 00007ff9`3a4af000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`3c740000 00007ff9`3c7e4000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`39f40000 00007ff9`39fde000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`39dd0000 00007ff9`39e6e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`388d0000 00007ff9`388f9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`38a20000 00007ff9`38a44000   C:\Windows\System32\profapi.dll
ModLoad: 00007ff9`38350000 00007ff9`3835c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`38a70000 00007ff9`38af2000   C:\Windows\System32\bcryptPrimitives.dll
(1894.1a60): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`3ca9260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Fri Jul 16 13:34:22.254 2021 (UTC + 0:00)
System Uptime: 0 days 5:46:54.896
Process Uptime: 0 days 0:00:00.566
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\generic-struct.cdb\generic-struct.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\generic-struct.cdb\a.exe'  Debugger Process 0x15A8 
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
0:000> bp `generic-struct.rs:82`
*** WARNING: Unable to verify checksum for a.exe
Breakpoint 0 hit
Breakpoint 0 hit
a!generic_struct::main+0x78:
00007ff7`ac0110a8 e813000000      call    a!generic_struct::zzz (00007ff7`ac0110c0)
0:000> dx int_int
int_int          [Type: generic_struct::AGenericStruct<i32,i32>]
    [+0x000] key              : 0 [Type: int]
    [+0x004] value            : 1 [Type: int]
0:000> dx int_float
int_float        [Type: generic_struct::AGenericStruct<i32,f64>]
    [+0x008] key              : 2 [Type: int]
    [+0x000] value            : 3.500000 [Type: double]
0:000> dx float_int
float_int        [Type: generic_struct::AGenericStruct<f64,i32>]
    [+0x000] key              : 4.500000 [Type: double]
    [+0x008] value            : 5 [Type: int]
0:000> dx float_int_float
float_int_float  [Type: generic_struct::AGenericStruct<f64,generic_struct::AGenericStruct<i32,f64> >]
    [+0x000] key              : 6.500000 [Type: double]
    [+0x008] value            [Type: generic_struct::AGenericStruct<i32,f64>]
0:000> 
float_int_float  [Type: generic_struct::AGenericStruct<f64,generic_struct::AGenericStruct<i32,f64> >]
    [+0x000] key              : 6.500000 [Type: double]
    [+0x008] value            [Type: generic_struct::AGenericStruct<i32,f64>]
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


------------------------------------------


---- [debuginfo-cdb] debuginfo\msvc-pretty-enums.rs stdout ----

error: line not found in debugger output: l,!              : $T2 [Type: enum$<core::result::Result<u32, enum$<msvc_pretty_enums::Empty> >, Ok>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\msvc-pretty-enums.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`b9820000 00007ff7`b9829000   a.exe   
ModLoad: 00007ff9`3c9c0000 00007ff9`3cbad000   ntdll.dll
ModLoad: 00007ff9`39e70000 00007ff9`39f23000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`395f0000 00007ff9`39885000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`36bf0000 00007ff9`36c7c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`393f0000 00007ff9`394ea000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`2d6d0000 00007ff9`2d6eb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`0a740000 00007ff9`0abee000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-f021d642ed65bd1b.dll
ModLoad: 00007ff9`3a2a0000 00007ff9`3a30d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`3a390000 00007ff9`3a4af000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`3c740000 00007ff9`3c7e4000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`39f40000 00007ff9`39fde000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`39dd0000 00007ff9`39e6e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`388d0000 00007ff9`388f9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`38a20000 00007ff9`38a44000   C:\Windows\System32\profapi.dll
ModLoad: 00007ff9`38350000 00007ff9`3835c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`38a70000 00007ff9`38af2000   C:\Windows\System32\bcryptPrimitives.dll
(810.13e4): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`3ca9260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Fri Jul 16 13:34:26.203 2021 (UTC + 0:00)
System Uptime: 0 days 5:46:58.846
Process Uptime: 0 days 0:00:00.567
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\msvc-pretty-enums.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe'  Debugger Process 0xFE0 
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
00007ff7`b9822f55 e846000000      call    a!msvc_pretty_enums::zzz (00007ff7`b9822fa0)
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
        [+0x000] __0              : 0x7ff7b98244a0 : 0x1 [Type: unsigned int *]
    [+0x000] discriminant     : 0x7ff7b98244a0 [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Discriminant$]
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
        [+0x004] __0              : 0x1f2 [Type: unsigned int]
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
    [+0x000] discriminant     : 0x1f2f549f3d0 [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>::Discriminant$]
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


------------------------------------------


---- [debuginfo-cdb] debuginfo\pretty-std-collections-hash.rs stdout ----

error: line not found in debugger output: hash_set,d [...] : { len=15 } [Type: [...]::HashSet<u64, [...]>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std-collections-hash.cdb\\pretty-std-collections-hash.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std-collections-hash.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`42be0000 00007ff7`42bf9000   a.exe   
ModLoad: 00007ff9`3c9c0000 00007ff9`3cbad000   ntdll.dll
ModLoad: 00007ff9`39e70000 00007ff9`39f23000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`395f0000 00007ff9`39885000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`36bf0000 00007ff9`36c7c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`393f0000 00007ff9`394ea000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`2d6d0000 00007ff9`2d6eb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`0a740000 00007ff9`0abee000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-f021d642ed65bd1b.dll
ModLoad: 00007ff9`3a2a0000 00007ff9`3a30d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`3a390000 00007ff9`3a4af000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`3c740000 00007ff9`3c7e4000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`39f40000 00007ff9`39fde000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`39dd0000 00007ff9`39e6e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`388d0000 00007ff9`388f9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`38a20000 00007ff9`38a44000   C:\Windows\System32\profapi.dll
ModLoad: 00007ff9`38350000 00007ff9`3835c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`38a70000 00007ff9`38af2000   C:\Windows\System32\bcryptPrimitives.dll
(17cc.1d4c): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`3ca9260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Fri Jul 16 13:34:28.658 2021 (UTC + 0:00)
System Uptime: 0 days 5:47:01.301
Process Uptime: 0 days 0:00:00.567
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\pretty-std-collections-hash.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\a.exe'  Debugger Process 0xFCC 
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
0:000> bp `pretty-std-collections-hash.rs:103`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!pretty_std_collections_hash::main+0x145:
00007ff7`42be9b15 e8c6000000      call    a!pretty_std_collections_hash::zzz (00007ff7`42be9be0)
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 0 [Type: u64]
    [2]              : 1 [Type: u64]
    [3]              : 3 [Type: u64]
    [4]              : 11 [Type: u64]
    [5]              : 14 [Type: u64]
    [6]              : 4 [Type: u64]
    [7]              : 6 [Type: u64]
    [8]              : 13 [Type: u64]
    [9]              : 7 [Type: u64]
    [10]             : 5 [Type: u64]
    [11]             : 2 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 10 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 0 [Type: u64]
    [2]              : 1 [Type: u64]
    [3]              : 3 [Type: u64]
    [4]              : 11 [Type: u64]
    [5]              : 14 [Type: u64]
    [6]              : 4 [Type: u64]
    [7]              : 6 [Type: u64]
    [8]              : 13 [Type: u64]
    [9]              : 7 [Type: u64]
    [10]             : 5 [Type: u64]
    [11]             : 2 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 10 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 0 [Type: u64]
    [2]              : 1 [Type: u64]
    [3]              : 3 [Type: u64]
    [4]              : 11 [Type: u64]
    [5]              : 14 [Type: u64]
    [6]              : 4 [Type: u64]
    [7]              : 6 [Type: u64]
    [8]              : 13 [Type: u64]
    [9]              : 7 [Type: u64]
    [10]             : 5 [Type: u64]
    [11]             : 2 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 10 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 0 [Type: u64]
    [2]              : 1 [Type: u64]
    [3]              : 3 [Type: u64]
    [4]              : 11 [Type: u64]
    [5]              : 14 [Type: u64]
    [6]              : 4 [Type: u64]
    [7]              : 6 [Type: u64]
    [8]              : 13 [Type: u64]
    [9]              : 7 [Type: u64]
    [10]             : 5 [Type: u64]
    [11]             : 2 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 10 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 0 [Type: u64]
    [2]              : 1 [Type: u64]
    [3]              : 3 [Type: u64]
    [4]              : 11 [Type: u64]
    [5]              : 14 [Type: u64]
    [6]              : 4 [Type: u64]
    [7]              : 6 [Type: u64]
    [8]              : 13 [Type: u64]
    [9]              : 7 [Type: u64]
    [10]             : 5 [Type: u64]
    [11]             : 2 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 10 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64,std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 0 [Type: u64]
    [2]              : 1 [Type: u64]
    [3]              : 3 [Type: u64]
    [4]              : 11 [Type: u64]
    [5]              : 14 [Type: u64]
    [6]              : 4 [Type: u64]
    [7]              : 6 [Type: u64]
    [8]              : 13 [Type: u64]
    [9]              : 7 [Type: u64]
    [10]             : 5 [Type: u64]
    [11]             : 2 [Type: u64]
---
    [debuginfo-cdb] debuginfo\result-types.rs

test result: FAILED. 106 passed; 5 failed; 19 ignored; 0 measured; 0 filtered out; finished in 25.23s

make: *** [Makefile:72: ci-subset-1] Error 1


command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.6\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.6\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:30:53
