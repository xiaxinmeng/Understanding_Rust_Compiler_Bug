
Microsoft (R) Windows Debugger Version 10.0.14321.1024 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\a.exe
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`ed610000 00007ff7`ed627000   a.exe   
ModLoad: 00007ffb`1d610000 00007ffb`1d7bd000   ntdll.dll
ModLoad: 00007ffb`1c5f0000 00007ffb`1c72f000   C:\Windows\system32\KERNEL32.DLL
ModLoad: 00007ffb`1aaa0000 00007ffb`1abb5000   C:\Windows\system32\KERNELBASE.dll
ModLoad: 00007ffb`19310000 00007ffb`1939e000   C:\Windows\system32\apphelp.dll
SHIMVIEW: ShimInfo(Complete)
ModLoad: 00007ffa`e0720000 00007ffa`e0b02000   C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-3e140cd10516e859.dll
ModLoad: 00007ffb`0f780000 00007ffb`0f799000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffb`0f770000 00007ffb`0f774000   C:\Windows\SYSTEM32\api-ms-win-crt-runtime-l1-1-0.dll
ModLoad: 00007ffb`0f6c0000 00007ffb`0f6c5000   C:\Windows\SYSTEM32\api-ms-win-crt-math-l1-1-0.dll
ModLoad: 00007ffb`0f730000 00007ffb`0f734000   C:\Windows\SYSTEM32\api-ms-win-crt-stdio-l1-1-0.dll
ModLoad: 00007ffb`0f6f0000 00007ffb`0f6f3000   C:\Windows\SYSTEM32\api-ms-win-crt-locale-l1-1-0.dll
ModLoad: 00007ffb`0f720000 00007ffb`0f723000   C:\Windows\SYSTEM32\api-ms-win-crt-heap-l1-1-0.dll
ModLoad: 00007ffb`1c8e0000 00007ffb`1c98a000   C:\Windows\system32\ADVAPI32.dll
ModLoad: 00007ffb`1c4c0000 00007ffb`1c51a000   C:\Windows\system32\WS2_32.dll
ModLoad: 00007ffb`19d90000 00007ffb`19db1000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffb`0f760000 00007ffb`0f764000   C:\Windows\SYSTEM32\api-ms-win-crt-string-l1-1-0.dll
ModLoad: 00007ffb`0f740000 00007ffb`0f744000   C:\Windows\SYSTEM32\api-ms-win-crt-convert-l1-1-0.dll
ModLoad: 00007ffb`1c520000 00007ffb`1c5ca000   C:\Windows\system32\msvcrt.dll
ModLoad: 00007ffb`1d4e0000 00007ffb`1d538000   C:\Windows\SYSTEM32\sechost.dll
ModLoad: 00007ffb`1c7a0000 00007ffb`1c8e0000   C:\Windows\system32\RPCRT4.dll
ModLoad: 00007ffb`1ad00000 00007ffb`1ad09000   C:\Windows\system32\NSI.dll
ModLoad: 00007ffb`1a740000 00007ffb`1a755000   C:\Windows\SYSTEM32\profapi.dll
ModLoad: 00007ffb`0f5c0000 00007ffb`0f6b4000   C:\Windows\SYSTEM32\ucrtbase.DLL
ModLoad: 00007ffb`1ac10000 00007ffb`1ac3e000   C:\Windows\system32\SspiCli.dll
ModLoad: 00007ffb`1a680000 00007ffb`1a68b000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ffb`1a610000 00007ffb`1a673000   C:\Windows\SYSTEM32\bcryptPrimitives.dll
(1478.25f4): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffb`1d6d1c20 cc              int     3
0:000> version
Windows 8.1 Version 9600 MP (8 procs) Free x64
Product: WinNt, suite: SingleUserTS
kernelbase.dll version: 6.3.9600.19724 (winblue_ltsb_escrow.200519-1914)
Machine Name:
Debug session time: Sat Sep  5 12:51:02.474 2020 (UTC + 3:00)
System Uptime: 7 days 1:57:25.609
Process Uptime: 0 days 0:00:00.015
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.14321.1024 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\pretty-std-collections-hash.debugger.script C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\test\debuginfo\pretty-std-collections-hash.cdb\a.exe'  Debugger Process 0x2C0C 
dbgeng:  image 10.0.14321.1024, built Thu Jul 28 08:25:22 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.14321.1024, built Thu Jul 28 08:23:41 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 40116
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;C:\msys64\home\we\rust\build\x86_64-pc-windows-msvc\stage0\bin;C:\Users\we\.cargo\bin;C:\Python38;C:\Program Files\CMake\bin;C:\Program Files\Git\cmd;C:\ProgramData\chocolatey\bin;C:\msys64\mingw64\bin;C:\msys64\usr\local\bin;C:\msys64\usr\bin;C:\msys64\usr\bin;C:\Windows\System32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\
Extension DLL chain:
    dbghelp: image 10.0.14321.1024, API 10.0.6, built Thu Jul 28 08:23:41 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.14321.1024, API 1.0.0, built Thu Jul 28 08:22:16 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.14321.1024, API 1.0.0, built Thu Jul 28 08:22:41 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.14321.1024, API 1.0.0, built Thu Jul 28 08:22:26 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.14393.33, API 1.0.0, built Thu Jul 28 08:32:24 2016
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\atlmfc.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\concurrency.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\cpp_rest.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Kernel.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\stl.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Data.Json.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Geolocation.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Sensors.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Media.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\windows.natvis
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\winrt.natvis
0:000> bp `pretty-std-collections-hash.rs:94`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!pretty_std_collections_hash::main+0xf9:
00007ff7`ed61d6d9 e8b2000000      call    a!pretty_std_collections_hash::zzz (00007ff7`ed61d790)
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_set,d
hash_set,d       : { size=15 } [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::set::HashSet<u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : 3 [Type: u64]
    [1]              : 5 [Type: u64]
    [2]              : 2 [Type: u64]
    [3]              : 8 [Type: u64]
    [4]              : 7 [Type: u64]
    [5]              : 10 [Type: u64]
    [6]              : 12 [Type: u64]
    [7]              : 4 [Type: u64]
    [8]              : 14 [Type: u64]
    [9]              : 0 [Type: u64]
    [10]             : 6 [Type: u64]
    [11]             : 11 [Type: u64]
    [12]             : 13 [Type: u64]
    [13]             : 1 [Type: u64]
    [14]             : 9 [Type: u64]
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000>  dx hash_map,d
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000> 
hash_map,d       : { size=15 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 15 [Type: unsigned __int64]
    [capacity]       : 28
    [state]          [Type: std::collections::hash::map::RandomState]
    [0]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [1]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [2]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [3]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [4]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [5]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [6]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [7]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [8]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [9]              : Unable to find type 'tuple<u64,u64> *' for cast.
    [10]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [11]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [12]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [13]             : Unable to find type 'tuple<u64,u64> *' for cast.
    [14]             : Unable to find type 'tuple<u64,u64> *' for cast.
0:000> qq
quit:
