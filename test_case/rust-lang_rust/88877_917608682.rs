plain
test [debuginfo-cdb] debuginfo\unique-enum.rs ... ok

failures:

---- [debuginfo-cdb] debuginfo\msvc-scalarpair-params.rs stdout ----

error: line not found in debugger output:     [len]            : 0x14 [Type: unsigned __int64]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\msvc-scalarpair-params.cdb\\msvc-scalarpair-params.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\msvc-scalarpair-params.cdb\\a.exe"
------------------------------------------

Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Microsoft (R) Windows Debugger Version 10.0.20348.1 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\msvc-scalarpair-params.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00200000 00206000   a.exe   
ModLoad: 77d90000 77f2c000   ntdll.dll
ModLoad: 776f0000 777d0000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 774f0000 776ec000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 72800000 7289c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 76190000 762b3000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 72920000 72934000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 71310000 717c1000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-a37452cb25ae95d9.dll
ModLoad: 770c0000 7711f000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 755b0000 7566e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 753f0000 75410000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 753e0000 753ea000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 75670000 756d5000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 77ab0000 77b29000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 77370000 773ee000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 762c0000 76380000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 72940000 72965000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 77b30000 77b4c000   C:\Windows\SysWOW64\profapi.dll
(191c.1648): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=011de000 ecx=73fa0000 edx=00000000 esi=00295fb8 edi=77da383c
eip=77e3ea46 esp=0135f9a0 ebp=0135f9cc iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
77e3ea46 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.x86fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Sun Sep 12 10:31:50.098 2021 (UTC + 0:00)
System Uptime: 0 days 3:58:48.324
Process Uptime: 0 days 0:00:00.119
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.015
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.20348.1 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\msvc-scalarpair-params.cdb\msvc-scalarpair-params.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\msvc-scalarpair-params.cdb\a.exe'  Debugger Process 0x1894 
dbgeng:  image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.20348.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 28900
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.20348.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.20348.1, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
    ext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\ext.dll]
    wow64exts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\wow64exts.dll]
    exts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\exts.dll]
    uext: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\uext.dll]
    ntsdexts: image 10.0.20348.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\ntsdexts.dll]
WOW64 extensions loaded
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `msvc-scalarpair-params.rs:61`
*** WARNING: Unable to verify checksum for a.exe
0:000> bp `msvc-scalarpair-params.rs:73`
0:000> bp `msvc-scalarpair-params.rs:77`
0:000> bp `msvc-scalarpair-params.rs:81`
0:000> bp `msvc-scalarpair-params.rs:85`
0:000> bp `msvc-scalarpair-params.rs:89`
0:000>  g
Breakpoint 0 hit
eax=0000001e ebx=0000000a ecx=00000000 edx=00000014 esi=00000000 edi=0000000c
eip=002012c5 esp=0135fd18 ebp=0135fd48 iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!msvc_scalarpair_params::range+0x35:
002012c5 e876010000      call    a!msvc_scalarpair_params::zzz (00201440)
0:000>  dx r1
r1               : (0xa..0xc) [Type: core::ops::range::Range<u32>]
    [<Raw View>]     [Type: core::ops::range::Range<u32>]
0:000>  dx r2
r2               : (0x14..0x1e) [Type: core::ops::range::Range<u64>]
    [<Raw View>]     [Type: core::ops::range::Range<u64>]
0:000>  g
Breakpoint 1 hit
eax=00000000 ebx=00000009 ecx=00000000 edx=0000000c esi=00000000 edi=00000014
eip=00201344 esp=0135fd18 ebp=0135fd48 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
a!msvc_scalarpair_params::range_mut+0x64:
00201344 e8f7000000      call    a!msvc_scalarpair_params::zzz (00201440)
0:000>  dx r1
r1               : (0x9..0x64) [Type: core::ops::range::Range<u32>]
    [<Raw View>]     [Type: core::ops::range::Range<u32>]
