plain
failures:

---- [debuginfo-cdb] src/test\debuginfo\rc_arc.rs stdout ----

error: line not found in debugger output:     [Length]         : 3 [Type: unsigned __int64]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\rc_arc.cdb\\rc_arc.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\rc_arc.cdb\\a.exe"
--- stdout -------------------------------
Microsoft (R) Windows Debugger Version 10.0.22000.194 X86
Copyright (c) Microsoft Corporation. All rights reserved.
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc

CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rc_arc.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 01080000 01090000   a.exe   
ModLoad: 77d70000 77f0c000   ntdll.dll
ModLoad: 75d50000 75e30000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 765a0000 7679c000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 722e0000 7237c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 76d90000 76eb3000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 721e0000 721f5000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 69930000 69ed1000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-222683428a92a8b8.dll
ModLoad: 75b40000 75bc0000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 76800000 768c0000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 75cd0000 75d49000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 75c10000 75ccd000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 753b0000 753d0000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 753a0000 753aa000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 76d20000 76d85000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 767a0000 767ff000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 72200000 72225000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 764a0000 764b9000   C:\Windows\SysWOW64\bcrypt.dll
ModLoad: 76310000 7632c000   C:\Windows\SysWOW64\profapi.dll
(e74.904): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=012aa000 ecx=7bb40000 edx=00000000 esi=00026338 edi=77d8383c
eip=77e1ea46 esp=0102f5c0 ebp=0102f5ec iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
77e1ea46 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.x86fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Wed Jun 15 17:41:38.575 2022 (UTC + 0:00)
System Uptime: 0 days 3:28:11.985
Process Uptime: 0 days 0:00:00.107
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.031
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.22000.194 X86
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rc_arc.cdb\rc_arc.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\rc_arc.cdb\a.exe'  Debugger Process 0x1E8C 
dbgeng:  image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.22000.194, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
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
0:000> bp `rc_arc.rs:134`
*** WARNING: Unable to verify checksum for a.exe
Breakpoint 0 hit
Breakpoint 0 hit
eax=00000003 ebx=012aa000 ecx=00043020 edx=00000003 esi=0102faac edi=00045060
eip=01085a9c esp=0102f8c8 ebp=0102f9a4 iopl=0         nv up ei pl nz na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000206
a!rc_arc::main+0x35c:
01085a9c c745f00b000000  mov     dword ptr [ebp-10h],0Bh ss:002b:0102f994=0000000a
0:000> dx rc,d
rc,d             : 111 [Type: alloc::rc::Rc<i32>]
    [<Raw View>]     [Type: alloc::rc::Rc<i32>]
    [Reference count] : 11 [Type: core::cell::Cell<usize>]
    [Weak reference count] : 2 [Type: core::cell::Cell<usize>]
0:000> dx weak_rc,d
weak_rc,d        : 111 [Type: alloc::rc::Weak<i32>]
    [<Raw View>]     [Type: alloc::rc::Weak<i32>]
    [Reference count] : 11 [Type: core::cell::Cell<usize>]
    [Weak reference count] : 2 [Type: core::cell::Cell<usize>]
0:000> dx arc,d
arc,d            : 222 [Type: alloc::sync::Arc<i32>]
    [<Raw View>]     [Type: alloc::sync::Arc<i32>]
    [Reference count] : 21 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
0:000> dx weak_arc,d
weak_arc,d       : 222 [Type: alloc::sync::Weak<i32>]
    [<Raw View>]     [Type: alloc::sync::Weak<i32>]
    [Reference count] : 21 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
0:000> dx dyn_rc,d
dyn_rc,d         [Type: alloc::rc::Rc<dyn$<core::fmt::Debug> >]
    [<Raw View>]     [Type: alloc::rc::Rc<dyn$<core::fmt::Debug> >]
    [Reference count] : 31 [Type: core::cell::Cell<usize>]
    [Weak reference count] : 2 [Type: core::cell::Cell<usize>]
