plain
failures:

---- [debuginfo-cdb] src/test\debuginfo\marker-types.rs stdout ----

error: line not found in debugger output: pin              : Pin(0x[...]: "this") [Type: core::pin::Pin<ref_mut$<alloc::string::String> >]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\marker-types.cdb\\marker-types.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\marker-types.cdb\\a.exe"
--- stdout -------------------------------
Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\marker-types.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`a3900000 00007ff7`a3909000   a.exe   
ModLoad: 00007ff9`b1880000 00007ff9`b1a6d000   ntdll.dll
ModLoad: 00007ff9`aeab0000 00007ff9`aeb64000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`adca0000 00007ff9`adf34000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`a73e0000 00007ff9`a746c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`ae910000 00007ff9`aea0a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`a13b0000 00007ff9`a13cb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`807e0000 00007ff9`80dab000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-88c540a6b7a4449a.dll
ModLoad: 00007ff9`b0bd0000 00007ff9`b0c3d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`af2c0000 00007ff9`af3dd000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`adc70000 00007ff9`adc96000   C:\Windows\System32\bcrypt.dll
ModLoad: 00007ff9`b0b20000 00007ff9`b0bc7000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`aecf0000 00007ff9`aed8e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`aeb70000 00007ff9`aec0f000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`ad740000 00007ff9`ad769000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`ad8e0000 00007ff9`ad904000   C:\Windows\System32\profapi.dll
(a6c.1890): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`b195260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Thu Apr 14 11:03:10.543 2022 (UTC + 0:00)
System Uptime: 0 days 15:24:03.314
Process Uptime: 0 days 0:00:00.069
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\marker-types.cdb\marker-types.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\marker-types.cdb\a.exe'  Debugger Process 0x1E84 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.22000.194, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.22000.194, API 1.0.0, 
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `marker-types.rs:46`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!marker_types::main+0x94:
00007ff7`a3901fb4 e847000000      call    a!marker_types::zzz (00007ff7`a3902000)
0:000>  dx nonnull
nonnull          : NonNull(0x7ff7a3904438: 0xc) [Type: core::ptr::non_null::NonNull<u32>]
    [<Raw View>]     [Type: core::ptr::non_null::NonNull<u32>]
    0xc [Type: unsigned int]
0:000>  dx manuallydrop
manuallydrop     : 12345 [Type: core::mem::manually_drop::ManuallyDrop<i32>]
    [<Raw View>]     [Type: core::mem::manually_drop::ManuallyDrop<i32>]
0:000>  dx pin
pin              : Pin(0x2120f9fbe0: {...}) [Type: core::pin::Pin<ref_mut$<alloc::string::String> >]
    [<Raw View>]     [Type: core::pin::Pin<ref_mut$<alloc::string::String> >]
    [len]            : 0x4 [Type: unsigned __int64]
    [capacity]       : 0x4 [Type: unsigned __int64]
    [chars]         
0:000>  dx unique
unique           [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [<Raw View>]     [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [0]              : 0x2a [Type: unsigned __int64]
    [1]              : 4321 [Type: int]
0:000> 
unique           [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [<Raw View>]     [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [0]              : 0x2a [Type: unsigned __int64]
    [1]              : 4321 [Type: int]
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
stderr: none


---- [debuginfo-cdb] src/test\debuginfo\msvc-pretty-enums.rs stdout ----
---- [debuginfo-cdb] src/test\debuginfo\msvc-pretty-enums.rs stdout ----

error: line not found in debugger output:     [+0x000] __0              : "IAMA optional string!" [Type: alloc::string::String]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\msvc-pretty-enums.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\a.exe"
--- stdout -------------------------------
Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`f36c0000 00007ff7`f36c9000   a.exe   
ModLoad: 00007ff9`b1880000 00007ff9`b1a6d000   ntdll.dll
ModLoad: 00007ff9`aeab0000 00007ff9`aeb64000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`adca0000 00007ff9`adf34000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`a73e0000 00007ff9`a746c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`ae910000 00007ff9`aea0a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`a13b0000 00007ff9`a13cb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`807e0000 00007ff9`80dab000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-88c540a6b7a4449a.dll
ModLoad: 00007ff9`b0bd0000 00007ff9`b0c3d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`af2c0000 00007ff9`af3dd000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`adc70000 00007ff9`adc96000   C:\Windows\System32\bcrypt.dll
ModLoad: 00007ff9`b0b20000 00007ff9`b0bc7000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`aecf0000 00007ff9`aed8e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`aeb70000 00007ff9`aec0f000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`ad740000 00007ff9`ad769000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`ad8e0000 00007ff9`ad904000   C:\Windows\System32\profapi.dll
(1adc.1424): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`b195260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Thu Apr 14 11:03:11.216 2022 (UTC + 0:00)
System Uptime: 0 days 15:24:03.988
Process Uptime: 0 days 0:00:00.068
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\msvc-pretty-enums.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe'  Debugger Process 0x1EDC 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.22000.194, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `msvc-pretty-enums.rs:97`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!msvc_pretty_enums::main+0x85:
00007ff7`f36c1f65 e846000000      call    a!msvc_pretty_enums::zzz (00007ff7`f36c1fb0)
0:000>  dx a
a                :  Some({...}) [Type: enum$<core::option::Option<msvc_pretty_enums::CStyleEnum>, 2, 16, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<msvc_pretty_enums::CStyleEnum>, 2, 16, Some>]
    [variant]        :  Some
    [+0x000] __0              : Low (0x2) [Type: msvc_pretty_enums::CStyleEnum]
