plain
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to D:\a\rust\rust\build\i686-pc-windows-gnu\test\ui\panics\issue-47429-short-backtraces.legacy\issue-47429-short-backtraces.legacy.run.stderr

error in revision `legacy`: 1 errors occurred comparing run output.
status: exit code: 101
command: PATH="D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.0\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.0\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.1\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.352-8\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\panics\\issue-47429-short-backtraces.legacy\\a.exe"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', D:\a\rust\rust\src/test\ui\panics\issue-47429-short-backtraces.rs:23:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------


---
test result: FAILED. 13729 passed; 1 failed; 214 ignored; 0 measured; 0 filtered out; finished in 488.82s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-gnu target=i686-pc-windows-gnu
Build completed unsuccessfully in 0:37:08
make: *** [Makefile:85: ci-mingw-subset-2] Error 1