0:000> dx dyn_rc_weak,d
dyn_rc_weak,d    [Type: alloc::rc::Weak<dyn$<core::fmt::Debug> >]
    [<Raw View>]     [Type: alloc::rc::Weak<dyn$<core::fmt::Debug> >]
    [Reference count] : 31 [Type: core::cell::Cell<usize>]
    [Weak reference count] : 2 [Type: core::cell::Cell<usize>]
0:000> dx slice_rc,d
slice_rc,d       : { len=3 } [Type: alloc::rc::Rc<slice$<u32> >]
    [<Raw View>]     [Type: alloc::rc::Rc<slice$<u32> >]
    [Length]         : 3 [Type: unsigned int]
    [Reference count] : 41 [Type: core::cell::Cell<usize>]
    [Weak reference count] : 2 [Type: core::cell::Cell<usize>]
    [0]              : 1 [Type: u32]
    [1]              : 2 [Type: u32]
    [2]              : 3 [Type: u32]
0:000> dx slice_rc_weak,d
slice_rc_weak,d  : { len=3 } [Type: alloc::rc::Weak<slice$<u32> >]
    [<Raw View>]     [Type: alloc::rc::Weak<slice$<u32> >]
    [Length]         : 3 [Type: unsigned int]
    [Reference count] : 41 [Type: core::cell::Cell<usize>]
    [Weak reference count] : 2 [Type: core::cell::Cell<usize>]
    [0]              : 1 [Type: u32]
    [1]              : 2 [Type: u32]
    [2]              : 3 [Type: u32]
0:000> dx dyn_arc,d
dyn_arc,d        [Type: alloc::sync::Arc<dyn$<core::fmt::Debug> >]
    [<Raw View>]     [Type: alloc::sync::Arc<dyn$<core::fmt::Debug> >]
    [Reference count] : 51 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
0:000> dx dyn_arc_weak,d
dyn_arc_weak,d   [Type: alloc::sync::Weak<dyn$<core::fmt::Debug> >]
    [<Raw View>]     [Type: alloc::sync::Weak<dyn$<core::fmt::Debug> >]
    [Reference count] : 51 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
0:000> dx slice_arc,d
slice_arc,d      : { len=3 } [Type: alloc::sync::Arc<slice$<u32> >]
    [<Raw View>]     [Type: alloc::sync::Arc<slice$<u32> >]
    [Length]         : 3 [Type: unsigned int]
    [Reference count] : 61 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
    [0]              : 4 [Type: u32]
    [1]              : 5 [Type: u32]
    [2]              : 6 [Type: u32]
0:000> dx slice_arc_weak,d
slice_arc_weak,d : { len=3 } [Type: alloc::sync::Weak<slice$<u32> >]
    [<Raw View>]     [Type: alloc::sync::Weak<slice$<u32> >]
    [Length]         : 3 [Type: unsigned int]
    [Reference count] : 61 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
    [0]              : 4 [Type: u32]
    [1]              : 5 [Type: u32]
    [2]              : 6 [Type: u32]
0:000> 
slice_arc_weak,d : { len=3 } [Type: alloc::sync::Weak<slice$<u32> >]
    [<Raw View>]     [Type: alloc::sync::Weak<slice$<u32> >]
    [Length]         : 3 [Type: unsigned int]
    [Reference count] : 61 [Type: core::sync::atomic::AtomicUsize]
    [Weak reference count] : 2 [Type: core::sync::atomic::AtomicUsize]
    [0]              : 4 [Type: u32]
    [1]              : 5 [Type: u32]
    [2]              : 6 [Type: u32]
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
    [debuginfo-cdb] src/test\debuginfo\rc_arc.rs

test result: FAILED. 119 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 20.37s

Build completed unsuccessfully in 0:33:33
make: *** [Makefile:70: ci-subset-1] Error 1
