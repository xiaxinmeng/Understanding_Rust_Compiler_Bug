plain
test [codegen] codegen\branch-protection.rs#LEAF ... ok
test [codegen] codegen\async-fn-debug-msvc.rs ... ok
test [codegen] codegen\branch-protection.rs#NONE ... ok
test [codegen] codegen\branch-protection.rs#PACRET ... ok
test [codegen] codegen\async-fn-debug-awaitee-field.rs ... FAILED
test [codegen] codegen\c-variadic-opt.rs ... ok
test [codegen] codegen\call-metadata.rs ... ok
test [codegen] codegen\call-llvm-intrinsics.rs ... ok
test [codegen] codegen\c-variadic.rs ... ok
---
test [codegen] codegen\zip.rs ... ok

failures:

---- [codegen] codegen\async-fn-debug-awaitee-field.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen\\async-fn-debug-awaitee-field\\async-fn-debug-awaitee-field.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\async-fn-debug-awaitee-field.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,MSVC"
stdout: none
--- stderr -------------------------------
D:\a\rust\rust\src/test\codegen\async-fn-debug-awaitee-field.rs:13:11: error: CHECK: expected string not found in input
Some tests failed in compiletest suite=codegen mode=codegen host=i686-pc-windows-msvc target=i686-pc-windows-msvc
// CHECK: [[GEN:!.*]] = !DICompositeType(tag: DW_TAG_structure_type, name: "{async_fn_env#0}",
          ^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-awaitee-field\async-fn-debug-awaitee-field.ll:1:1: note: scanning from here
; ModuleID = 'async_fn_debug_awaitee_field.d7a5d95a-cgu.0'


Input file: D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-awaitee-field\async-fn-debug-awaitee-field.ll
Check file: D:\a\rust\rust\src/test\codegen\async-fn-debug-awaitee-field.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'async_fn_debug_awaitee_field.d7a5d95a-cgu.0' 
check:13     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          2: source_filename = "async_fn_debug_awaitee_field.d7a5d95a-cgu.0" 
check:13     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          3: target datalayout = "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32-a:0:32-S32" 
          4: target triple = "i686-pc-windows-msvc" 
check:13     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          5:  
check:13     ~
check:13     ~
          6: @vtable.0 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i32**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hcab28b5e4841b6f1E" to i8*), [8 x i8] c"\04\00\00\00\04\00\00\00", i8* bitcast (i32 (i32**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h6ed5ef0a7fdc15d8E" to i8*), i8* bitcast (i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2b8743a4caa36b42E" to i8*), i8* bitcast (i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2b8743a4caa36b42E" to i8*) }>, align 4, !dbg !0 
          .
          .
          .
>>>>>>
>>>>>>
------------------------------------------



failures:
    [codegen] codegen\async-fn-debug-awaitee-field.rs
test result: FAILED. 241 passed; 1 failed; 87 ignored; 0 measured; 0 filtered out; finished in 3.53s

Build completed unsuccessfully in 0:25:12
Build completed unsuccessfully in 0:25:12
make: *** [Makefile:72: ci-subset-1] Error 1