0:000>  dx r2
r2               : (0xc..0x5a) [Type: core::ops::range::Range<u64>]
    [<Raw View>]     [Type: core::ops::range::Range<u64>]
0:000>  g
Breakpoint 2 hit
eax=0000162e ebx=00000001 ecx=00000000 edx=00000001 esi=00000000 edi=000004d2
eip=00201395 esp=0135fd18 ebp=0135fd48 iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!msvc_scalarpair_params::option+0x35:
00201395 e8a6000000      call    a!msvc_scalarpair_params::zzz (00201440)
0:000>  dx o1
o1               : Some [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : Some
    [+0x004] __0              : 0x4d2 [Type: unsigned int]
0:000>  dx o2
o2               : Some [Type: enum$<core::option::Option<u64> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u64> >]
    [variant]        : Some
    [+0x008] __0              : 0x162e [Type: unsigned __int64]
0:000>  g
Breakpoint 3 hit
eax=00000028 ebx=0000000a ecx=00000000 edx=0000001e esi=00000000 edi=00000014
eip=002013e5 esp=0135fd18 ebp=0135fd48 iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!msvc_scalarpair_params::tuple+0x35:
002013e5 e856000000      call    a!msvc_scalarpair_params::zzz (00201440)
0:000>  dx t1
t1               : (0xa, 0x14) [Type: tuple$<u32,u32>]
    [<Raw View>]     [Type: tuple$<u32,u32>]
    [0]              : 0xa [Type: unsigned int]
    [1]              : 0x14 [Type: unsigned int]
0:000>  dx t2
t2               : (0x1e, 0x28) [Type: tuple$<u64,u64>]
    [<Raw View>]     [Type: tuple$<u64,u64>]
    [0]              : 0x1e [Type: unsigned __int64]
    [1]              : 0x28 [Type: unsigned __int64]
0:000>  g
Breakpoint 4 hit
eax=00000014 ebx=00000028 ecx=00203110 edx=0000001e esi=0000001e edi=00000000
eip=00201412 esp=0135fd44 ebp=0135fdec iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!msvc_scalarpair_params::str+0x12:
00201412 e829000000      call    a!msvc_scalarpair_params::zzz (00201440)
0:000>  dx s
s                : "this is a static str" [Type: str]
    [<Raw View>]     [Type: str]
    [len]            : 0x14 [Type: unsigned int]
    [chars]         
0:000>  g
Breakpoint 5 hit
eax=00000005 ebx=00000028 ecx=00203124 edx=0000001e esi=0000001e edi=00000000
eip=00201432 esp=0135fd44 ebp=0135fdec iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!msvc_scalarpair_params::slice+0x12:
00201432 e809000000      call    a!msvc_scalarpair_params::zzz (00201440)
0:000>  dx s
s                : { len=0x5 } [Type: slice$<u8>]
    [<Raw View>]     [Type: slice$<u8>]
    [len]            : 0x5 [Type: unsigned int]
    [0]              : 0x1 [Type: unsigned char]
    [1]              : 0x2 [Type: unsigned char]
    [2]              : 0x3 [Type: unsigned char]
    [3]              : 0x4 [Type: unsigned char]
    [4]              : 0x5 [Type: unsigned char]
0:000> 
s                : { len=0x5 } [Type: slice$<u8>]
    [<Raw View>]     [Type: slice$<u8>]
    [len]            : 0x5 [Type: unsigned int]
    [0]              : 0x1 [Type: unsigned char]
    [1]              : 0x2 [Type: unsigned char]
    [2]              : 0x3 [Type: unsigned char]
    [3]              : 0x4 [Type: unsigned char]
    [4]              : 0x5 [Type: unsigned char]
0:000> qq
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
------------------------------------------
stderr:
------------------------------------------

---
test result: FAILED. 111 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out; finished in 19.18s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"



make: *** [Makefile:72: ci-subset-1] Error 1
