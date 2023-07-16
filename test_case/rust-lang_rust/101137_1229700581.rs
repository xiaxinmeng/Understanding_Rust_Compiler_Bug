plain
test [ui] src/test\ui\threads-sendsync\mpsc_stress.rs ... ok

failures:

---- [ui] src/test\ui\unpretty\avoid-crash.rs stdout ----


- error: pretty-print failed to write `/tmp/` due to error `Is a directory (os error 21)`
+ error: pretty-print failed to write `/tmp/` due to error `The filename, directory name, or volume label syntax is incorrect. (os error 123)`
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\ui\unpretty\avoid-crash\avoid-crash.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unpretty\avoid-crash.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;C:\Program Files\PowerShell\7;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.345-1\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\ui\\unpretty\\avoid-crash.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\unpretty\\avoid-crash" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-o/tmp/" "-Zunpretty=ast-tree" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\unpretty\\avoid-crash\\auxiliary"
stdout: none
--- stderr -------------------------------
error: pretty-print failed to write `/tmp/` due to error `The filename, directory name, or volume label syntax is incorrect. (os error 123)`
error: aborting due to previous error
------------------------------------------


---
test result: FAILED. 13290 passed; 1 failed; 152 ignored; 0 measured; 0 filtered out; finished in 522.62s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:36:07
make: *** [Makefile:75: ci-subset-2] Error 1
