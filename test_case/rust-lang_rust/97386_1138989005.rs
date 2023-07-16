plain
error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make-fulldeps/stdin-non-utf8'
cp non-utf8 /d/a/rust/rust/build/x86_64-pc-windows-gnu/test/run-make-fulldeps/stdin-non-utf8/stdin-non-utf8/non-utf.rs
cat /d/a/rust/rust/build/x86_64-pc-windows-gnu/test/run-make-fulldeps/stdin-non-utf8/stdin-non-utf8/non-utf.rs | PATH="/d/a/rust/rust/build/x86_64-pc-windows-gnu/test/run-make-fulldeps/stdin-non-utf8/stdin-non-utf8:D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin:/d/a/rust/rust/build/x86_64-pc-windows-gnu/stage0-bootstrap-tools/x86_64-pc-windows-gnu/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-gnu/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/mingw64/bin:/c/hostedtoolcache/windows/Python/3.10.4/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.4/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.2.2/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.0/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.17.10/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.332-9/x64/bin:/c/npm/prefix:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.5/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-gnu/test/run-make-fulldeps/stdin-non-utf8/stdin-non-utf8 -L /d/a/rust/rust/build/x86_64-pc-windows-gnu/test/run-make-fulldeps/stdin-non-utf8/stdin-non-utf8  - 2>&1 \
 | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "error: couldn't read from stdin, as it did not contain valid UTF-8"
[[[ begin stdout ]]]
I/O error: operation failed to complete synchronously

[[[ end stdout ]]]
Error: cannot match: error: couldn't read from stdin, as it did not contain valid UTF-8
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make-fulldeps/stdin-non-utf8'
--- stderr -------------------------------
--- stderr -------------------------------
make[1]: *** [Makefile:5: all] Error 1



failures:
failures:
    [run-make] src/test\run-make-fulldeps\stdin-non-utf8

Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
test result: FAILED. 192 passed; 1 failed; 37 ignored; 0 measured; 0 filtered out; finished in 81.43s

Build completed unsuccessfully in 1:24:11
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
