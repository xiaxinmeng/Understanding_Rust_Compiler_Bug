
2022-08-11T21:28:58.6194839Z failures:
2022-08-11T21:28:58.6195136Z 
2022-08-11T21:28:58.6195668Z ---- [codegen] src/test\codegen\async-fn-debug-awaitee-field.rs stdout ----
2022-08-11T21:28:58.6196251Z 
2022-08-11T21:28:58.6196614Z error: verification with 'FileCheck' failed
2022-08-11T21:28:58.6197234Z status: exit code: 1
2022-08-11T21:28:58.6212473Z command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.342-7\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\async-fn-debug-awaitee-field\\async-fn-debug-awaitee-field.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\async-fn-debug-awaitee-field.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,MSVC"
2022-08-11T21:28:58.6224833Z stdout: none
2022-08-11T21:28:58.6225224Z --- stderr -------------------------------
2022-08-11T21:28:58.6225849Z D:\a\rust\rust\src/test\codegen\async-fn-debug-awaitee-field.rs:15:10: error: MSVC: expected string not found in input
2022-08-11T21:28:58.6226673Z // MSVC: [[GEN:!.*]] = !DICompositeType(tag: DW_TAG_union_type, name: "enum$<async_fn_debug_awaitee_field::async_fn_test::async_fn_env$0>",
2022-08-11T21:28:58.6227178Z          ^
2022-08-11T21:28:58.6227795Z D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\async-fn-debug-awaitee-field\async-fn-debug-awaitee-field.ll:1:1: note: scanning from here
2022-08-11T21:28:58.6228487Z ; ModuleID = 'async_fn_debug_awaitee_field.d7a5d95a-cgu.0'
2022-08-11T21:28:58.6228861Z ^
2022-08-11T21:28:58.6229006Z 
2022-08-11T21:28:58.6229426Z Input file: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\async-fn-debug-awaitee-field\async-fn-debug-awaitee-field.ll
2022-08-11T21:28:58.6230142Z Check file: D:\a\rust\rust\src/test\codegen\async-fn-debug-awaitee-field.rs
2022-08-11T21:28:58.6230490Z 
2022-08-11T21:28:58.6230697Z -dump-input=help explains the following input dump.
2022-08-11T21:28:58.6231519Z Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
2022-08-11T21:28:58.6232103Z 
2022-08-11T21:28:58.6232218Z Input was:
2022-08-11T21:28:58.6232485Z <<<<<<
2022-08-11T21:28:58.6232892Z           1: ; ModuleID = 'async_fn_debug_awaitee_field.d7a5d95a-cgu.0' 
2022-08-11T21:28:58.6233378Z check:15     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
2022-08-11T21:28:58.6233863Z           2: source_filename = "async_fn_debug_awaitee_field.d7a5d95a-cgu.0" 
2022-08-11T21:28:58.6234348Z check:15     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6234830Z           3: target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
2022-08-11T21:28:58.6235718Z check:15     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6236191Z           4: target triple = "x86_64-pc-windows-msvc" 
2022-08-11T21:28:58.6236592Z check:15     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6236906Z           5:  
2022-08-11T21:28:58.6237202Z check:15     ~
2022-08-11T21:28:58.6238538Z           6: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc6fdcc4086fa079eE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h778f9aa03e6928e3E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9df40a73a1323249E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9df40a73a1323249E" to i8*) }>, align 8, !dbg !0 
2022-08-11T21:28:58.6239964Z check:15     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6240452Z           .
2022-08-11T21:28:58.6240707Z           .
2022-08-11T21:28:58.6241071Z           .
2022-08-11T21:28:58.6241324Z >>>>>>
2022-08-11T21:28:58.6241658Z ------------------------------------------
2022-08-11T21:28:58.6241891Z 
2022-08-11T21:28:58.6241897Z 
2022-08-11T21:28:58.6242150Z ---- [codegen] src/test\codegen\async-fn-debug-msvc.rs stdout ----
2022-08-11T21:28:58.6242448Z 
2022-08-11T21:28:58.6242630Z error: verification with 'FileCheck' failed
2022-08-11T21:28:58.6243005Z status: exit code: 1
2022-08-11T21:28:58.6251542Z command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.342-7\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\async-fn-debug-msvc\\async-fn-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\async-fn-debug-msvc.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,MSVC"
2022-08-11T21:28:58.6259400Z stdout: none
2022-08-11T21:28:58.6259746Z --- stderr -------------------------------
2022-08-11T21:28:58.6260404Z D:\a\rust\rust\src/test\codegen\async-fn-debug-msvc.rs:19:15: error: CHECK-DAG: expected string not found in input
2022-08-11T21:28:58.6261202Z // CHECK-DAG: [[GEN:!.*]] = !DICompositeType(tag: DW_TAG_union_type, name: "enum$<async_fn_debug_msvc::async_fn_test::async_fn_env$0>",
2022-08-11T21:28:58.6261649Z               ^
2022-08-11T21:28:58.6262173Z D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll:1:1: note: scanning from here
2022-08-11T21:28:58.6262762Z ; ModuleID = 'async_fn_debug_msvc.c20ed631-cgu.0'
2022-08-11T21:28:58.6263079Z ^
2022-08-11T21:28:58.6263214Z 
2022-08-11T21:28:58.6263549Z Input file: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll
2022-08-11T21:28:58.6264151Z Check file: D:\a\rust\rust\src/test\codegen\async-fn-debug-msvc.rs
2022-08-11T21:28:58.6264422Z 
2022-08-11T21:28:58.6264615Z -dump-input=help explains the following input dump.
2022-08-11T21:28:58.6264858Z 
2022-08-11T21:28:58.6265146Z Input was:
2022-08-11T21:28:58.6265409Z <<<<<<
2022-08-11T21:28:58.6265758Z         1: ; ModuleID = 'async_fn_debug_msvc.c20ed631-cgu.0' 
2022-08-11T21:28:58.6266278Z dag:19     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
2022-08-11T21:28:58.6266751Z         2: source_filename = "async_fn_debug_msvc.c20ed631-cgu.0" 
2022-08-11T21:28:58.6267176Z dag:19     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6267657Z         3: target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
2022-08-11T21:28:58.6268515Z dag:19     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6268963Z         4: target triple = "x86_64-pc-windows-msvc" 
2022-08-11T21:28:58.6269357Z dag:19     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6269688Z         5:  
2022-08-11T21:28:58.6269951Z dag:19     ~
2022-08-11T21:28:58.6272319Z         6: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hfa88a8a6b6c32f22E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h57f02bc96426a5f0E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h402c96db8439e49eE" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h402c96db8439e49eE" to i8*) }>, align 8, !dbg !0 
2022-08-11T21:28:58.6273749Z dag:19     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6274253Z         .
2022-08-11T21:28:58.6274508Z         .
2022-08-11T21:28:58.6274782Z         .
2022-08-11T21:28:58.6275028Z >>>>>>
2022-08-11T21:28:58.6275349Z ------------------------------------------
2022-08-11T21:28:58.6275585Z 
2022-08-11T21:28:58.6275591Z 
2022-08-11T21:28:58.6275841Z ---- [codegen] src/test\codegen\debug-vtable.rs stdout ----
2022-08-11T21:28:58.6276115Z 
2022-08-11T21:28:58.6276293Z error: verification with 'FileCheck' failed
2022-08-11T21:28:58.6276644Z status: exit code: 1
2022-08-11T21:28:58.6285493Z command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.342-7\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\debug-vtable\\debug-vtable.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\debug-vtable.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,MSVC"
2022-08-11T21:28:58.6293502Z stdout: none
2022-08-11T21:28:58.6293848Z --- stderr -------------------------------
2022-08-11T21:28:58.6294423Z D:\a\rust\rust\src/test\codegen\debug-vtable.rs:49:10: error: MSVC: expected string not found in input
2022-08-11T21:28:58.6295280Z // MSVC: !DIGlobalVariable(name: "impl$<debug_vtable::bar::closure_env$0, core::ops::function::FnOnce<tuple$<enum$<core::option::Option<ref$<dyn$<core::ops::function::Fn<tuple$<>,assoc$<Output,tuple$<> > > > > >, {{.*}}, {{.*}}, Some> > > >::vtable$"
2022-08-11T21:28:58.6296011Z          ^
2022-08-11T21:28:58.6296500Z D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\debug-vtable\debug-vtable.ll:1513:127: note: scanning from here
2022-08-11T21:28:58.6297150Z !67 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !63, file: !5, baseType: !18, size: 64, align: 64, offset: 128)
2022-08-11T21:28:58.6297671Z                                                                                                                               ^
2022-08-11T21:28:58.6298235Z D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\debug-vtable\debug-vtable.ll:1515:16: note: possible intended match here
2022-08-11T21:28:58.6299157Z !69 = distinct !DIGlobalVariable(name: "impl$<debug_vtable::bar::closure_env$0, core::ops::function::FnOnce<tuple$<enum2$<core::option::Option<ref$<dyn$<core::ops::function::Fn<tuple$<>,assoc$<Output,tuple$<> > > > > > > > > >::vtable$", scope: null, file: !5, type: !70, isLocal: true, isDefinition: true)
2022-08-11T21:28:58.6299842Z                ^
2022-08-11T21:28:58.6300001Z 
2022-08-11T21:28:58.6300301Z Input file: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\debug-vtable\debug-vtable.ll
2022-08-11T21:28:58.6300846Z Check file: D:\a\rust\rust\src/test\codegen\debug-vtable.rs
2022-08-11T21:28:58.6301179Z 
2022-08-11T21:28:58.6301375Z -dump-input=help explains the following input dump.
2022-08-11T21:28:58.6301628Z 
2022-08-11T21:28:58.6301733Z Input was:
2022-08-11T21:28:58.6302004Z <<<<<<
2022-08-11T21:28:58.6302234Z             .
2022-08-11T21:28:58.6302476Z             .
2022-08-11T21:28:58.6302738Z             .
2022-08-11T21:28:58.6303243Z          1508: !62 = distinct !DIGlobalVariable(name: "impl$<debug_vtable::Foo, _>::vtable$", scope: null, file: !5, type: !63, isLocal: true, isDefinition: true) 
2022-08-11T21:28:58.6304118Z          1509: !63 = !DICompositeType(tag: DW_TAG_structure_type, name: "impl$<debug_vtable::Foo, _>::vtable_type$", file: !5, size: 192, align: 64, flags: DIFlagArtificial, elements: !64, vtableHolder: !52, templateParams: !12, identifier: "ac581b623bdeca801c07f93dc57230ab") 
2022-08-11T21:28:58.6304770Z          1510: !64 = !{!65, !66, !67} 
2022-08-11T21:28:58.6305246Z          1511: !65 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !63, file: !5, baseType: !21, size: 64, align: 64) 
2022-08-11T21:28:58.6305943Z          1512: !66 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !63, file: !5, baseType: !18, size: 64, align: 64, offset: 64) 
2022-08-11T21:28:58.6306575Z          1513: !67 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !63, file: !5, baseType: !18, size: 64, align: 64, offset: 128) 
2022-08-11T21:28:58.6307113Z check:49'0                                                                                                                                   X error: no match found
2022-08-11T21:28:58.6307618Z          1514: !68 = !DIGlobalVariableExpression(var: !69, expr: !DIExpression()) 
2022-08-11T21:28:58.6308075Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6308851Z          1515: !69 = distinct !DIGlobalVariable(name: "impl$<debug_vtable::bar::closure_env$0, core::ops::function::FnOnce<tuple$<enum2$<core::option::Option<ref$<dyn$<core::ops::function::Fn<tuple$<>,assoc$<Output,tuple$<> > > > > > > > > >::vtable$", scope: null, file: !5, type: !70, isLocal: true, isDefinition: true) 
2022-08-11T21:28:58.6309652Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6310231Z check:49'1                    ?                                                                                                                                                                                                                                                                                                    possible intended match
2022-08-11T21:28:58.6311321Z          1516: !70 = !DICompositeType(tag: DW_TAG_structure_type, name: "impl$<debug_vtable::bar::closure_env$0, core::ops::function::FnOnce<tuple$<enum2$<core::option::Option<ref$<dyn$<core::ops::function::Fn<tuple$<>,assoc$<Output,tuple$<> > > > > > > > > >::vtable_type$", file: !5, size: 256, align: 64, flags: DIFlagArtificial, elements: !71, vtableHolder: !76, templateParams: !12, identifier: "2f6d937dc1b34fca4682d9504a455881") 
2022-08-11T21:28:58.6312305Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6313211Z          1517: !71 = !{!72, !73, !74, !75} 
2022-08-11T21:28:58.6313549Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6314033Z          1518: !72 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !70, file: !5, baseType: !21, size: 64, align: 64) 
2022-08-11T21:28:58.6314549Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6315936Z          1519: !73 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !70, file: !5, baseType: !18, size: 64, align: 64, offset: 64) 
2022-08-11T21:28:58.6316508Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6317094Z          1520: !74 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !70, file: !5, baseType: !18, size: 64, align: 64, offset: 128) 
2022-08-11T21:28:58.6317651Z check:49'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6317991Z             .
2022-08-11T21:28:58.6318277Z             .
2022-08-11T21:28:58.6318535Z             .
2022-08-11T21:28:58.6318847Z >>>>>>
2022-08-11T21:28:58.6319158Z ------------------------------------------
2022-08-11T21:28:58.6319413Z 
2022-08-11T21:28:58.6319532Z 
2022-08-11T21:28:58.6319798Z ---- [codegen] src/test\codegen\generator-debug-msvc.rs stdout ----
2022-08-11T21:28:58.6320102Z 
2022-08-11T21:28:58.6320285Z error: verification with 'FileCheck' failed
2022-08-11T21:28:58.6320663Z status: exit code: 1
2022-08-11T21:28:58.6328830Z command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.342-7\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\generator-debug-msvc\\generator-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\generator-debug-msvc.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,MSVC"
2022-08-11T21:28:58.6336773Z stdout: none
2022-08-11T21:28:58.6337106Z --- stderr -------------------------------
2022-08-11T21:28:58.6337658Z D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs:23:15: error: CHECK-DAG: expected string not found in input
2022-08-11T21:28:58.6338390Z // CHECK-DAG: [[GEN:!.*]] = !DICompositeType(tag: DW_TAG_union_type, name: "enum$<generator_debug_msvc::generator_test::generator_env$0>"
2022-08-11T21:28:58.6338837Z               ^
2022-08-11T21:28:58.6339370Z D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:1:1: note: scanning from here
2022-08-11T21:28:58.6339976Z ; ModuleID = 'generator_debug_msvc.724629a4-cgu.0'
2022-08-11T21:28:58.6340291Z ^
2022-08-11T21:28:58.6340425Z 
2022-08-11T21:28:58.6340768Z Input file: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll
2022-08-11T21:28:58.6341547Z Check file: D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs
2022-08-11T21:28:58.6341825Z 
2022-08-11T21:28:58.6342040Z -dump-input=help explains the following input dump.
2022-08-11T21:28:58.6342284Z 
2022-08-11T21:28:58.6342382Z Input was:
2022-08-11T21:28:58.6342634Z <<<<<<
2022-08-11T21:28:58.6342949Z         1: ; ModuleID = 'generator_debug_msvc.724629a4-cgu.0' 
2022-08-11T21:28:58.6343364Z dag:23     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
2022-08-11T21:28:58.6343799Z         2: source_filename = "generator_debug_msvc.724629a4-cgu.0" 
2022-08-11T21:28:58.6344193Z dag:23     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6344627Z         3: target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
2022-08-11T21:28:58.6345077Z dag:23     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6345447Z         4: target triple = "x86_64-pc-windows-msvc" 
2022-08-11T21:28:58.6345804Z dag:23     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6346091Z         5:  
2022-08-11T21:28:58.6346324Z dag:23     ~
2022-08-11T21:28:58.6347418Z         6: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h39f7d92ca7bf99daE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hcaaf3fe3e5294db1E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h202346c304cbf001E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h202346c304cbf001E" to i8*) }>, align 8, !dbg !0 
2022-08-11T21:28:58.6348754Z dag:23     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2022-08-11T21:28:58.6349204Z         .
2022-08-11T21:28:58.6349430Z         .
2022-08-11T21:28:58.6349651Z         .
2022-08-11T21:28:58.6349885Z >>>>>>
2022-08-11T21:28:58.6350165Z ------------------------------------------
2022-08-11T21:28:58.6350374Z 
2022-08-11T21:28:58.6350379Z 
2022-08-11T21:28:58.6350384Z 
2022-08-11T21:28:58.6350479Z failures:
2022-08-11T21:28:58.6350850Z     [codegen] src/test\codegen\async-fn-debug-awaitee-field.rs
2022-08-11T21:28:58.6351434Z     [codegen] src/test\codegen\async-fn-debug-msvc.rs
2022-08-11T21:28:58.6351831Z     [codegen] src/test\codegen\debug-vtable.rs
2022-08-11T21:28:58.6352249Z     [codegen] src/test\codegen\generator-debug-msvc.rs
2022-08-11T21:28:58.6352483Z 
2022-08-11T21:28:58.6352748Z test result: FAILED. 268 passed; 4 failed; 77 ignored; 0 measured; 0 filtered out; finished in 4.50s
