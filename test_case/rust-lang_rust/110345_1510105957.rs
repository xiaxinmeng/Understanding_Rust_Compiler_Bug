plain
failures:

---- [rustdoc] tests\rustdoc\trait-impl.rs stdout ----

error: rustdoc failed!
status: exit code: 0xc00000fd
command: PATH="C:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;C:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;C:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\a\rust\rust\ninja;C:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.20.2\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\Program Files\OpenSSL\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustdoc.exe" "-L" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-L" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\rustdoc\\trait-impl\\auxiliary" "-o" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\rustdoc\\trait-impl" "--deny" "warnings" "C:\\a\\rust\\rust\\tests\\rustdoc\\trait-impl.rs"
stdout: none
--- stderr -------------------------------
thread 'main' has overflowed its stack



failures:
failures:
    [rustdoc] tests\rustdoc\trait-impl.rs

test result: FAILED. 601 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 107.09s

Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
Build completed unsuccessfully in 0:49:22
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
