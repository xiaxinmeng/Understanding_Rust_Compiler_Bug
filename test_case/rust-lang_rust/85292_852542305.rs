plain
failures:

---- [codegen] codegen\async-fn-debug-msvc.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen\\async-fn-debug-msvc\\async-fn-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\async-fn-debug-msvc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
D:\a\rust\rust\src/test\codegen\async-fn-debug-msvc.rs:20:15: error: CHECK-DAG: expected string not found in input
// CHECK-DAG: [[GEN:!.*]] = !DICompositeType(tag: DW_TAG_union_type, name: "generator-0", scope: [[ASYNC_FN]]
              ^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll:1:1: note: scanning from here
; ModuleID = 'async_fn_debug_msvc.e938393b84a64d94-cgu.0'
^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll:1:1: note: with "ASYNC_FN" equal to "!22"
; ModuleID = 'async_fn_debug_msvc.e938393b84a64d94-cgu.0'


Input file: D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll
Check file: D:\a\rust\rust\src/test\codegen\async-fn-debug-msvc.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'async_fn_debug_msvc.e938393b84a64d94-cgu.0'
dag:20'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
dag:20'1                                                               with "ASYNC_FN" equal to "!22"
          2: source_filename = "async_fn_debug_msvc.e938393b84a64d94-cgu.0"
dag:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          3: target datalayout = "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:32-n8:16:32-a:0:32-S32"
dag:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          4: target triple = "i686-pc-windows-msvc"
dag:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          5: 
dag:20'0     ~
          6: @vtable.0 = private unnamed_addr constant { void (i32**)*, i32, i32, i32 (i32**)*, i32 (i32**)*, i32 (i32**)* } { void (i32**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he9e43874381c0f9fE", i32 4, i32 4, i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h50494d4187c1bcb3E", i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h50494d4187c1bcb3E", i32 (i32**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h92b9b6af7f406337E" }, align 4, !dbg !0
dag:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          .
          .
>>>>>>


------------------------------------------


---- [codegen] codegen\generator-debug-msvc.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen\\generator-debug-msvc\\generator-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\generator-debug-msvc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs:24:15: error: CHECK-DAG: expected string not found in input
// CHECK-DAG: [[GEN:!.*]] = !DICompositeType(tag: DW_TAG_union_type, name: "generator-0", scope: [[GEN_FN]]
              ^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:1:1: note: scanning from here
; ModuleID = 'generator_debug_msvc.1154da68f92272b2-cgu.0'
^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:1:1: note: with "GEN_FN" equal to "!22"
; ModuleID = 'generator_debug_msvc.1154da68f92272b2-cgu.0'


Input file: D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll
Check file: D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'generator_debug_msvc.1154da68f92272b2-cgu.0'
dag:24'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
dag:24'1                                                                with "GEN_FN" equal to "!22"
          2: source_filename = "generator_debug_msvc.1154da68f92272b2-cgu.0"
dag:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          3: target datalayout = "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:32-n8:16:32-a:0:32-S32"
dag:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          4: target triple = "i686-pc-windows-msvc"
dag:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          5: 
dag:24'0     ~
          6: @vtable.0 = private unnamed_addr constant { void (i32**)*, i32, i32, i32 (i32**)*, i32 (i32**)*, i32 (i32**)* } { void (i32**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0532a2b21a876298E", i32 4, i32 4, i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2b5012fdf6f39b6cE", i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2b5012fdf6f39b6cE", i32 (i32**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h671f452f6e516610E" }, align 4, !dbg !0
dag:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          .
          .
>>>>>>

---
failures:
    [codegen] codegen\async-fn-debug-msvc.rs
    [codegen] codegen\generator-debug-msvc.rs

Some tests failed in compiletest suite=codegen mode=codegen host=i686-pc-windows-msvc target=i686-pc-windows-msvc




command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\codegen" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "codegen" "--mode" "codegen" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:24:13
Build completed unsuccessfully in 0:24:13
make: *** [Makefile:72: ci-subset-1] Error 1
