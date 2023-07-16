plain
failures:

---- [debuginfo-cdb] debuginfo\generator-objects.rs stdout ----

error: line not found in debugger output:     [+0x008] c                : 6 [Type: int]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.4\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\generator-objects.cdb\\generator-objects.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\generator-objects.cdb\\a.exe"
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
--- stdout -------------------------------
Microsoft (R) Windows Debugger Version 10.0.22000.194 X86
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\generator-objects.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00f50000 00f56000   a.exe   
ModLoad: 77010000 771ac000   ntdll.dll
ModLoad: 752b0000 75390000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 75030000 7522b000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 71a50000 71aec000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 753f0000 75513000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 71b70000 71b85000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 70460000 70a02000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-1577fbcf20d62964.dll
ModLoad: 76cb0000 76d0f000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 76da0000 76e5d000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 74650000 74670000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 74640000 7464a000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 74fc0000 75025000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 768d0000 76949000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 76fe0000 76ff9000   C:\Windows\SysWOW64\bcrypt.dll
ModLoad: 76c20000 76ca0000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 76a50000 76b10000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 71b90000 71bb5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 746c0000 746dc000   C:\Windows\SysWOW64\profapi.dll
(1468.e70): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=01112000 ecx=05ff0000 edx=00000000 esi=01466298 edi=7702383c
eip=770bea46 esp=00e2f5b4 ebp=00e2f5e0 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
770bea46 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.x86fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Tue Mar 15 01:29:55.749 2022 (UTC + 0:00)
System Uptime: 0 days 0:54:13.429
Process Uptime: 0 days 0:00:00.103
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\generator-objects.cdb\generator-objects.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\generator-objects.cdb\a.exe'  Debugger Process 0x1ED4 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.4\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
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
0:000> bp `generator-objects.rs:91`
*** WARNING: Unable to verify checksum for a.exe
0:000> bp `generator-objects.rs:93`
0:000> bp `generator-objects.rs:95`
0:000> bp `generator-objects.rs:97`
0:000>  g
Breakpoint 0 hit
eax=00e2f968 ebx=01112000 ecx=00f511c0 edx=00e2fa6c esi=00e2faa0 edi=01485558
eip=00f511d8 esp=00e2f958 ebp=00e2f9cc iopl=0         nv up ei pl nz ac po nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000212
a!generator_objects::main+0x18:
00f511d8 e8e3030000      call    a!generator_objects::_zzz (00f515c0)
0:000>  dx b
b                : Unresumed [Type: enum$<generator_objects::main::generator_env$0>]
    [<Raw View>]     [Type: enum$<generator_objects::main::generator_env$0>]
    [variant]        : Unresumed
    [+0x000] _ref__a          : 0xe2f968 : 5 [Type: int *]
0:000>  g
Breakpoint 1 hit
eax=00000000 ebx=01112000 ecx=00e2f9b4 edx=00e2f9b4 esi=00e2faa0 edi=01485558
eip=00f51209 esp=00e2f958 ebp=00e2f9cc iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!generator_objects::main+0x49:
00f51209 e8b2030000      call    a!generator_objects::_zzz (00f515c0)
0:000>  dx b
b                : Suspend0 [Type: enum$<generator_objects::main::generator_env$0>]
    [<Raw View>]     [Type: enum$<generator_objects::main::generator_env$0>]
    [variant]        : Suspend0
    [+0x004] c                : 6 [Type: int]
    [+0x008] d                : 7 [Type: int]
    [+0x000] _ref__a          : 0xe2f968 : 5 [Type: int *]
0:000>  g
Breakpoint 2 hit
eax=00000000 ebx=01112000 ecx=00e2f9b4 edx=00000006 esi=00e2faa0 edi=01485558
eip=00f5123a esp=00e2f958 ebp=00e2f9cc iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!generator_objects::main+0x7a:
00f5123a e881030000      call    a!generator_objects::_zzz (00f515c0)
0:000>  dx b
b                : Suspend1 [Type: enum$<generator_objects::main::generator_env$0>]
    [<Raw View>]     [Type: enum$<generator_objects::main::generator_env$0>]
    [variant]        : Suspend1
    [+0x004] c                : 7 [Type: int]
    [+0x008] d                : 8 [Type: int]
    [+0x000] _ref__a          : 0xe2f968 : 6 [Type: int *]
0:000>  g
Breakpoint 3 hit
Breakpoint 3 hit
eax=00000001 ebx=01112000 ecx=00e2f9b4 edx=00000004 esi=00e2faa0 edi=01485558
eip=00f5126b esp=00e2f958 ebp=00e2f9cc iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!generator_objects::main+0xab:
00f5126b e850030000      call    a!generator_objects::_zzz (00f515c0)
0:000>  dx b
b                : Returned [Type: enum$<generator_objects::main::generator_env$0>]
    [<Raw View>]     [Type: enum$<generator_objects::main::generator_env$0>]
    [variant]        : Returned
    [+0x000] _ref__a          : 0xe2f968 : 6 [Type: int *]
0:000> 
b                : Returned [Type: enum$<generator_objects::main::generator_env$0>]
    [<Raw View>]     [Type: enum$<generator_objects::main::generator_env$0>]
    [variant]        : Returned
    [+0x000] _ref__a          : 0xe2f968 : 6 [Type: int *]
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
stderr: none




failures:
    [debuginfo-cdb] debuginfo\generator-objects.rs

test result: FAILED. 115 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 20.76s

Build completed unsuccessfully in 0:34:17
make: *** [Makefile:72: ci-subset-1] Error 1