0:000>  dx b
b                : None [Type: enum$<core::option::Option<msvc_pretty_enums::CStyleEnum>, 2, 16, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<msvc_pretty_enums::CStyleEnum>, 2, 16, Some>]
    [variant]        : None
0:000>  dx c
c                : Tag1 [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [<Raw View>]     [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [variant]        : Tag1
0:000>  dx d
d                :  Data({...}) [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [<Raw View>]     [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [variant]        :  Data
    [+0x000] my_data          : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
0:000>  dx e
e                : Tag2 [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [<Raw View>]     [Type: enum$<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
    [variant]        : Tag2
0:000>  dx f
f                :  Some({...}) [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [variant]        :  Some
    [+0x000] __0              : 0x7ff7f36c4438 : 0x1 [Type: unsigned int *]
0:000>  dx g
g                : None [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<ref$<u32> >, 1, 18446744073709551615, Some>]
    [variant]        : None
0:000>  dx h
h                : Some [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : Some
    [+0x004] __0              : 0xc [Type: unsigned int]
0:000>  dx i
i                : None [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : None
0:000>  dx j
j                : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
0:000>  dx k
k                :  Some({...}) [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<alloc::string::String>, 1, 18446744073709551615, Some>]
    [variant]        :  Some
    [+0x000] __0              [Type: alloc::string::String]
0:000>  dx l
l                :  Ok [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [<Raw View>]     [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [variant]        :  Ok
    [+0x000] __0              : 0x2a [Type: unsigned int]
0:000> 
l                :  Ok [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [<Raw View>]     [Type: enum$<core::result::Result<u32,enum$<msvc_pretty_enums::Empty> >, Ok>]
    [variant]        :  Ok
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
stderr: none


---- [debuginfo-cdb] src/test\debuginfo\pretty-std.rs stdout ----
---- [debuginfo-cdb] src/test\debuginfo\pretty-std.rs stdout ----

error: line not found in debugger output:     [0]              : 4 [Type: unsigned __int64]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\pretty-std.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\a.exe"
--- stdout -------------------------------
Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`b4cf0000 00007ff7`b4d03000   a.exe   
ModLoad: 00007ff9`b1880000 00007ff9`b1a6d000   ntdll.dll
ModLoad: 00007ff9`aeab0000 00007ff9`aeb64000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`adca0000 00007ff9`adf34000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`a73e0000 00007ff9`a746c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`ae910000 00007ff9`aea0a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`a13b0000 00007ff9`a13cb000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`807e0000 00007ff9`80dab000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-88c540a6b7a4449a.dll
ModLoad: 00007ff9`b0bd0000 00007ff9`b0c3d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`af2c0000 00007ff9`af3dd000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`adc70000 00007ff9`adc96000   C:\Windows\System32\bcrypt.dll
ModLoad: 00007ff9`b0b20000 00007ff9`b0bc7000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`aecf0000 00007ff9`aed8e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`aeb70000 00007ff9`aec0f000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`ad740000 00007ff9`ad769000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`ad8e0000 00007ff9`ad904000   C:\Windows\System32\profapi.dll
(410.a30): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`b195260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Thu Apr 14 11:03:13.302 2022 (UTC + 0:00)
System Uptime: 0 days 15:24:06.073
Process Uptime: 0 days 0:00:00.063
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\pretty-std.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe'  Debugger Process 0x188C 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.22000.194, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `pretty-std.rs:188`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!pretty_std::main+0x146:
00007ff7`b4cf4c16 e865010000      call    a!pretty_std::zzz (00007ff7`b4cf4d80)
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
0:000>  dx str_slice
str_slice        : "IAMA string slice!" [Type: str]
    [<Raw View>]     [Type: str]
    [len]            : 0x12 [Type: unsigned __int64]
    [chars]         
0:000>  dx string
string           [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned __int64]
    [capacity]       : 0xc [Type: unsigned __int64]
    [chars]         
0:000>  dx -r2 string
string           [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned __int64]
    [capacity]       : 0xc [Type: unsigned __int64]
    [chars]         
0:000>  dx os_string
os_string        [Type: std::ffi::os_str::OsString]
    [<Raw View>]     [Type: std::ffi::os_str::OsString]
    [chars]         
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
    [+0x000] __0              [Type: alloc::string::String]
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
    [0x0]            : Object does not support indexing with []
    [0x1]            : Object does not support indexing with []
0:000> 
vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [len]            : 0x2
    [capacity]       : 0x8 [Type: unsigned __int64]
    [0x0]            : Object does not support indexing with []
    [0x1]            : Object does not support indexing with []
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
stderr: none




failures:
    [debuginfo-cdb] src/test\debuginfo\marker-types.rs
    [debuginfo-cdb] src/test\debuginfo\msvc-pretty-enums.rs
    [debuginfo-cdb] src/test\debuginfo\pretty-std.rs

test result: FAILED. 114 passed; 3 failed; 18 ignored; 0 measured; 0 filtered out; finished in 18.81s

make: *** [Makefile:72: ci-subset-1] Error 1
