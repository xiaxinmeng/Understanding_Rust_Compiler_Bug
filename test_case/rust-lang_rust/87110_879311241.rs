plain
failures:

---- [debuginfo-cdb] debuginfo\function-names.rs stdout ----

error: line not found in debugger output: [...] a!function_names::impl$6::trait_function<i32, 0x1> (void)
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\function-names.cdb\\function-names.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\function-names.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\function-names.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00f70000 00f76000   a.exe   
ModLoad: 77430000 775cc000   ntdll.dll
ModLoad: 76240000 76320000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 76370000 7656b000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71e80000 71f1c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 75af0000 75c13000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71fa0000 71fb4000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 70160000 705f0000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-ed2888a8f1a739a7.dll
ModLoad: 74c60000 74cbf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 75fa0000 7605e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74a80000 74aa0000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74a70000 74a7a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 74b60000 74bc5000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 76cd0000 76d49000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 76c50000 76cce000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 75ee0000 75fa0000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71fc0000 71fe5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76060000 7607c000   C:\Windows\SysWOW64\profapi.dll
(1ba8.96c): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=00a8b000 ecx=310a0000 edx=00000000 esi=01025ed8 edi=7744382c
eip=774dea26 esp=00cff3f0 ebp=00cff41c iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
774dea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Tue Jul 13 18:34:07.496 2021 (UTC + 0:00)
System Uptime: 0 days 7:55:24.347
Process Uptime: 0 days 0:00:00.594
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\function-names.cdb\function-names.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\function-names.cdb\a.exe'  Debugger Process 0x4B4 
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
0:000> x a!function_names::main
*** WARNING: Unable to verify checksum for a.exe
00f712a0          a!function_names::main (void)
0:000> x a!function_names::generic_func<*
00f71440          a!function_names::generic_func<i32> (int)
0:000> x a!function_names::*::impl_function*
00f71280          a!function_names::Mod1::TestStruct2::impl_function (void)
00f713c0          a!function_names::TestStruct1::impl_function (void)
00f713e0          a!function_names::GenericStruct<i32, i32>::impl_function<i32, i32> (void)
00f71400          a!function_names::impl$2::impl_function::closure$0<i32, i32> (void)
0:000> x a!function_names::*::trait_function*
00f71410          a!function_names::impl$3::trait_function<i32> (void)
00f713d0          a!function_names::impl$1::trait_function (void)
00f71430          a!function_names::impl$6::trait_function<i32, 1> (void)
00f71420          a!function_names::impl$5::trait_function3<function_names::TestStruct1> (void)
00f71290          a!function_names::Mod1::impl$1::trait_function (void)
0:000> x a!function_names::*::closure*
00f71330          a!function_names::main::closure$0 (void)
00f714b0          a!function_names::generic_func::closure$0<i32> (void)
00f71400          a!function_names::impl$2::impl_function::closure$0<i32, i32> (void)
0:000> x a!function_names::*::generator*
00f71340          a!function_names::main::generator$1 (void)
0:000> x a!function_names::const_generic_fn*
00f714c0          a!function_names::const_generic_fn_bool<false> (void)
00f714d0          a!function_names::const_generic_fn_non_int<CONST$fe3cfa0214ac55c7> (void)
00f714f0          a!function_names::const_generic_fn_unsigned_int<14> (void)
00f714e0          a!function_names::const_generic_fn_signed_int<-7> (void)
0:000> 
00f714c0          a!function_names::const_generic_fn_bool<false> (void)
00f714d0          a!function_names::const_generic_fn_non_int<CONST$fe3cfa0214ac55c7> (void)
00f714f0          a!function_names::const_generic_fn_unsigned_int<14> (void)
00f714e0          a!function_names::const_generic_fn_signed_int<-7> (void)
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


---- [debuginfo-cdb] debuginfo\marker-types.rs stdout ----

error: line not found in debugger output:     [len]            : 0x4 [Type: unsigned __int64]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\marker-types.cdb\\marker-types.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\marker-types.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\marker-types.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00c80000 00c88000   a.exe   
ModLoad: 77430000 775cc000   ntdll.dll
ModLoad: 76240000 76320000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 76370000 7656b000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71e80000 71f1c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 75af0000 75c13000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71fa0000 71fb4000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 70160000 705f0000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-ed2888a8f1a739a7.dll
ModLoad: 74c60000 74cbf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 75fa0000 7605e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74a80000 74aa0000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74a70000 74a7a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 74b60000 74bc5000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 76cd0000 76d49000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 76c50000 76cce000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 75ee0000 75fa0000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71fc0000 71fe5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76060000 7607c000   C:\Windows\SysWOW64\profapi.dll
(1528.183c): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=01182000 ecx=91490000 edx=00000000 esi=00d75ed8 edi=7744382c
eip=774dea26 esp=012ff500 ebp=012ff52c iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
774dea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Tue Jul 13 18:34:12.263 2021 (UTC + 0:00)
System Uptime: 0 days 7:55:29.115
Process Uptime: 0 days 0:00:00.596
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
make: *** [Makefile:72: ci-subset-1] Error 1
Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\marker-types.cdb\marker-types.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\marker-types.cdb\a.exe'  Debugger Process 0x1F3C 
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
0:000> bp `marker-types.rs:46`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=012ff8f8 ebx=01182000 ecx=012ff8f8 edx=00000004 esi=012ff8d0 edi=00d94020
eip=00c81ded esp=012ff8d0 ebp=012ff94c iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!marker_types::main+0xbd:
00c81ded e84e000000      call    a!marker_types::zzz (00c81e40)
0:000>  dx nonnull
nonnull          : NonNull(0xc8420c: 0xc) [Type: core::ptr::non_null::NonNull<u32>]
    [<Raw View>]     [Type: core::ptr::non_null::NonNull<u32>]
    0xc [Type: unsigned int]
