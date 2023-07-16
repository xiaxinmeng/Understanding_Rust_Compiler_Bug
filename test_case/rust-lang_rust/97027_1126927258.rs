plain
failures:

---- [debuginfo-cdb] src/test\debuginfo\mutable-locs.rs stdout ----

error: line not found in debugger output: b,d              : 42 [Type: core::cell::RefMut<i32>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.17.10\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\mutable-locs.cdb\\mutable-locs.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\mutable-locs.cdb\\a.exe"
--- stdout -------------------------------
Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\mutable-locs.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`c2210000 00007ff7`c2218000   a.exe   
ModLoad: 00007ffe`92030000 00007ffe`9221d000   ntdll.dll
ModLoad: 00007ffe`90020000 00007ffe`900d3000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ffe`8eec0000 00007ffe`8f154000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ffe`83560000 00007ffe`835ec000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ffe`8f160000 00007ffe`8f25a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ffe`84850000 00007ffe`8486b000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffe`557c0000 00007ffe`55d84000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-6396560c2e020b10.dll
ModLoad: 00007ffe`8f870000 00007ffe`8f917000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ffe`8fbb0000 00007ffe`8fc4e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ffe`8f620000 00007ffe`8f6bf000   C:\Windows\System32\sechost.dll
ModLoad: 00007ffe`8f920000 00007ffe`8fa3d000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ffe`8f5b0000 00007ffe`8f61d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ffe`8e990000 00007ffe`8e9b6000   C:\Windows\System32\bcrypt.dll
ModLoad: 00007ffe`8df20000 00007ffe`8df49000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffe`8e010000 00007ffe`8e034000   C:\Windows\System32\profapi.dll
(1f00.d80): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffe`9210260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Sun May 15 11:18:05.176 2022 (UTC + 0:00)
System Uptime: 0 days 2:54:13.867
Process Uptime: 0 days 0:00:00.053
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\mutable-locs.cdb\mutable-locs.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\mutable-locs.cdb\a.exe'  Debugger Process 0x860 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.17.10\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
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
0:000> bp `mutable-locs.rs:86`
*** WARNING: Unable to verify checksum for a.exe
0:000> bp `mutable-locs.rs:91`
0:000> bp `mutable-locs.rs:94`
0:000> bp `mutable-locs.rs:98`
0:000> bp `mutable-locs.rs:101`
0:000>  g
Breakpoint 0 hit
a!mutable_locs::main+0x9d:
00007ff7`c2211d5d e82e010000      call    a!mutable_locs::zzz (00007ff7`c2211e90)
0:000> dx static_c,d
static_c,d       : 10 [Type: core::cell::Cell<i32>]
    [<Raw View>]     [Type: core::cell::Cell<i32>]
0:000>  dx static_c.value,d
static_c.value,d : 10 [Type: core::cell::UnsafeCell<i32>]
    [<Raw View>]     [Type: core::cell::UnsafeCell<i32>]
0:000>   dx dynamic_c,d
dynamic_c,d      : 15 [Type: core::cell::RefCell<i32>]
    [<Raw View>]     [Type: core::cell::RefCell<i32>]
    [Borrow state]   : Unborrowed
0:000>  dx dynamic_c.value,d
dynamic_c.value,d : 15 [Type: core::cell::UnsafeCell<i32>]
    [<Raw View>]     [Type: core::cell::UnsafeCell<i32>]
0:000>  dx b,d
b,d              : NonNull(0xfd8951f8b0: 42) [Type: core::cell::RefMut<i32>]
    [<Raw View>]     [Type: core::cell::RefMut<i32>]
    42 [Type: int]
0:000>  g
Breakpoint 1 hit
a!mutable_locs::main+0xce:
00007ff7`c2211d8e e8fd000000      call    a!mutable_locs::zzz (00007ff7`c2211e90)
0:000>  dx dynamic_c,d
dynamic_c,d      : 15 [Type: core::cell::RefCell<i32>]
    [<Raw View>]     [Type: core::cell::RefCell<i32>]
    [Borrow state]   : Immutably borrowed
0:000>  dx r_borrow,d
r_borrow,d       : NonNull(0xfd8951f8a0: 15) [Type: core::cell::Ref<i32>]
    [<Raw View>]     [Type: core::cell::Ref<i32>]
    15 [Type: int]
0:000>  g
Breakpoint 2 hit
a!mutable_locs::main+0xe0:
00007ff7`c2211da0 e8eb000000      call    a!mutable_locs::zzz (00007ff7`c2211e90)
0:000>  dx dynamic_c,d
dynamic_c,d      : 15 [Type: core::cell::RefCell<i32>]
    [<Raw View>]     [Type: core::cell::RefCell<i32>]
    [Borrow state]   : Unborrowed
0:000>  g
Breakpoint 3 hit
a!mutable_locs::main+0x111:
00007ff7`c2211dd1 e8ba000000      call    a!mutable_locs::zzz (00007ff7`c2211e90)
0:000>  dx dynamic_c,d
dynamic_c,d      : 15 [Type: core::cell::RefCell<i32>]
    [<Raw View>]     [Type: core::cell::RefCell<i32>]
    [Borrow state]   : Mutably borrowed
0:000>  dx r_borrow_mut,d
r_borrow_mut,d   : NonNull(0xfd8951f8a0: 15) [Type: core::cell::RefMut<i32>]
    [<Raw View>]     [Type: core::cell::RefMut<i32>]
    15 [Type: int]
0:000>  g
Breakpoint 4 hit
a!mutable_locs::main+0x123:
00007ff7`c2211de3 e8a8000000      call    a!mutable_locs::zzz (00007ff7`c2211e90)
0:000>  dx dynamic_c,d
dynamic_c,d      : 15 [Type: core::cell::RefCell<i32>]
    [<Raw View>]     [Type: core::cell::RefCell<i32>]
    [Borrow state]   : Unborrowed
0:000> 
dynamic_c,d      : 15 [Type: core::cell::RefCell<i32>]
    [<Raw View>]     [Type: core::cell::RefCell<i32>]
    [Borrow state]   : Unborrowed
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
    [debuginfo-cdb] src/test\debuginfo\mutable-locs.rs

test result: FAILED. 118 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 17.28s

Build completed unsuccessfully in 0:27:17
make: *** [Makefile:70: ci-subset-1] Error 1
