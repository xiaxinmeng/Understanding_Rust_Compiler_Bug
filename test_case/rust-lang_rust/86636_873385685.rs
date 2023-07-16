plain
failures:

---- [codegen] codegen\async-fn-debug-msvc.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen\\async-fn-debug-msvc\\async-fn-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\async-fn-debug-msvc.rs" "--check-prefixes" "CHECK,MSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
D:\a\rust\rust\src/test\codegen\async-fn-debug-msvc.rs:46:11: error: CHECK: expected string not found in input
Some tests failed in compiletest suite=codegen mode=codegen host=i686-pc-windows-msvc target=i686-pc-windows-msvc
// CHECK: {{!.*}} = !DIDerivedType(tag: DW_TAG_member, name: "variant$", scope: [[S1]],
          ^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll:357:1: note: scanning from here
!222 = !{!223, !251}
^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll:357:1: note: with "S1" equal to "!221"
!222 = !{!223, !251}
^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll:375:1: note: possible intended match here
!240 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !237, file: !2, baseType: !58, size: 32, align: 32)


Input file: D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\async-fn-debug-msvc\async-fn-debug-msvc.ll
Check file: D:\a\rust\rust\src/test\codegen\async-fn-debug-msvc.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          352: !217 = !DIDerivedType(tag: DW_TAG_member, name: "discriminant", scope: !209, file: !2, baseType: !32, size: 8, align: 8)
          353: !218 = !{!219}
          354: !219 = !DITemplateTypeParameter(name: "T", type: !209)
          355: !220 = !DIDerivedType(tag: DW_TAG_member, name: "variant4", scope: !194, file: !185, line: 14, baseType: !221, size: 128, align: 32, extraData: i64 4)
          356: !221 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend1", scope: !194, file: !2, size: 128, align: 32, elements: !222, templateParams: !4, identifier: "c3b7479fea7968e295ac631888456ee9::Suspend1")
          357: !222 = !{!223, !251}
check:46'0     X~~~~~~~~~~~~~~~~~~~ error: no match found
check:46'1                          with "S1" equal to "!221"
          358: !223 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !221, file: !2, baseType: !224, size: 96, align: 32, offset: 32)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          359: !224 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !225, file: !2, size: 96, align: 32, elements: !227, templateParams: !4, identifier: "aa44a0fa3b83ad9be5d32ce43f13f16e")
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          360: !225 = !DINamespace(name: "string", scope: !226)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          361: !226 = !DINamespace(name: "alloc", scope: null)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          362: !227 = !{!228}
check:46'0     ~~~~~~~~~~~~~~
            .
            .
            .
          370: !235 = !{!236, !244, !245}
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          371: !236 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !233, file: !2, baseType: !237, size: 32, align: 32)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          372: !237 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<u8>", scope: !238, file: !2, size: 32, align: 32, elements: !239, templateParams: !59, identifier: "5a8dfb3b09ce5e632db5e373a5335d4")
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          373: !238 = !DINamespace(name: "unique", scope: !55)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          374: !239 = !{!240, !241}
check:46'0     ~~~~~~~~~~~~~~~~~~~~
          375: !240 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !237, file: !2, baseType: !58, size: 32, align: 32)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:46'2     ?                                                                                                                     possible intended match
          376: !241 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !237, file: !2, baseType: !242, align: 8)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          377: !242 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<u8>", scope: !243, file: !2, align: 8, elements: !4, templateParams: !59, identifier: "9e75bceacc42a8929924b910fd9d8bfd")
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          378: !243 = !DINamespace(name: "marker", scope: !37)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          379: !244 = !DIDerivedType(tag: DW_TAG_member, name: "cap", scope: !233, file: !2, baseType: !67, size: 32, align: 32, offset: 32)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          380: !245 = !DIDerivedType(tag: DW_TAG_member, name: "alloc", scope: !233, file: !2, baseType: !246, align: 8)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen\generator-debug-msvc.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30037\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.9.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.1\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen\\generator-debug-msvc\\generator-debug-msvc.ll" "D:\\a\\rust\\rust\\src/test\\codegen\\generator-debug-msvc.rs" "--check-prefixes" "CHECK,MSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs:50:11: error: CHECK: expected string not found in input
// CHECK: {{!.*}} = !DIDerivedType(tag: DW_TAG_member, name: "variant$", scope: [[S1]],
          ^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:333:1: note: scanning from here
