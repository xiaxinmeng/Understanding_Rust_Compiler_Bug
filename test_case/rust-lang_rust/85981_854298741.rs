plain
failures:

---- [debuginfo-cdb] debuginfo\result-types.rs stdout ----

error: line not found in debugger output: x,d              : Ok(-3) [Type: core::result::Result<i32, str>]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\result-types.cdb\\result-types.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\result-types.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\result-types.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff6`487c0000 00007ff6`487c7000   a.exe   
ModLoad: 00007ff8`74b60000 00007ff8`74d4d000   ntdll.dll
ModLoad: 00007ff8`71d90000 00007ff8`71e43000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ff8`70c20000 00007ff8`70eb5000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ff8`66400000 00007ff8`6648c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ff8`70f90000 00007ff8`7108a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ff8`67b40000 00007ff8`67b5b000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff8`437d0000 00007ff8`43bc3000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-f47ffd3f623fb223.dll
ModLoad: 00007ff8`749e0000 00007ff8`74a4d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ff8`728d0000 00007ff8`729ef000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ff8`724a0000 00007ff8`72543000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ff8`733e0000 00007ff8`7347e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ff8`73310000 00007ff8`733ae000   C:\Windows\System32\sechost.dll
ModLoad: 00007ff8`70a80000 00007ff8`70aa9000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ff8`70bf0000 00007ff8`70c14000   C:\Windows\System32\profapi.dll
ModLoad: 00007ff8`70500000 00007ff8`7050c000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff8`71d10000 00007ff8`71d8f000   C:\Windows\System32\bcryptPrimitives.dll
(c24.9f8): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff8`74c3260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Fri Jun  4 01:55:45.682 2021 (UTC + 0:00)
System Uptime: 0 days 4:45:34.996
Process Uptime: 0 days 0:00:00.558
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\result-types.cdb\result-types.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\result-types.cdb\a.exe'  Debugger Process 0xA90 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.19041.685, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
    ext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\ext.dll]
    exts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\exts.dll]
    uext: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\uext.dll]
    ntsdexts: image 10.0.19041.685, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP\ntsdexts.dll]
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> bp `result-types.rs:23`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
a!result_types::main+0x130:
00007ff6`487c1340 e84b000000      call    a!result_types::zzz (00007ff6`487c1390)
0:000>  dx x,d
x,d              : Ok [Type: enum$<core::result::Result<i32, str>>]
    [<Raw View>]     [Type: enum$<core::result::Result<i32, str>>]
    [+0x000] variant$         : Ok (0) [Type: core::result::Result]
    [+0x004] __0              : -3 [Type: int]
0:000>  dx y
y                : Err [Type: enum$<core::result::Result<i32, str>>]
    [<Raw View>]     [Type: enum$<core::result::Result<i32, str>>]
    [+0x000] variant$         : Err (0x1) [Type: core::result::Result]
    [+0x008] __0              : "Some error message" [Type: str]
0:000> 
y                : Err [Type: enum$<core::result::Result<i32, str>>]
    [<Raw View>]     [Type: enum$<core::result::Result<i32, str>>]
    [+0x000] variant$         : Err (0x1) [Type: core::result::Result]
    [+0x008] __0              : "Some error message" [Type: str]
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\Visualizers\winrt.natvis'
------------------------------------------
stderr:
------------------------------------------

---
test result: FAILED. 105 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 15.98s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:25:34
Build completed unsuccessfully in 0:25:34
make: *** [Makefile:72: ci-subset-1] Error 1
