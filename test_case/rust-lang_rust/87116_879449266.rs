plain
failures:

---- [debuginfo-cdb] debuginfo\function-names.rs stdout ----

error: line not found in debugger output: [...] a!function_names::impl$6::trait_function<i32, 0x1> (void)
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\function-names.cdb\\function-names.debugger.script" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\function-names.cdb\\a.exe"
------------------------------------------


Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\function-names.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`3ce60000 00007ff7`3ce68000   a.exe   
ModLoad: 00007ffd`4cc30000 00007ffd`4ce1d000   ntdll.dll
ModLoad: 00007ffd`4a350000 00007ffd`4a403000   C:\Windows\System32\KERNEL32.DLL
ModLoad: 00007ffd`48e30000 00007ffd`490c5000   C:\Windows\System32\KERNELBASE.dll
ModLoad: 00007ffd`46dc0000 00007ffd`46e4c000   C:\Windows\SYSTEM32\apphelp.dll
ModLoad: 00007ffd`492a0000 00007ffd`4939a000   C:\Windows\System32\ucrtbase.dll
ModLoad: 00007ffd`3da70000 00007ffd`3da8b000   C:\Windows\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ffd`198a0000 00007ffd`19d48000   D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib\std-0454953abc42d8e2.dll
ModLoad: 00007ffd`4c000000 00007ffd`4c06d000   C:\Windows\System32\WS2_32.dll
ModLoad: 00007ffd`4bee0000 00007ffd`4bfff000   C:\Windows\System32\RPCRT4.dll
ModLoad: 00007ffd`4c490000 00007ffd`4c534000   C:\Windows\System32\ADVAPI32.dll
ModLoad: 00007ffd`4c6e0000 00007ffd`4c77e000   C:\Windows\System32\msvcrt.dll
ModLoad: 00007ffd`4c280000 00007ffd`4c31e000   C:\Windows\System32\sechost.dll
ModLoad: 00007ffd`48b40000 00007ffd`48b69000   C:\Windows\SYSTEM32\USERENV.dll
ModLoad: 00007ffd`48c10000 00007ffd`48c34000   C:\Windows\System32\profapi.dll
ModLoad: 00007ffd`485c0000 00007ffd`485cc000   C:\Windows\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ffd`49680000 00007ffd`49702000   C:\Windows\System32\bcryptPrimitives.dll
(e74.5a4): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ffd`4cd0260c cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x64
Product: Server, suite: TerminalServer DataCenter SingleUserTS
17763.1.amd64fre.rs5_release.180914-1434
Machine Name:
Debug session time: Tue Jul 13 22:31:32.241 2021 (UTC + 0:00)
System Uptime: 0 days 3:55:48.828
Process Uptime: 0 days 0:00:00.568
  Kernel time: 0 days 0:00:00.015
  User time: 0 days 0:00:00.000
Live user mode: <Local>

Microsoft (R) Windows Debugger Version 10.0.19041.685 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe" -lines -cf D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\function-names.cdb\function-names.debugger.script D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\debuginfo\function-names.cdb\a.exe'  Debugger Process 0x1A44 
dbgeng:  image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbgeng.dll]
dbghelp: image 10.0.19041.685, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\dbghelp.dll]
        DIA version: 27412
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions;C:\Program Files (x86)\Windows Kits\10\Debuggers\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
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
0:000> x a!function_names::main
*** WARNING: Unable to verify checksum for a.exe
00007ff7`3ce61230 a!function_names::main (void)
0:000> x a!function_names::generic_func<*
00007ff7`3ce613f0 a!function_names::generic_func<i32> (int)
0:000> x a!function_names::*::impl_function*
00007ff7`3ce61210 a!function_names::Mod1::TestStruct2::impl_function (void)
00007ff7`3ce61370 a!function_names::TestStruct1::impl_function (void)
00007ff7`3ce61390 a!function_names::GenericStruct<i32, i32>::impl_function<i32, i32> (void)
00007ff7`3ce613b0 a!function_names::impl$2::impl_function::closure$0<i32, i32> (void)
0:000> x a!function_names::*::trait_function*
00007ff7`3ce613c0 a!function_names::impl$3::trait_function<i32> (void)
00007ff7`3ce61380 a!function_names::impl$1::trait_function (void)
00007ff7`3ce613e0 a!function_names::impl$6::trait_function<i32, 1> (void)
00007ff7`3ce613d0 a!function_names::impl$5::trait_function3<function_names::TestStruct1> (void)
00007ff7`3ce61220 a!function_names::Mod1::impl$1::trait_function (void)
0:000> x a!function_names::*::closure*
00007ff7`3ce612d0 a!function_names::main::closure$0 (void)
00007ff7`3ce61440 a!function_names::generic_func::closure$0<i32> (void)
00007ff7`3ce613b0 a!function_names::impl$2::impl_function::closure$0<i32, i32> (void)
0:000> x a!function_names::*::generator*
00007ff7`3ce612e0 a!function_names::main::generator$1 (void)
0:000> x a!function_names::const_generic_fn*
00007ff7`3ce61450 a!function_names::const_generic_fn_bool<false> (void)
00007ff7`3ce61460 a!function_names::const_generic_fn_non_int<CONST$fe3cfa0214ac55c7> (void)
00007ff7`3ce61480 a!function_names::const_generic_fn_unsigned_int<14> (void)
00007ff7`3ce61470 a!function_names::const_generic_fn_signed_int<-7> (void)
0:000> 
00007ff7`3ce61450 a!function_names::const_generic_fn_bool<false> (void)
00007ff7`3ce61460 a!function_names::const_generic_fn_non_int<CONST$fe3cfa0214ac55c7> (void)
00007ff7`3ce61480 a!function_names::const_generic_fn_unsigned_int<14> (void)
00007ff7`3ce61470 a!function_names::const_generic_fn_signed_int<-7> (void)
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
test result: FAILED. 107 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 19.25s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\debuginfo" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"



make: *** [Makefile:72: ci-subset-1] Error 1
