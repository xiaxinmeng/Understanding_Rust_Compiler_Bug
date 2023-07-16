plain
failures:

---- [debuginfo-cdb] debuginfo\msvc-pretty-enums.rs stdout ----

error: line not found in debugger output:         [+0x004] __0              : 0x5c0065 [Type: unsigned int]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\msvc-pretty-enums.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\msvc-pretty-enums.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00c40000 00c48000   a.exe   
ModLoad: 77a00000 77b9c000   ntdll.dll
ModLoad: 75b10000 75bf0000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 753e0000 755db000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 72450000 724ec000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 76390000 764b3000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 72570000 72584000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 70730000 70bbe000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-ed2888a8f1a739a7.dll
ModLoad: 75bf0000 75c4f000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 76b40000 76bfe000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 75050000 75070000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 75040000 7504a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 76c00000 76c65000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 75ed0000 75f49000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 76ac0000 76b3e000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 762d0000 76390000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 72590000 725b5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 75ae0000 75afc000   C:\Windows\SysWOW64\profapi.dll
(16cc.1620): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=0054f000 ecx=680b0000 edx=00000000 esi=00a15ed8 edi=77a1382c
eip=77aaea26 esp=0077f9e0 ebp=0077fa0c iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
77aaea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Tue Jul  6 04:12:35.856 2021 (UTC + 0:00)
System Uptime: 0 days 5:32:26.579
Process Uptime: 0 days 0:00:00.591
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\msvc-pretty-enums.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\msvc-pretty-enums.cdb\a.exe'  Debugger Process 0x1F00 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
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
0:000> bp `msvc-pretty-enums.rs:121`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=00000015 ebx=0054f000 ecx=0077fe00 edx=00000015 esi=0077fe74 edi=00a34040
eip=00c42e5b esp=0077fdc4 ebp=0077fe2c iopl=0         nv up ei pl nz ac pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000216
a!msvc_pretty_enums::main+0x9b:
00c42e5b c745f000000000  mov     dword ptr [ebp-10h],0 ss:002b:0077fe1c=ffffffff
0:000>  dx -r2 a,!
a,!              [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Some]
        [+0x000] __0              : Low (0x2) [Type: msvc_pretty_enums::CStyleEnum]
    [+0x000] discriminant     : 0x2 [Type: enum$<core::option::Option<enum$<msvc_pretty_enums::CStyleEnum> >, 2, 16, Some>::Discriminant$]
0:000>  dx -r2 b,!
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
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
f,!              [Type: enum$<core::option::Option<ref$<u32> >, 1, 4294967295, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<ref$<u32> >, 1, 4294967295, Some>::Some]
        [+0x000] __0              : 0xc442d8 : 0x1 [Type: unsigned int *]
    [+0x000] discriminant     : 0xc442d8 [Type: enum$<core::option::Option<ref$<u32> >, 1, 4294967295, Some>::Discriminant$]
0:000>  dx -r2 g,!
g,!              [Type: enum$<core::option::Option<ref$<u32> >, 1, 4294967295, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<ref$<u32> >, 1, 4294967295, Some>::Some]
        [+0x000] __0              : 0x0 [Type: unsigned int *]
    [+0x000] discriminant     : None (0x0) [Type: enum$<core::option::Option<ref$<u32> >, 1, 4294967295, Some>::Discriminant$]
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
        [+0x004] __0              : 0x77a3d34e [Type: unsigned int]
    [+0x000] discriminant     : None (0x0) [Type: core::option::Option]
0:000>  dx i
i                : None [Type: enum$<core::option::Option<u32> >]
    [<Raw View>]     [Type: enum$<core::option::Option<u32> >]
    [variant]        : None
0:000>  dx j
j                : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
0:000>  dx -r2 k,!
k,!              [Type: enum$<core::option::Option<alloc::string::String>, 1, 4294967295, Some>]
    [+0x000] dataful_variant  [Type: enum$<core::option::Option<alloc::string::String>, 1, 4294967295, Some>::Some]
        [+0x000] __0              [Type: alloc::string::String]
    [+0x000] discriminant     : 0xa317a8 [Type: enum$<core::option::Option<alloc::string::String>, 1, 4294967295, Some>::Discriminant$]
0:000>  dx -r2 l,!
l,!              : $T2 [Type: enum$<core::result::Result<u32, enum$<msvc_pretty_enums::Empty> >, Ok>]
    [+0x000] Ok               [Type: enum$<core::result::Result<u32, enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
        [+0x000] __0              : 0x2a [Type: unsigned int]
0:000> 
l,!              : $T2 [Type: enum$<core::result::Result<u32, enum$<msvc_pretty_enums::Empty> >, Ok>]
    [+0x000] Ok               [Type: enum$<core::result::Result<u32, enum$<msvc_pretty_enums::Empty> >, Ok>::Ok]
        [+0x000] __0              : 0x2a [Type: unsigned int]
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

---
test result: FAILED. 107 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 18.09s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:30:02
Build completed unsuccessfully in 0:30:02
make: *** [Makefile:72: ci-subset-1] Error 1
