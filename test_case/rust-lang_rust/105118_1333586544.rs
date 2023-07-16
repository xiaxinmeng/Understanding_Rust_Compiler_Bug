plain
failures:

---- [codegen] src/test\codegen\dllimports\main.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";D:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.0\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.0\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.1\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.352-8\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\codegen\\dllimports\\main\\main.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\dllimports\\main.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
Some tests failed in compiletest suite=codegen mode=codegen host=i686-pc-windows-gnu target=i686-pc-windows-gnu
--- stderr -------------------------------
D:\a\rust\rust\src/test\codegen\dllimports\main.rs:17:11: error: CHECK: expected string not found in input
// CHECK: declare dllimport i32 @dylib_func1(i32)
          ^
D:\a\rust\rust\build\i686-pc-windows-gnu\test\codegen\dllimports\main\main.ll:10:57: note: scanning from here
@static_global2 = external local_unnamed_addr global i32
                                                        ^
D:\a\rust\rust\build\i686-pc-windows-gnu\test\codegen\dllimports\main\main.ll:66:11: note: possible intended match here
 %_1 = tail call i32 @dylib_func1(i32 %_2)

Input file: D:\a\rust\rust\build\i686-pc-windows-gnu\test\codegen\dllimports\main\main.ll
Check file: D:\a\rust\rust\src/test\codegen\dllimports\main.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            5:  
            6: @vtable.0 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc33dbc46a78866a3E", [8 x i8] c"\04\00\00\00\04\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he8e4479381a18240E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3e27a21b5afec928E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3e27a21b5afec928E" }>, align 4 
            7: @dylib_global1 = external dllimport local_unnamed_addr global i32 
            8: @dylib_global2 = external dllimport local_unnamed_addr global i32 
            9: @static_global1 = external local_unnamed_addr global i32 
           10: @static_global2 = external local_unnamed_addr global i32 
check:17'0                                                             X error: no match found
           11:  
check:17'0     ~
           12: ; std::sys_common::backtrace::__rust_begin_short_backtrace 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: ; Function Attrs: noinline uwtable 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: define internal fastcc void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hf73f6aacf9a2fb6aE(ptr nocapture noundef nonnull readonly %f) unnamed_addr #0 personality ptr @rust_eh_personality { 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: start: 
check:17'0     ~~~~~~~
            .
            .
           61: ; main::main 
           61: ; main::main 
check:17'0     ~~~~~~~~~~~~~
           62: ; Function Attrs: uwtable 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           63: define internal void @_ZN4main4main17hccda64c235b71048E() unnamed_addr #1 { 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           64: start: 
check:17'0     ~~~~~~~
           65:  %_2 = load i32, ptr @dylib_global1, align 4 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  %_1 = tail call i32 @dylib_func1(i32 %_2) 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:17'1               ?                                 possible intended match
           67:  %_5 = load i32, ptr @dylib_global2, align 4 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  %_4 = tail call i32 @dylib_func2(i32 %_5) 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69:  %_8 = load i32, ptr @static_global1, align 4 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  %_7 = tail call i32 @static_func1(i32 %_8) 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           71:  %_11 = load i32, ptr @static_global2, align 4 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
---

test result: FAILED. 278 passed; 1 failed; 106 ignored; 0 measured; 0 filtered out; finished in 6.09s

Build completed unsuccessfully in 0:31:04
make: *** [Makefile:83: ci-mingw-subset-1] Error 1
