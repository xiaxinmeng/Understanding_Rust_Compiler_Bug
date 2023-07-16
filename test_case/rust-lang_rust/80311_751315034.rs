plain
failures:

---- [debuginfo-cdb] debuginfo\pretty-std-collections-hash.rs stdout ----

error: line not found in debugger output: hash_set,d [...] : { size=15 } [Type: [...]::HashSet<u64, [...]>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.28.29333\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.28.29333\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.5.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.2.2\tools\ghc-8.10.2\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.0.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.14.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\Java\jdk8u275-b01\bin;C:\npm\prefix;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std-collections-hash.cdb\\pretty-std-collections-hash.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\pretty-std-collections-hash.cdb\\a.exe"
------------------------------------------

Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Microsoft (R) Windows Debugger Version 10.0.19041.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`c0470000 00007ff7`c048b000   a.exe   
ModLoad: 00007ffa`77a10000 00007ffa`77bfd000   ntdll.dll
ModLoad: 00007ffa`76c40000 00007ffa`76cf3000   C:\windows\System32\KERNEL32.DLL
ModLoad: 00007ffa`73de0000 00007ffa`74075000   C:\windows\System32\KERNELBASE.dll
ModLoad: 00007ffa`6a540000 00007ffa`6a5cc000   C:\windows\SYSTEM32\apphelp.dll
ModLoad: 00007ffa`74380000 00007ffa`7447a000   C:\windows\System32\ucrtbase.dll
ModLoad: 00007ffa`68c10000 00007ffa`68c29000   C:\windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffa`43fb0000 00007ffa`443d2000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-3b10fdbd5cd1d022.dll
ModLoad: 00007ffa`74ec0000 00007ffa`74f63000   C:\windows\System32\ADVAPI32.dll
ModLoad: 00007ffa`76d00000 00007ffa`76d9e000   C:\windows\System32\msvcrt.dll
ModLoad: 00007ffa`77620000 00007ffa`776bf000   C:\windows\System32\sechost.dll
ModLoad: 00007ffa`77740000 00007ffa`77862000   C:\windows\System32\RPCRT4.dll
ModLoad: 00007ffa`76970000 00007ffa`769dd000   C:\windows\System32\WS2_32.dll
ModLoad: 00007ffa`73920000 00007ffa`73949000   C:\windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffa`73a70000 00007ffa`73a94000   C:\windows\System32\profapi.dll
ModLoad: 00007ffa`733d0000 00007ffa`733dc000   C:\windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ffa`74080000 00007ffa`740ff000   C:\windows\System32\bcryptPrimitives.dll
(1c4c.698): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffa`77ae250c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Sat Dec 26 03:48:13.132 2020 (UTC + 0:00)
System Uptime: 0 days 5:00:27.406
Process Uptime: 0 days 0:00:00.564
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.1 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\pretty-std-collections-hash.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\a.exe'  Debugger Process 0x830 
dbgeng:  image 10.0.19041.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.19041.1, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.28.29333\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.28.29333\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.5.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.2.2\tools\ghc-8.10.2\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.0.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.14.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\Java\jdk8u275-b01\bin;C:\npm\prefix;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.19041.1, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.19041.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.19041.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.19041.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.19041.1, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `pretty-std-collections-hash.rs:98`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!pretty_std_collections_hash::main+0xff:
00007ff7`c047a25f e8ac000000      call    a!pretty_std_collections_hash::zzz (00007ff7`c047a310)
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { len=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 8 [Type: u64]
    [1]              : 9 [Type: u64]
    [2]              : 14 [Type: u64]
    [3]              : 7 [Type: u64]
    [4]              : 3 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 6 [Type: u64]
    [7]              : 2 [Type: u64]
    [8]              : 1 [Type: u64]
    [9]              : 4 [Type: u64]
    [10]             : 0 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 12 [Type: u64]
    [13]             : 5 [Type: u64]
    [14]             : 13 [Type: u64]
0:000>  dx hash_map,d
hash_map,d       : { len=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    ["0x3"]          : 3 [Type: unsigned __int64]
    ["0xa"]          : 10 [Type: unsigned __int64]
    ["0x0"]          : 0 [Type: unsigned __int64]
    ["0x2"]          : 2 [Type: unsigned __int64]
    ["0xd"]          : 13 [Type: unsigned __int64]
    ["0x7"]          : 7 [Type: unsigned __int64]
    ["0xe"]          : 14 [Type: unsigned __int64]
    ["0x6"]          : 6 [Type: unsigned __int64]
    ["0x5"]          : 5 [Type: unsigned __int64]
    ["0x1"]          : 1 [Type: unsigned __int64]
    ["0x4"]          : 4 [Type: unsigned __int64]
    ["0x8"]          : 8 [Type: unsigned __int64]
    ["0xb"]          : 11 [Type: unsigned __int64]
    ["0xc"]          : 12 [Type: unsigned __int64]
    ["0x9"]          : 9 [Type: unsigned __int64]
0:000>  dx hash_map,d
hash_map,d       : { len=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    ["0x3"]          : 3 [Type: unsigned __int64]
    ["0xa"]          : 10 [Type: unsigned __int64]
    ["0x0"]          : 0 [Type: unsigned __int64]
    ["0x2"]          : 2 [Type: unsigned __int64]
    ["0xd"]          : 13 [Type: unsigned __int64]
    ["0x7"]          : 7 [Type: unsigned __int64]
    ["0xe"]          : 14 [Type: unsigned __int64]
    ["0x6"]          : 6 [Type: unsigned __int64]
    ["0x5"]          : 5 [Type: unsigned __int64]
    ["0x1"]          : 1 [Type: unsigned __int64]
    ["0x4"]          : 4 [Type: unsigned __int64]
    ["0x8"]          : 8 [Type: unsigned __int64]
    ["0xb"]          : 11 [Type: unsigned __int64]
    ["0xc"]          : 12 [Type: unsigned __int64]
    ["0x9"]          : 9 [Type: unsigned __int64]
0:000>  dx hash_map,d
hash_map,d       : { len=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    ["0x3"]          : 3 [Type: unsigned __int64]
    ["0xa"]          : 10 [Type: unsigned __int64]
    ["0x0"]          : 0 [Type: unsigned __int64]
    ["0x2"]          : 2 [Type: unsigned __int64]
    ["0xd"]          : 13 [Type: unsigned __int64]
    ["0x7"]          : 7 [Type: unsigned __int64]
    ["0xe"]          : 14 [Type: unsigned __int64]
    ["0x6"]          : 6 [Type: unsigned __int64]
    ["0x5"]          : 5 [Type: unsigned __int64]
    ["0x1"]          : 1 [Type: unsigned __int64]
    ["0x4"]          : 4 [Type: unsigned __int64]
    ["0x8"]          : 8 [Type: unsigned __int64]
    ["0xb"]          : 11 [Type: unsigned __int64]
    ["0xc"]          : 12 [Type: unsigned __int64]
    ["0x9"]          : 9 [Type: unsigned __int64]
0:000>  dx hash_map,d
hash_map,d       : { len=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    ["0x3"]          : 3 [Type: unsigned __int64]
    ["0xa"]          : 10 [Type: unsigned __int64]
    ["0x0"]          : 0 [Type: unsigned __int64]
    ["0x2"]          : 2 [Type: unsigned __int64]
    ["0xd"]          : 13 [Type: unsigned __int64]
    ["0x7"]          : 7 [Type: unsigned __int64]
    ["0xe"]          : 14 [Type: unsigned __int64]
    ["0x6"]          : 6 [Type: unsigned __int64]
    ["0x5"]          : 5 [Type: unsigned __int64]
    ["0x1"]          : 1 [Type: unsigned __int64]
    ["0x4"]          : 4 [Type: unsigned __int64]
    ["0x8"]          : 8 [Type: unsigned __int64]
    ["0xb"]          : 11 [Type: unsigned __int64]
    ["0xc"]          : 12 [Type: unsigned __int64]
    ["0x9"]          : 9 [Type: unsigned __int64]
0:000>  dx hash_map,d
hash_map,d       : { len=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [len]            : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    ["0x3"]          : 3 [Type: unsigned __int64]
    ["0xa"]          : 10 [Type: unsigned __int64]
    ["0x0"]          : 0 [Type: unsigned __int64]
    ["0x2"]          : 2 [Type: unsigned __int64]
    ["0xd"]          : 13 [Type: unsigned __int64]
    ["0x7"]          : 7 [Type: unsigned __int64]
    ["0xe"]          : 14 [Type: unsigned __int64]
    ["0x6"]          : 6 [Type: unsigned __int64]
    ["0x5"]          : 5 [Type: unsigned __int64]
    ["0x1"]          : 1 [Type: unsigned __int64]
    ["0x4"]          : 4 [Type: unsigned __int64]
---
test result: FAILED. 95 passed; 2 failed; 19 ignored; 0 measured; 0 filtered out; finished in 14.18s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.1\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.1\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "11.0.0-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"



make: *** [Makefile:73: ci-subset-1] Error 1
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
