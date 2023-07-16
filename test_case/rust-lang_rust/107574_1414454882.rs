plain
failures:

---- [ui] tests\ui\numbers-arithmetic\overflowing-rsh-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(-1073741819).
status: exit code: 0xc0000005
command: PATH="D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.4\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\numbers-arithmetic\\overflowing-rsh-3.rs" "-Zthreads=1" "--target=i686-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\numbers-arithmetic\\overflowing-rsh-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\numbers-arithmetic\\overflowing-rsh-3\\auxiliary" "-C" "debug-assertions"
stdout: none
--- stderr -------------------------------
error: this arithmetic operation will overflow
  --> fake-test-src-base\numbers-arithmetic\overflowing-rsh-3.rs:7:14
   |
LL |     let _x = -1_i64 >> 64;
   |              ^^^^^^^^^^^^ attempt to shift right by `64_i32`, which would overflow
note: the lint level is defined here
note: the lint level is defined here
  --> fake-test-src-base\numbers-arithmetic\overflowing-rsh-3.rs:4:9
   |
LL | #![deny(arithmetic_overflow)]

error: aborting due to previous error
------------------------------------------

---
test result: FAILED. 14163 passed; 1 failed; 219 ignored; 0 measured; 0 filtered out; finished in 1187.32s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-gnu target=i686-pc-windows-gnu
Build completed successfully in 0:48:32
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
