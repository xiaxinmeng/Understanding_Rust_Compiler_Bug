plain
37 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\i686-pc-windows-msvc\test\ui\coercion\coerce-issue-49593-box-never.nofallback\coerce-issue-49593-box-never.nofallback.stderr
To only update this specific test, also pass `--test-args coercion\coerce-issue-49593-box-never.rs`


error in revision `nofallback`: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.17.8\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\ui\\coercion\\coerce-issue-49593-box-never.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "--cfg" "nofallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\coercion\\coerce-issue-49593-box-never.nofallback" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\coercion\\coerce-issue-49593-box-never.nofallback\\auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(): std::error::Error` is not satisfied
   |
   |
LL |     /* *mut $0 is coerced to Box<dyn Error> here */ Box::<_ /* ! */>::new(x)
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
   = help: the following other types implement trait `std::error::Error`:
             !
             &'a T
             AccessError
---

error[E0277]: the trait bound `(): std::error::Error` is not satisfied
  --> D:\a\rust\rust\src/test\ui\coercion\coerce-issue-49593-box-never.rs:22:49
   |
LL |     /* *mut $0 is coerced to *mut Error here */ raw_ptr_box::<_ /* ! */>(x)
   |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
   = help: the following other types implement trait `std::error::Error`:
             !
             &'a T
             AccessError
---
test result: FAILED. 12733 passed; 1 failed; 192 ignored; 0 measured; 0 filtered out; finished in 520.35s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-msvc target=i686-pc-windows-msvc
Build completed unsuccessfully in 0:39:48
make: *** [Makefile:74: ci-subset-2] Error 1
