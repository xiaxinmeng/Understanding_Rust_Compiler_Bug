plain
failures:

---- [debuginfo-cdb] debuginfo\basic-types.rs stdout ----

error: line not found in debugger output: s                : "Hello, World!" [Type: str]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\basic-types.cdb\\basic-types.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\basic-types.cdb\\a.exe"
------------------------------------------
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc


Microsoft (R) Windows Debugger Version 10.0.20348.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\basic-types.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff6`33040000 00007ff6`33047000   a.exe   
ModLoad: 00007ffc`9e1b0000 00007ffc`9e39d000   ntdll.dll
ModLoad: 00007ffc`9b550000 00007ffc`9b603000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ffc`9b020000 00007ffc`9b2b5000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ffc`983f0000 00007ffc`9847c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ffc`9a640000 00007ffc`9a73a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ffc`8e2d0000 00007ffc`8e2eb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffc`6c380000 00007ffc`6c84d000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-5ce8943ec2bee4a1.dll
ModLoad: 00007ffc`9bd30000 00007ffc`9bd9d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ffc`9bb60000 00007ffc`9bc7f000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ffc`9bc80000 00007ffc`9bd24000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ffc`9ba60000 00007ffc`9bafe000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ffc`9b3e0000 00007ffc`9b47e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ffc`9a090000 00007ffc`9a0b9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffc`9a190000 00007ffc`9a1b4000   C:\Windows\System32\profapi.dll
ModLoad: 00007ffc`99b50000 00007ffc`99b5c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ffc`9a840000 00007ffc`9a8c2000   C:\Windows\System32\bcryptPrimitives.dll
(1588.1f04): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffc`9e28260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Thu Sep  9 18:46:39.342 2021 (UTC + 0:00)
System Uptime: 0 days 1:08:32.732
Process Uptime: 0 days 0:00:02.341
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.015
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.20348.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\basic-types.cdb\basic-types.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\basic-types.cdb\a.exe'  Debugger Process 0x1F84 
dbgeng:  image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 28900
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.20348.1, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `basic-types.rs:154`
*** WARNING: Unable to verify checksum for a.exe
Breakpoint 0 hit
Breakpoint 0 hit
a!basic_types::main+0x94:
00007ff6`330410a4 e817000000      call    a!basic_types::_zzz (00007ff6`330410c0)
0:000> dx b
b                : false [Type: bool]
0:000> dx i
i                : -1 [Type: __int64]
0:000> dx i8
i8               : 68 [Type: char]
0:000> dx i16
i16              : -16 [Type: short]
0:000> dx i32
i32              : -32 [Type: int]
0:000> dx i64
i64              : -64 [Type: __int64]
0:000> dx u
u                : 0x1 [Type: unsigned __int64]
0:000> dx u8
u8               : 0x64 [Type: unsigned char]
0:000> dx u16
u16              : 0x10 [Type: unsigned short]
0:000> dx u32
u32              : 0x20 [Type: unsigned int]
0:000> dx u64
u64              : 0x40 [Type: unsigned __int64]
0:000> dx f32
f32              : 2.500000 [Type: float]
0:000> dx f64
f64              : 3.500000 [Type: double]
0:000> .enable_unicode 1
0:000> dx  s
s                : 72 [Type: str]
    [<Raw View>]     [Type: str]
    [len]            : 0xd [Type: unsigned __int64]
    [chars]         
0:000> 
s                : 72 [Type: str]
    [<Raw View>]     [Type: str]
    [len]            : 0xd [Type: unsigned __int64]
    [chars]         
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\ObjectiveC.natvis'
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

error: line not found in debugger output: a,!              [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\msvc-pretty-enums.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.20348.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`dd710000 00007ff7`dd719000   a.exe   
ModLoad: 00007ffc`9e1b0000 00007ffc`9e39d000   ntdll.dll
ModLoad: 00007ffc`9b550000 00007ffc`9b603000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ffc`9b020000 00007ffc`9b2b5000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ffc`983f0000 00007ffc`9847c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ffc`9a640000 00007ffc`9a73a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ffc`8e2d0000 00007ffc`8e2eb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffc`6b740000 00007ffc`6bc0d000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-5ce8943ec2bee4a1.dll
ModLoad: 00007ffc`9bd30000 00007ffc`9bd9d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ffc`9bb60000 00007ffc`9bc7f000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ffc`9bc80000 00007ffc`9bd24000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ffc`9ba60000 00007ffc`9bafe000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ffc`9b3e0000 00007ffc`9b47e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ffc`9a090000 00007ffc`9a0b9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffc`9a190000 00007ffc`9a1b4000   C:\Windows\System32\profapi.dll
ModLoad: 00007ffc`99b50000 00007ffc`99b5c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ffc`9a840000 00007ffc`9a8c2000   C:\Windows\System32\bcryptPrimitives.dll
(10f4.1058): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffc`9e28260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Thu Sep  9 18:46:47.360 2021 (UTC + 0:00)
System Uptime: 0 days 1:08:40.749
Process Uptime: 0 days 0:00:00.068
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.20348.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\msvc-pretty-enums.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe'  Debugger Process 0xB80 
dbgeng:  image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 28900
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.20348.1, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `msvc-pretty-enums.rs:121`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!msvc_pretty_enums::main+0x85:
00007ff7`dd712f85 e846000000      call    a!msvc_pretty_enums::zzz (00007ff7`dd712fd0)
0:000>  dx -r2 a,!
a,!              :  Some({...}) [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Some]
        [+0x000] __0              : Low (0x2) [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : 0x2 [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Discriminant$]
0:000>  dx -r2 b,!
b,!              : None [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Some]
        [+0x000] __0              : 0x11 [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : None (0x11) [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Discriminant$]
0:000>  dx -r2 c,!
c,!              : Tag1 [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [+0x000] dataful_variant  [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
        [+0x000] my_data          : 0x11 [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : Tag1 (0x11) [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Discriminant$]
0:000>  dx -r2 d,!
d,!              :  Data({...}) [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [+0x000] dataful_variant  [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
        [+0x000] my_data          : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : 0x10 [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Discriminant$]
0:000>  dx -r2 e,!
e,!              : Tag2 [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [+0x000] dataful_variant  [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
        [+0x000] my_data          : 0x13 [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : Tag2 (0x13) [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Discriminant$]
0:000>  dx -r2 f,!
f,!              :  Some({...}) [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Some]
        [+0x000] __0              : 0x7ff7dd7144a0 [Type: unsigned int *]
    [+0x000] discriminant     : 0x7ff7dd7144a0 [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>::Discriminant$]
0:000>  dx -r2 g,!
g,!              : None [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
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
        [+0x004] __0              : 0x219 [Type: unsigned int]
    [+0x000] discriminant     : None (0x0) [Type: core::option::Option]
0:000>  dx i
i                : None [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : None
0:000>  dx j
j                : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
0:000>  dx -r2 k,!
k,!              :  Some({...}) [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>::Some]
        [+0x000] __0              [Type: alloc::string::String]
    [+0x000] discriminant     : 0x2194f9608f0 [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>::Discriminant$]
0:000>  dx -r2 l,!
l,!              :  Ok [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [+0x000] Ok               [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
        [+0x000] __0              : 0x2a [Type: unsigned int]
0:000> 
l,!              :  Ok [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [+0x000] Ok               [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
        [+0x000] __0              : 0x2a [Type: unsigned int]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\ObjectiveC.natvis'
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


---- [debuginfo-cdb] debuginfo\pretty-std.rs stdout ----

error: line not found in debugger output: os_string        : "IAMA OS string 😃" [Type: std::ffi::os_str::OsString]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\pretty-std.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.20348.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff6`93ae0000 00007ff6`93af1000   a.exe   
ModLoad: 00007ffc`9e1b0000 00007ffc`9e39d000   ntdll.dll
ModLoad: 00007ffc`9b550000 00007ffc`9b603000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ffc`9b020000 00007ffc`9b2b5000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ffc`983f0000 00007ffc`9847c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ffc`9a640000 00007ffc`9a73a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ffc`8e2d0000 00007ffc`8e2eb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffc`6b740000 00007ffc`6bc0d000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-5ce8943ec2bee4a1.dll
ModLoad: 00007ffc`9bd30000 00007ffc`9bd9d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ffc`9bb60000 00007ffc`9bc7f000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ffc`9bc80000 00007ffc`9bd24000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ffc`9ba60000 00007ffc`9bafe000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ffc`9b3e0000 00007ffc`9b47e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ffc`9a090000 00007ffc`9a0b9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffc`9a190000 00007ffc`9a1b4000   C:\Windows\System32\profapi.dll
ModLoad: 00007ffc`99b50000 00007ffc`99b5c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ffc`9a840000 00007ffc`9a8c2000   C:\Windows\System32\bcryptPrimitives.dll
(38c.1710): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffc`9e28260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Thu Sep  9 18:46:48.892 2021 (UTC + 0:00)
System Uptime: 0 days 1:08:42.281
Process Uptime: 0 days 0:00:00.066
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.20348.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\pretty-std.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe'  Debugger Process 0xA20 
dbgeng:  image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 28900
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.20348.1, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `pretty-std.rs:184`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!pretty_std::main+0x146:
00007ff6`93ae42e6 e865010000      call    a!pretty_std::zzz (00007ff6`93ae4450)
0:000>  dx slice,d
slice,d          : { len=4 } [Type: slice$<i32>]
    [<Raw View>]     [Type: slice$<i32>]
    [len]            : 4 [Type: unsigned __int64]
    [0]              : 0 [Type: int]
    [1]              : 1 [Type: int]
    [2]              : 2 [Type: int]
    [3]              : 3 [Type: int]
0:000>  dx vec,d
vec,d            : { len=4 } [Type: alloc::vec::Vec<u64,alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::vec::Vec<u64,alloc::alloc::Global>]
    [len]            : 4 [Type: unsigned __int64]
    [capacity]       : 4 [Type: unsigned __int64]
    [0]              : 4 [Type: unsigned __int64]
    [1]              : 5 [Type: unsigned __int64]
    [2]              : 6 [Type: unsigned __int64]
    [3]              : 7 [Type: unsigned __int64]
0:000>  dx str_slice
str_slice        : "IAMA string slice!" [Type: str]
    [<Raw View>]     [Type: str]
    [len]            : 0x12 [Type: unsigned __int64]
    [chars]         
0:000>  dx string
string           : "IAMA string!" [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned __int64]
    [capacity]       : 0xc [Type: unsigned __int64]
    [chars]          : "IAMA string!"
0:000>  dx -r2 string
string           : "IAMA string!" [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned __int64]
    [capacity]       : 0xc [Type: unsigned __int64]
    [chars]          : "IAMA string!"
        [0]              : 73 'I' [Type: char]
        [1]              : 65 'A' [Type: char]
        [2]              : 77 'M' [Type: char]
        [3]              : 65 'A' [Type: char]
        [4]              : 32 ' ' [Type: char]
        [5]              : 115 's' [Type: char]
        [6]              : 116 't' [Type: char]
        [7]              : 114 'r' [Type: char]
        [8]              : 105 'i' [Type: char]
        [9]              : 110 'n' [Type: char]
        [10]             : 103 'g' [Type: char]
        [11]             : 33 '!' [Type: char]
0:000>  dx os_string
os_string        : "IAMA OS string �.�" [Type: std::ffi::os_str::OsString]
    [<Raw View>]     [Type: std::ffi::os_str::OsString]
    [chars]          : "IAMA OS string �.�"
0:000>  dx some
some             : Some [Type: enum$<core::option::Option<i16> >]
    [<Raw View>]     [Type: enum$<core::option::Option<i16> >]
    [variant]        : Some
    [+0x002] __0              : 8 [Type: short]
0:000>  dx none
none             : None [Type: enum$<core::option::Option<i64> >]
    [<Raw View>]     [Type: enum$<core::option::Option<i64> >]
    [variant]        : None
0:000>  dx some_string
some_string      :  Some({...}) [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>]
    [variant]        :  Some
    [+0x000] __0              : "IAMA optional string!" [Type: alloc::string::String]
0:000>  dx linkedlist
linkedlist       : { len=0x2 } [Type: alloc::collections::linked_list::LinkedList<i32>]
    [<Raw View>]     [Type: alloc::collections::linked_list::LinkedList<i32>]
    [0x0]            : 128 [Type: int]
    [0x1]            : 42 [Type: int]
0:000>  dx vecdeque
vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [len]            : 0x2
    [capacity]       : 0x8 [Type: unsigned __int64]
    [0x0]            : 90 [Type: int]
    [0x1]            : 20 [Type: int]
0:000> 
vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [len]            : 0x2
    [capacity]       : 0x8 [Type: unsigned __int64]
    [0x0]            : 90 [Type: int]
    [0x1]            : 20 [Type: int]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\ObjectiveC.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\windows.natvis'
---
test result: FAILED. 111 passed; 3 failed; 19 ignored; 0 measured; 0 filtered out; finished in 18.25s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"



make: *** [Makefile:72: ci-subset-1] Error 1