0:000>  dx manuallydrop
manuallydrop     : 12345 [Type: core::mem::manually_drop::ManuallyDrop<i32>]
    [<Raw View>]     [Type: core::mem::manually_drop::ManuallyDrop<i32>]
0:000>  dx pin
pin              : Pin(0x12ff8ec: "this") [Type: core::pin::Pin<ref_mut$<alloc::string::String> >]
    [<Raw View>]     [Type: core::pin::Pin<ref_mut$<alloc::string::String> >]
    [len]            : 0x4 [Type: unsigned int]
    [capacity]       : 0x4 [Type: unsigned int]
    [chars]          : "this"
0:000>  dx unique
unique           : Unique(0x12ff8f8: (0x2a, 4321)) [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [<Raw View>]     [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [0]              : 0x2a [Type: unsigned __int64]
    [1]              : 4321 [Type: int]
0:000> 
unique           : Unique(0x12ff8f8: (0x2a, 4321)) [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [<Raw View>]     [Type: core::ptr::unique::Unique<tuple$<u64,i32> >]
    [0]              : 0x2a [Type: unsigned __int64]
    [1]              : 4321 [Type: int]
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

error: line not found in debugger output:     [capacity]       : 0x8 [Type: unsigned __int64]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\pretty-std.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00a90000 00a9e000   a.exe   
ModLoad: 77430000 775cc000   ntdll.dll
ModLoad: 76240000 76320000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 76370000 7656b000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71e80000 71f1c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 75af0000 75c13000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71fa0000 71fb4000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 70160000 705f0000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-ed2888a8f1a739a7.dll
ModLoad: 74c60000 74cbf000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 75fa0000 7605e000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74a80000 74aa0000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74a70000 74a7a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 74b60000 74bc5000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 76cd0000 76d49000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 76c50000 76cce000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 75ee0000 75fa0000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71fc0000 71fe5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76060000 7607c000   C:\Windows\SysWOW64\profapi.dll
(a40.e3c): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=01134000 ecx=6dda0000 edx=00000000 esi=01685ed8 edi=7744382c
eip=774dea26 esp=012ff948 ebp=012ff974 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
774dea26 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.x86fre.rs5_release.180914-1434
Machine Name:
Debug session time: Tue Jul 13 18:34:15.274 2021 (UTC + 0:00)
System Uptime: 0 days 7:55:32.126
Process Uptime: 0 days 0:00:00.601
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\pretty-std.cdb\pretty-std.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe'  Debugger Process 0x584 
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
0:000> bp `pretty-std.rs:184`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=012ffd78 ebx=01134000 ecx=0000005a edx=0000005a esi=012ffce0 edi=016a4000
eip=00a939c1 esp=012ffce0 ebp=012ffd94 iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!pretty_std::main+0x201:
00a939c1 e86a010000      call    a!pretty_std::zzz (00a93b30)
0:000>  dx slice,d
slice,d          : { len=4 } [Type: slice$<i32>]
    [<Raw View>]     [Type: slice$<i32>]
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
    [chars]          : "IAMA string!"
0:000>  dx -r2 string
string           : "IAMA string!" [Type: alloc::string::String]
    [<Raw View>]     [Type: alloc::string::String]
    [len]            : 0xc [Type: unsigned int]
    [capacity]       : 0xc [Type: unsigned int]
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
os_string        : "IAMA OS string 😃" [Type: std::ffi::os_str::OsString]
    [<Raw View>]     [Type: std::ffi::os_str::OsString]
    [chars]          : "IAMA OS string 😃"
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
some_string      [Type: enum$<core::option::Option<alloc::string::String>, 1, 4294967295, Some>]
    [<Raw View>]     [Type: enum$<core::option::Option<alloc::string::String>, 1, 4294967295, Some>]
0:000>  dx linkedlist
linkedlist       : { len=0x2 } [Type: alloc::collections::linked_list::LinkedList<i32>]
    [<Raw View>]     [Type: alloc::collections::linked_list::LinkedList<i32>]
    [0x0]            : 128 [Type: int]
    [0x1]            : 42 [Type: int]
0:000>  dx vecdeque
vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32>]
    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32>]
    [len]            : 0x2
    [capacity]       : 0x8 [Type: unsigned int]
    [0x0]            : 90 [Type: int]
    [0x1]            : 20 [Type: int]
0:000> 
vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32>]
    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32>]
    [len]            : 0x2
    [capacity]       : 0x8 [Type: unsigned int]
    [0x0]            : 90 [Type: int]
    [0x1]            : 20 [Type: int]
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
test result: FAILED. 108 passed; 3 failed; 19 ignored; 0 measured; 0 filtered out; finished in 18.97s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:32:25
