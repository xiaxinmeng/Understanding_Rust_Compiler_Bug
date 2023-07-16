plain
test [debuginfo-cdb] debuginfo\basic-types-globals-metadata.rs ... ok

failures:

---- [debuginfo-cdb] debuginfo\mutable-locs.rs stdout ----

error: line not found in debugger output:     [+0x008] value            [Type: core::cell::UnsafeCell<i32>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\mutable-locs.cdb\\mutable-locs.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\mutable-locs.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\mutable-locs.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00030000 00036000   a.exe   
ModLoad: 77200000 7739c000   ntdll.dll
ModLoad: 75fb0000 76090000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 751c0000 753bb000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71c60000 71cfc000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 74a70000 74b93000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71d80000 71d94000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 708a0000 70c79000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-36f377ca5816bf82.dll
ModLoad: 76760000 767bf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 74990000 74a4e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74850000 74870000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74840000 7484a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 767e0000 76842000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 765b0000 76629000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 74910000 7498e000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 76850000 76910000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71da0000 71dc5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76630000 7664c000   C:\Windows\SysWOW64\profapi.dll
(198c.1d24): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=00bb8000 ecx=c4cf0000 edx=00000000 esi=001e5e58 edi=7721382c
eip=772aea26 esp=00cff718 ebp=00cff744 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
ntdll!LdrpDoDebuggerBreak+0x2b:
772aea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Sun May 30 07:22:39.185 2021 (UTC + 0:00)
System Uptime: 0 days 7:21:25.053
Process Uptime: 0 days 0:00:00.597
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.031
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\mutable-locs.cdb\mutable-locs.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\mutable-locs.cdb\a.exe'  Debugger Process 0xFCC 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.19041.685, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
    ext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\ext.dll]
    wow64exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\wow64exts.dll]
    exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\exts.dll]
    uext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\uext.dll]
    ntsdexts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\ntsdexts.dll]
WOW64 extensions loaded
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `mutable-locs.rs:47`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=00cffb44 ebx=00cffc08 ecx=00cffb48 edx=00cffb40 esi=00cffbb8 edi=00033108
eip=0003114c esp=00cffb24 ebp=00cffb6c iopl=0         nv up ei pl nz na po nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000202
a!mutable_locs::main+0xcc:
0003114c e84f000000      call    a!mutable_locs::zzz (000311a0)
0:000> dx static_c,d
static_c,d       [Type: core::cell::Cell<i32>]
    [+0x000] value            [Type: core::cell::UnsafeCell<i32>]
0:000>  dx static_c.value,d
static_c.value,d [Type: core::cell::UnsafeCell<i32>]
    [+0x000] value            : 10 [Type: int]
0:000>   dx dynamic_c,d
dynamic_c,d      [Type: core::cell::RefCell<i32>]
    [+0x000] borrow           [Type: core::cell::Cell<isize>]
    [+0x004] value            [Type: core::cell::UnsafeCell<i32>]
0:000>  dx dynamic_c.value,d
dynamic_c.value,d [Type: core::cell::UnsafeCell<i32>]
    [+0x000] value            : 15 [Type: int]
0:000>  dx b,d
b,d              [Type: core::cell::RefMut<i32>]
    [+0x000] value            : 0xcffb44 : 42 [Type: int *]
    [+0x004] borrow           [Type: core::cell::BorrowRefMut]
0:000> 
b,d              [Type: core::cell::RefMut<i32>]
    [+0x000] value            : 0xcffb44 : 42 [Type: int *]
    [+0x004] borrow           [Type: core::cell::BorrowRefMut]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\winrt.natvis'
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-cdb] debuginfo\pretty-std.rs stdout ----

