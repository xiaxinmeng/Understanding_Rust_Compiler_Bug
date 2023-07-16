plain
failures:

---- [codegen] src/test\codegen\generator-debug-msvc.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.345-1\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\generator-debug-msvc\\generator-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\generator-debug-msvc.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,MSVC"
stdout: none
--- stderr -------------------------------
D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs:30:16: error: CHECK-SAME: expected string not found in input
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
// CHECK-SAME: file: [[FILE]], line: 14,
               ^
D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:172:71: note: scanning from here
!45 = !DIDerivedType(tag: DW_TAG_member, name: "variant1", scope: !33, file: !36, line: 18, baseType: !46, size: 256, align: 64)
                                                                      ^
D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:172:71: note: with "FILE" equal to "!36"
!45 = !DIDerivedType(tag: DW_TAG_member, name: "variant1", scope: !33, file: !36, line: 18, baseType: !46, size: 256, align: 64)
                                                                      ^
D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:172:72: note: possible intended match here
!45 = !DIDerivedType(tag: DW_TAG_member, name: "variant1", scope: !33, file: !36, line: 18, baseType: !46, size: 256, align: 64)


Input file: D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll
Check file: D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
         167: !40 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unresumed", scope: !33, file: !2, size: 256, align: 64, elements: !8, identifier: "6e342e92283ecc2cc10c3ab10cd5e80a") 
         168: !41 = !DIDerivedType(tag: DW_TAG_member, name: "NAME", scope: !37, file: !2, baseType: !32, align: 32, flags: DIFlagStaticMember, extraData: i64 0) 
         169: !42 = !DIDerivedType(tag: DW_TAG_member, name: "DISCR_EXACT", scope: !37, file: !2, baseType: !43, align: 8, flags: DIFlagStaticMember, extraData: i64 0) 
         170: !43 = !DIDerivedType(tag: DW_TAG_typedef, name: "u8", file: !2, baseType: !44) 
         171: !44 = !DIBasicType(name: "unsigned __int8", size: 8, encoding: DW_ATE_unsigned) 
         172: !45 = !DIDerivedType(tag: DW_TAG_member, name: "variant1", scope: !33, file: !36, line: 18, baseType: !46, size: 256, align: 64) 
same:30'0                                                                           X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
same:30'1                                                                                                                                       with "FILE" equal to "!36"
same:30'2                                                                            ?                                                          possible intended match
         173: !46 = !DICompositeType(tag: DW_TAG_structure_type, name: "Variant1", scope: !33, file: !2, size: 256, align: 64, elements: !47, templateParams: !8, identifier: "886e0237a35a8e966b47b97f462ab34") 
same:30'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         174: !47 = !{!48, !50, !51} 
same:30'0     ~~~~~~~~~~~~~~~~~~~~~~~
         175: !48 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !46, file: !2, baseType: !49, size: 256, align: 64) 
same:30'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         176: !49 = !DICompositeType(tag: DW_TAG_structure_type, name: "Returned", scope: !33, file: !2, size: 256, align: 64, elements: !8, identifier: "a433b307d8e4c634abb4085b64429256") 
same:30'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         177: !50 = !DIDerivedType(tag: DW_TAG_member, name: "NAME", scope: !46, file: !2, baseType: !32, align: 32, flags: DIFlagStaticMember, extraData: i64 1) 
same:30'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
---

test result: FAILED. 277 passed; 1 failed; 75 ignored; 0 measured; 0 filtered out; finished in 3.68s

Build completed unsuccessfully in 0:23:11
make: *** [Makefile:73: ci-subset-1] Error 1
