plain
failures:

---- [debuginfo-cdb] tests\debuginfo\pretty-std.rs stdout ----

error: line not found in debugger output: linkedlist       : { len=0x2 } [Type: alloc::collections::linked_list::LinkedList<i32>]
status: exit code: 0
command: PATH="C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;C:\a\rust\rust\ninja;C:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\pretty-std.debugger.script" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std.cdb\\a.exe"
Microsoft (R) Windows Debugger Version 10.0.22000.832 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.


CommandLine: C:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`bab60000 00007ff7`bab72000   a.exe   
ModLoad: 00007ff9`35370000 00007ff9`3555e000   ntdll.dll
ModLoad: 00007ff9`32a80000 00007ff9`32b33000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff9`31660000 00007ff9`318fd000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff9`2f5c0000 00007ff9`2f64c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff9`32450000 00007ff9`3254a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff9`2f960000 00007ff9`2f97b000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`1c790000 00007ff9`1cd00000   C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-7a0122314bcfd5c5.dll
ModLoad: 00007ff9`32fc0000 00007ff9`33067000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff9`325a0000 00007ff9`3263e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff9`35170000 00007ff9`3520f000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff9`33560000 00007ff9`3367d000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff9`336f0000 00007ff9`3375d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff9`31240000 00007ff9`31269000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`32550000 00007ff9`32576000   C:\Windows\System32\bcrypt.dll
ModLoad: 00007ff9`31360000 00007ff9`31383000   C:\Windows\System32\profapi.dll
ModLoad: 00007ff9`30c60000 00007ff9`30c6c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`31900000 00007ff9`31982000   C:\Windows\System32\bcryptPrimitives.dll
(132c.1840): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`3544338c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.amd64fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Mon Apr  3 04:51:52.316 2023 (UTC + 0:00)
System Uptime: 0 days 0:47:01.350
Process Uptime: 0 days 0:00:00.045
  Kernel time: 0 days 0:00:00.031
  User time: 0 days 0:00:00.015
Live user mode: <Local>
Microsoft (R) Windows Debugger Version 10.0.22000.832 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.


command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf C:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\pretty-std.debugger.script C:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std.cdb\a.exe'  Debugger Process 0x1AA0 
dbgeng:  image 10.0.22000.832, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.22000.832, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;C:\a\rust\rust\ninja;C:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.22000.832, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `pretty-std.rs:182`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!pretty_std::main+0x192:
00007ff7`bab65452 e879010000      call    a!pretty_std::zzz (00007ff7`bab655d0)
0:000>  dx slice,d
slice,d          : { len=4 } [Type: ref$<slice2$<i32> >]
    [<Raw View>]     [Type: ref$<slice2$<i32> >]
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
str_slice        : "IAMA string slice!" [Type: ref$<str$>]
    [<Raw View>]     [Type: ref$<str$>]
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
some             : Some [Type: enum2$<core::option::Option<i16> >]
    [<Raw View>]     [Type: enum2$<core::option::Option<i16> >]
    [+0x002] __0              : 8 [Type: short]
0:000>  dx none
none             : None [Type: enum2$<core::option::Option<i64> >]
    [<Raw View>]     [Type: enum2$<core::option::Option<i64> >]
0:000>  dx some_string
some_string      : Some [Type: enum2$<core::option::Option<alloc::string::String> >]
    [<Raw View>]     [Type: enum2$<core::option::Option<alloc::string::String> >]
    [+0x000] __0              : "IAMA optional string!" [Type: alloc::string::String]
0:000>  dx linkedlist
linkedlist       : { len=0x2 } [Type: alloc::collections::linked_list::LinkedList<i32,alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::collections::linked_list::LinkedList<i32,alloc::alloc::Global>]
    [0x0]            : 128 [Type: int]
    [0x1]            : 42 [Type: int]
0:000>  dx vecdeque
vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
    [len]            : 0x2 [Type: unsigned __int64]
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
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\winrt.natvis'
stderr: none




failures:
    [debuginfo-cdb] tests\debuginfo\pretty-std.rs

test result: FAILED. 127 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 19.68s

Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:35:34
make: *** [Makefile:68: ci-subset-1] Error 1