error: line not found in debugger output:     [len]            : 4 [Type: unsigned __int64]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\pretty-std.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00af0000 00af9000   a.exe   
ModLoad: 77200000 7739c000   ntdll.dll
ModLoad: 75fb0000 76090000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 751c0000 753bb000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71c60000 71cfc000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 74a70000 74b93000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71d80000 71d94000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 708a0000 70c79000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-36f377ca5816bf82.dll
ModLoad: 76760000 767bf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 74990000 74a4e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74850000 74870000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74840000 7484a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 767e0000 76842000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 765b0000 76629000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 74910000 7498e000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 76850000 76910000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71da0000 71dc5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76630000 7664c000   C:\Windows\SysWOW64\profapi.dll
(1238.b2c): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=010f5000 ecx=308f0000 edx=00000000 esi=00b25e58 edi=7721382c
eip=772aea26 esp=012ff808 ebp=012ff834 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
772aea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Sun May 30 07:22:39.872 2021 (UTC + 0:00)
System Uptime: 0 days 7:21:25.740
Process Uptime: 0 days 0:00:00.578
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\pretty-std.cdb\pretty-std.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe'  Debugger Process 0x3C0 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.19041.685, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
    ext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\ext.dll]
    wow64exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\wow64exts.dll]
    exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\exts.dll]
    uext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\uext.dll]
    ntsdexts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\ntsdexts.dll]
WOW64 extensions loaded
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `pretty-std.rs:151`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=00000015 ebx=012ffcf8 ecx=012ffc20 edx=00000015 esi=012ffbc0 edi=00af520c
eip=00af2b04 esp=012ffbc0 ebp=012ffc5c iopl=0         nv up ei pl nz na po nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000202
a!pretty_std::main+0x164:
00af2b04 c7868800000003000000 mov dword ptr [esi+88h],3 ds:002b:012ffc48=00000002
0:000>  dx slice,d
slice,d          : { len=4 } [Type: slice<i32>]
    [<Raw View>]     [Type: slice<i32>]
    [len]            : 4 [Type: unsigned int]
    [0]              : 0 [Type: int]
    [1]              : 1 [Type: int]
    [2]              : 2 [Type: int]
    [3]              : 3 [Type: int]
0:000>  dx vec,d
vec,d            : { len=4 } [Type: alloc::vec::Vec<u64, alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::vec::Vec<u64, alloc::alloc::Global>]
    [len]            : 4 [Type: unsigned int]
    [capacity]       : 4 [Type: unsigned int]
    [0]              : 4 [Type: unsigned __int64]
    [1]              : 5 [Type: unsigned __int64]
    [2]              : 6 [Type: unsigned __int64]
    [3]              : 7 [Type: unsigned __int64]
0:000>  dx str_slice
str_slice        : "IAMA string slice!" [Type: str]
    [<Raw View>]     [Type: str]
    [len]            : 0x12 [Type: unsigned int]
    [chars]         
0:000>  dx string
string           : "IAMA string!" [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned int]
    [capacity]       : 0xc [Type: unsigned int]
    [chars]         
0:000>  dx -r2 string
string           : "IAMA string!" [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned int]
    [capacity]       : 0xc [Type: unsigned int]
    [chars]         
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
os_string        [Type: std::ffi::os_str::OsString]
    [+0x000] inner            [Type: std::sys::windows::os_str::Buf]
0:000>  dx some
some             : Some(8) [Type: core::option::Option<i16>]
    [<Raw View>]     [Type: core::option::Option<i16>]
    [value]          : 8 [Type: short]
0:000>  dx none
none             : None [Type: core::option::Option<i64>]
    [<Raw View>]     [Type: core::option::Option<i64>]
0:000>  dx some_string
some_string      : Some("IAMA optional string!") [Type: core::option::Option<alloc::string::String>]
    [<Raw View>]     [Type: core::option::Option<alloc::string::String>]
    Some             : 0x12ffc10 : "IAMA optional string!" [Type: alloc::string::String *]
0:000> 
some_string      : Some("IAMA optional string!") [Type: core::option::Option<alloc::string::String>]
    [<Raw View>]     [Type: core::option::Option<alloc::string::String>]
    Some             : 0x12ffc10 : "IAMA optional string!" [Type: alloc::string::String *]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\winrt.natvis'
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-cdb] debuginfo\result-types.rs stdout ----