!198 = !{!199}
^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:333:1: note: with "S1" equal to "!197"
!198 = !{!199}
^
D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll:351:1: note: possible intended match here
!216 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !213, file: !2, baseType: !55, size: 32, align: 32)


Input file: D:\a\rust\rust\build\i686-pc-windows-msvc\test\codegen\generator-debug-msvc\generator-debug-msvc.ll
Check file: D:\a\rust\rust\src/test\codegen\generator-debug-msvc.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          328: !193 = !DICompositeType(tag: DW_TAG_structure_type, name: "Panicked", scope: !186, file: !2, size: 128, align: 32, elements: !4, templateParams: !4, identifier: "6fce3d7ad0dcf713c22d7e3ea334926::Panicked")
          329: !194 = !DIDerivedType(tag: DW_TAG_member, name: "variant3", scope: !186, file: !182, line: 15, baseType: !195, size: 128, align: 32, extraData: i64 3)
          330: !195 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend0", scope: !186, file: !2, size: 128, align: 32, elements: !4, templateParams: !4, identifier: "6fce3d7ad0dcf713c22d7e3ea334926::Suspend0")
          331: !196 = !DIDerivedType(tag: DW_TAG_member, name: "variant4", scope: !186, file: !182, line: 17, baseType: !197, size: 128, align: 32, extraData: i64 4)
          332: !197 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend1", scope: !186, file: !2, size: 128, align: 32, elements: !198, templateParams: !4, identifier: "6fce3d7ad0dcf713c22d7e3ea334926::Suspend1")
          333: !198 = !{!199}
check:50'0     X~~~~~~~~~~~~~ error: no match found
check:50'1                    with "S1" equal to "!197"
          334: !199 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !197, file: !2, baseType: !200, size: 96, align: 32)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          335: !200 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !201, file: !2, size: 96, align: 32, elements: !203, templateParams: !4, identifier: "aa44a0fa3b83ad9be5d32ce43f13f16e")
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          336: !201 = !DINamespace(name: "string", scope: !202)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          337: !202 = !DINamespace(name: "alloc", scope: null)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          338: !203 = !{!204}
check:50'0     ~~~~~~~~~~~~~~
            .
            .
            .
          346: !211 = !{!212, !220, !221}
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          347: !212 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !209, file: !2, baseType: !213, size: 32, align: 32)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          348: !213 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<u8>", scope: !214, file: !2, size: 32, align: 32, elements: !215, templateParams: !56, identifier: "5a8dfb3b09ce5e632db5e373a5335d4")
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          349: !214 = !DINamespace(name: "unique", scope: !52)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          350: !215 = !{!216, !217}
check:50'0     ~~~~~~~~~~~~~~~~~~~~
          351: !216 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !213, file: !2, baseType: !55, size: 32, align: 32)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:50'2     ?                                                                                                                     possible intended match
          352: !217 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !213, file: !2, baseType: !218, align: 8)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          353: !218 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<u8>", scope: !219, file: !2, align: 8, elements: !4, templateParams: !56, identifier: "9e75bceacc42a8929924b910fd9d8bfd")
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          354: !219 = !DINamespace(name: "marker", scope: !34)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          355: !220 = !DIDerivedType(tag: DW_TAG_member, name: "cap", scope: !209, file: !2, baseType: !64, size: 32, align: 32, offset: 32)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          356: !221 = !DIDerivedType(tag: DW_TAG_member, name: "alloc", scope: !209, file: !2, baseType: !222, align: 8)
check:50'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 200 passed; 2 failed; 76 ignored; 0 measured; 0 filtered out; finished in 3.65s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\codegen" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\codegen" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "codegen" "--mode" "codegen" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.5\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"



make: *** [Makefile:72: ci-subset-1] Error 1
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
