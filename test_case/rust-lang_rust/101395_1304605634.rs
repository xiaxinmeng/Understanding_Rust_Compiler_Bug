plain
failures:

---- [assembly] src/test\assembly\strict_provenance.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: PATH=";D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.0\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.0\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.1\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.345-1\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\assembly\\strict_provenance\\strict_provenance.s" "D:\\a\\rust\\rust\\src/test\\assembly\\strict_provenance.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
D:\a\rust\rust\src/test\assembly\strict_provenance.rs:8:11: error: CHECK: expected string not found in input
// CHECK: movq %rdi, %rax
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:9:16: note: scanning from here
 .def old_style;
               ^
               ^
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:16:2: note: possible intended match here
 movq %rcx, %rax
D:\a\rust\rust\src/test\assembly\strict_provenance.rs:17:11: error: CHECK: expected string not found in input
D:\a\rust\rust\src/test\assembly\strict_provenance.rs:17:11: error: CHECK: expected string not found in input
// CHECK: movq %rdi, %rax
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:20:19: note: scanning from here
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:20:19: note: scanning from here
 .def cheri_compat;
                  ^
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:27:2: note: possible intended match here
 movq %rcx, %rax
D:\a\rust\rust\src/test\assembly\strict_provenance.rs:29:11: error: CHECK: expected string not found in input
D:\a\rust\rust\src/test\assembly\strict_provenance.rs:29:11: error: CHECK: expected string not found in input
// CHECK: movq %rdi, %rax
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:31:36: note: scanning from here
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:31:36: note: scanning from here
 .def definitely_not_a_null_pointer;
                                   ^
D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s:38:2: note: possible intended match here
 movq %rcx, %rax

Input file: D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\assembly\strict_provenance\strict_provenance.s
Check file: D:\a\rust\rust\src/test\assembly\strict_provenance.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text 
            1:  .text 
            2:  .def @feat.00; 
            3:  .scl 3; 
            4:  .type 0; 
            5:  .endef 
            6:  .globl @feat.00 
            7: .set @feat.00, 0 
            8:  .file "strict_provenance.f23f9f73-cgu.0" 
            9:  .def old_style; 
check:8'0                     X~ error: no match found
           10:  .scl 2; 
check:8'0      ~~~~~~~~~
           11:  .type 32; 
check:8'0      ~~~~~~~~~~~
           12:  .endef 
check:8'0      ~~~~~~~~
           13:  .globl old_style 
check:8'0      ~~~~~~~~~~~~~~~~~~
           14:  .p2align 4, 0x90 
check:8'0      ~~~~~~~~~~~~~~~~~~
           15: old_style: 
check:8'0      ~~~~~~~~~~~
           16:  movq %rcx, %rax 
check:8'0      ~~~~~~~~~~~~~~~~~
check:8'1       ?                possible intended match
           17:  orq $1, %rax 
check:8'0      ~~~~~~~~~~~~~~
           18:  retq 
check:8'0      ~~~~~~
           19:  
check:8'0      ~
           20:  .def cheri_compat; 
check:8'0      ~~~~~~~~~~~~~~~~~~
check:17'0                       X~ error: no match found
           21:  .scl 2; 
check:17'0     ~~~~~~~~~
           22:  .type 32; 
check:17'0     ~~~~~~~~~~~
           23:  .endef 
check:17'0     ~~~~~~~~
           24:  .globl cheri_compat 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~
           25:  .p2align 4, 0x90 
check:17'0     ~~~~~~~~~~~~~~~~~~
           26: cheri_compat: 
check:17'0     ~~~~~~~~~~~~~~
           27:  movq %rcx, %rax 
check:17'0     ~~~~~~~~~~~~~~~~~
check:17'1      ?                possible intended match
           28:  orq $1, %rax 
check:17'0     ~~~~~~~~~~~~~~
           29:  retq 
check:17'0     ~~~~~~
           30:  
check:17'0     ~
           31:  .def definitely_not_a_null_pointer; 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:29'0                                        X~ error: no match found
           32:  .scl 2; 
check:29'0     ~~~~~~~~~
           33:  .type 32; 
check:29'0     ~~~~~~~~~~~
           34:  .endef 
check:29'0     ~~~~~~~~
           35:  .globl definitely_not_a_null_pointer 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36:  .p2align 4, 0x90 
check:29'0     ~~~~~~~~~~~~~~~~~~
           37: definitely_not_a_null_pointer: 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38:  movq %rcx, %rax 
check:29'0     ~~~~~~~~~~~~~~~~~
check:29'1      ?                possible intended match
           39:  orq $1, %rax 
check:29'0     ~~~~~~~~~~~~~~
           40:  retq 
check:29'0     ~~~~~~
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
check:29'0     ~
------------------------------------------




failures:
    [assembly] src/test\assembly\strict_provenance.rs

test result: FAILED. 120 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out; finished in 1.51s

Build completed unsuccessfully in 0:28:08
make: *** [Makefile:83: ci-mingw-subset-1] Error 1