error: line not found in debugger output:     [value]          [Type: str]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\result-types.cdb\\result-types.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\result-types.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\result-types.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 01010000 01016000   a.exe   
ModLoad: 77200000 7739c000   ntdll.dll
ModLoad: 75fb0000 76090000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 751c0000 753bb000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71c60000 71cfc000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 74a70000 74b93000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71d80000 71d94000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 708a0000 70c79000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-36f377ca5816bf82.dll
ModLoad: 76760000 767bf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 74990000 74a4e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74850000 74870000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74840000 7484a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 767e0000 76842000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 765b0000 76629000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 74910000 7498e000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 76850000 76910000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71da0000 71dc5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76630000 7664c000   C:\Windows\SysWOW64\profapi.dll
(ea0.1538): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=00cba000 ecx=1eae0000 edx=00000000 esi=00135e58 edi=7721382c
eip=772aea26 esp=00bdf3fc ebp=00bdf428 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
772aea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Sun May 30 07:22:40.887 2021 (UTC + 0:00)
System Uptime: 0 days 7:21:26.754
Process Uptime: 0 days 0:00:00.579
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.031
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\result-types.cdb\result-types.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\result-types.cdb\a.exe'  Debugger Process 0xE48 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.19041.685, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
    ext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\ext.dll]
    wow64exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\wow64exts.dll]
    exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\exts.dll]
    uext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\uext.dll]
    ntsdexts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\ntsdexts.dll]
WOW64 extensions loaded
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `result-types.rs:23`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=00bdf8fe ebx=00bdf8ec ecx=01013166 edx=00bdf8b8 esi=00bdf89c edi=01013178
eip=010111ad esp=00bdf7bc ebp=00bdf880 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
a!result_types::main+0xfd:
010111ad e84e000000      call    a!result_types::zzz (01011200)
0:000>  dx x,d
x,d              : Ok(-3) [Type: core::result::Result<i32, str>]
    [<Raw View>]     [Type: core::result::Result<i32, str>]
    [value]          : -3 [Type: int]
0:000>  dx y
y                : Err("Some error message") [Type: core::result::Result<i32, str>]
    [<Raw View>]     [Type: core::result::Result<i32, str>]
    [value]          : "Some error message" [Type: str]
0:000> 
y                : Err("Some error message") [Type: core::result::Result<i32, str>]
    [<Raw View>]     [Type: core::result::Result<i32, str>]
    [value]          : "Some error message" [Type: str]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\winrt.natvis'
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-cdb] debuginfo\rwlock-read.rs stdout ----

error: line not found in debugger output:     [+0x008] poison           [Type: std::sync::poison::Flag]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\rwlock-read.cdb\\rwlock-read.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\rwlock-read.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rwlock-read.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 000d0000 000d8000   a.exe   
ModLoad: 77200000 7739c000   ntdll.dll
ModLoad: 75fb0000 76090000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 751c0000 753bb000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71c60000 71cfc000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 74a70000 74b93000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71d80000 71d94000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 708a0000 70c79000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-36f377ca5816bf82.dll
ModLoad: 76760000 767bf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 74990000 74a4e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74850000 74870000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74840000 7484a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 767e0000 76842000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 765b0000 76629000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 74910000 7498e000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 76850000 76910000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71da0000 71dc5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76630000 7664c000   C:\Windows\SysWOW64\profapi.dll
(968.1820): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=00434000 ecx=64430000 edx=00000000 esi=00975e58 edi=7721382c
eip=772aea26 esp=0075f91c ebp=0075f948 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
772aea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Sun May 30 07:22:40.923 2021 (UTC + 0:00)
System Uptime: 0 days 7:21:26.790
Process Uptime: 0 days 0:00:00.580
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.015
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rwlock-read.cdb\rwlock-read.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rwlock-read.cdb\a.exe'  Debugger Process 0xE8 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
