plain
test [codegen] src/test\codegen\naked-noinline.rs ... ok
test [codegen] src/test\codegen\no-assumes-on-casts.rs ... ok
test [codegen] src/test\codegen\no-dllimport-w-cross-lang-lto.rs ... ok
test [codegen] src/test\codegen\no-plt.rs ... ok
test [codegen] src/test\codegen\noalias-rwlockreadguard.rs ... ok
test [codegen] src/test\codegen\noalias-unpin.rs ... ok
test [codegen] src/test\codegen\nounwind.rs ... ignored
test [codegen] src/test\codegen\non-terminate\infinite-loop-1.rs ... ok
test [codegen] src/test\codegen\non-terminate\infinite-loop-2.rs ... ok
---
failures:

---- [debuginfo-cdb] src/test\debuginfo\rwlock-read.rs stdout ----

error: line not found in debugger output:     [...] lock             : [...] [Type: std::sync::rwlock::RwLock<i32> *]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\rwlock-read.cdb\\rwlock-read.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\rwlock-read.cdb\\a.exe"
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Microsoft (R) Windows Debugger Version 10.0.22000.194 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rwlock-read.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00c00000 00c06000   a.exe   
ModLoad: 777d0000 7796c000   ntdll.dll
ModLoad: 76000000 760e0000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 75170000 7536c000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 72090000 7212c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 75df0000 75f13000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 721e0000 721f5000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 700a0000 70643000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-222683428a92a8b8.dll
ModLoad: 75520000 755a0000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 75f30000 75ff0000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 75600000 75679000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 750b0000 7516d000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74e10000 74e30000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74e00000 74e0a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 76630000 76695000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 77310000 7736f000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 72200000 72225000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76170000 76189000   C:\Windows\SysWOW64\bcrypt.dll
ModLoad: 76610000 7662c000   C:\Windows\SysWOW64\profapi.dll
(10cc.894): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=00b81000 ecx=4aa70000 edx=00000000 esi=00156338 edi=777e383c
eip=7787ea46 esp=009bf7fc ebp=009bf828 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
7787ea46 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.x86fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Wed Jun 22 06:05:41.089 2022 (UTC + 0:00)
System Uptime: 0 days 7:05:04.674
Process Uptime: 0 days 0:00:00.082
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.015
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rwlock-read.cdb\rwlock-read.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rwlock-read.cdb\a.exe'  Debugger Process 0x51C 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.22000.194, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
    ext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\ext.dll]
    wow64exts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\wow64exts.dll]
    exts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\exts.dll]
    uext: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\uext.dll]
    ntsdexts: image 10.0.22000.194, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\ntsdexts.dll]
WOW64 extensions loaded
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `rwlock-read.rs:32`
*** WARNING: Unable to verify checksum for a.exe
Breakpoint 0 hit
Breakpoint 0 hit
eax=009bfbac ebx=00b81000 ecx=009bfbc8 edx=009bfba4 esi=009bfce8 edi=00175038
eip=00c01363 esp=009bfb94 ebp=009bfbe0 iopl=0         nv up ei pl nz ac po nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000212
a!rwlock_read::main+0x73:
00c01363 c745f000000000  mov     dword ptr [ebp-10h],0 ss:002b:009bfbd0=ffffffff
0:000> dx l
l                [Type: std::sync::rwlock::RwLock<i32>]
    [+0x000] inner            [Type: std::sys_common::rwlock::MovableRwLock]
    [+0x004] poison           [Type: std::sync::poison::Flag]
    [+0x008] data             : 0 [Type: core::cell::UnsafeCell<i32>]
0:000> dx r
r                [Type: std::sync::rwlock::RwLockReadGuard<i32>]
    [+0x000] data             : NonNull(0x9bfbac: 0) [Type: core::ptr::non_null::NonNull<i32>]
    [+0x004] inner_lock       : 0x9bfba4 [Type: std::sys_common::rwlock::MovableRwLock *]
0:000> dx r.lock->data,d
Error: Unable to bind name 'lock'
Error: Unable to bind name 'lock'
0:000> qq
quit:
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\ObjectiveC.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\winrt.natvis'
stderr: none




failures:
    [debuginfo-cdb] src/test\debuginfo\rwlock-read.rs

test result: FAILED. 120 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 19.06s

Build completed unsuccessfully in 0:27:56
make: *** [Makefile:70: ci-subset-1] Error 1
