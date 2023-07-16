plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make-fulldeps/coverage-spanview'
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
# Compile the test library with coverage instrumentation
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.9.1/x64/Scripts:/c/hostedtoolcache/windows/Python/3.9.1/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Users/runneradmin/.dotnet/tools:/c/Program Files/MongoDB/Server/4.4/bin:/c/aliyun-cli:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.5.1/x64:/c/ProgramData/chocolatey/lib/ghc.8.10.2.2/tools/ghc-8.10.2/bin:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/R/R-4.0.3/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Rust/.cargo/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.14.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.8/x64/bin:/c/Program Files/Java/jdk8u275-b01/bin:/c/npm/prefix:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.3/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/Program Files/TortoiseSVN/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/CMake/bin:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/doctest_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/doctest_crate.rs && echo "--edition=2018" ) \
  -Zinstrument-coverage \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate
for path in "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("C:\hostedtoolcache\windows\Python\3.9.1\x64\python3.exe" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "doctest_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate
rm -f "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/*
cp "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*InstrumentCoverage.0.html "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest_crate/ "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
# Compile the test library with coverage instrumentation
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.9.1/x64/Scripts:/c/hostedtoolcache/windows/Python/3.9.1/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Users/runneradmin/.dotnet/tools:/c/Program Files/MongoDB/Server/4.4/bin:/c/aliyun-cli:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.5.1/x64:/c/ProgramData/chocolatey/lib/ghc.8.10.2.2/tools/ghc-8.10.2/bin:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/R/R-4.0.3/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Rust/.cargo/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.14.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.8/x64/bin:/c/Program Files/Java/jdk8u275-b01/bin:/c/npm/prefix:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.3/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/Program Files/TortoiseSVN/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/CMake/bin:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/used_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/used_crate.rs && echo "--edition=2018" ) \
  -Zinstrument-coverage \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate
for path in "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("C:\hostedtoolcache\windows\Python\3.9.1\x64\python3.exe" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "used_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate
rm -f "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/*
cp "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate/*InstrumentCoverage.0.html "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.used_crate/ "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/
# Compile the test program with coverage instrumentation
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.9.1/x64/Scripts:/c/hostedtoolcache/windows/Python/3.9.1/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Users/runneradmin/.dotnet/tools:/c/Program Files/MongoDB/Server/4.4/bin:/c/aliyun-cli:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.5.1/x64:/c/ProgramData/chocolatey/lib/ghc.8.10.2.2/tools/ghc-8.10.2/bin:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/R/R-4.0.3/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Rust/.cargo/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.14.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.8/x64/bin:/c/Program Files/Java/jdk8u275-b01/bin:/c/npm/prefix:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.3/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/Program Files/TortoiseSVN/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/CMake/bin:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/abort.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/abort.rs && echo "--edition=2018" ) \
  -L "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.abort
for path in "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.abort/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("C:\hostedtoolcache\windows\Python\3.9.1\x64\python3.exe" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "abort" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort
rm -f "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort/*
cp "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.abort/*InstrumentCoverage.0.html "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort/
diff -u --strip-trailing-cr -r expected_mir_dump.abort/ "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.abort/
# Compile the test program with coverage instrumentation
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.9.1/x64/Scripts:/c/hostedtoolcache/windows/Python/3.9.1/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Users/runneradmin/.dotnet/tools:/c/Program Files/MongoDB/Server/4.4/bin:/c/aliyun-cli:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.5.1/x64:/c/ProgramData/chocolatey/lib/ghc.8.10.2.2/tools/ghc-8.10.2/bin:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/R/R-4.0.3/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Rust/.cargo/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.14.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.8/x64/bin:/c/Program Files/Java/jdk8u275-b01/bin:/c/npm/prefix:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.3/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/Program Files/TortoiseSVN/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/CMake/bin:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/assert.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/assert.rs && echo "--edition=2018" ) \
  -L "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert
for path in "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("C:\hostedtoolcache\windows\Python\3.9.1\x64\python3.exe" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "assert" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert
rm -f "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/*
cp "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert/*InstrumentCoverage.0.html "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/
diff -u --strip-trailing-cr -r expected_mir_dump.assert/ "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/
# Compile the test program with coverage instrumentation
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.9.1/x64/Scripts:/c/hostedtoolcache/windows/Python/3.9.1/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Users/runneradmin/.dotnet/tools:/c/Program Files/MongoDB/Server/4.4/bin:/c/aliyun-cli:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.5.1/x64:/c/ProgramData/chocolatey/lib/ghc.8.10.2.2/tools/ghc-8.10.2/bin:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/R/R-4.0.3/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Rust/.cargo/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/hostedtoolcache/windows/go/1.14.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.8/x64/bin:/c/Program Files/Java/jdk8u275-b01/bin:/c/npm/prefix:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files/Microsoft SDKs/Azure/Azure Dev Spaces CLI:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.3/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/Program Files/TortoiseSVN/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/CMake/bin:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/async.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/async.rs && echo "--edition=2018" ) \
  -L "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.async
for path in "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.async/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("C:\hostedtoolcache\windows\Python\3.9.1\x64\python3.exe" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "async" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async
rm -f "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async/*
cp "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.async/*InstrumentCoverage.0.html "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async/
diff -u --strip-trailing-cr -r expected_mir_dump.async/ "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.async/
diff -u --strip-trailing-cr -r expected_mir_dump.async/async.executor-block_on-VTABLE-{closure#0}.-------.InstrumentCoverage.0.html /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.async/async.executor-block_on-VTABLE-{closure#0}.-------.InstrumentCoverage.0.html
--- expected_mir_dump.async/async.executor-block_on-VTABLE-{closure#0}.-------.InstrumentCoverage.0.html 2021-01-24 05:47:43.823856200 +0000
+++ /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.async/async.executor-block_on-VTABLE-{closure#0}.-------.InstrumentCoverage.0.html 2021-01-24 06:50:05.168557300 +0000
@@ -69,7 +69,7 @@
 </style>
 </head>
 <body>
-<div class="code" style="counter-reset: line 16"><span class="line">        <span><span class="code even" style="--layer: 1" title="17:38-17:74: @1[3]: _13 = (move _14,)
+<div class="code" style="counter-reset: line 15"><span class="line">                                 <span><span class="code even" style="--layer: 1" title="17:38-17:74: @1[3]: _13 = (move _14,)
 17:38-17:74: @1[5]: FakeRead(ForMatchedPlace, _13)
 17:38-17:74: @1[7]: _25 = (_13.0: &amp;std::fmt::Arguments)
 17:38-17:74: @1[10]: _27 = &amp;(*_25)
@@ -79,6 +79,9 @@
 17:38-17:74: @2[5]: _11 = &amp;_12
 17:38-17:74: @2[6]: _10 = &amp;(*_11)
 17:38-17:74: @2[7]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-17:38-17:74: @2.Call: _4 = Arguments::new_v1(move _5, move _9) -&gt; [return: bb3, unwind: bb4]"><span class="annotation">@0,1,2,3⦊</span>‸<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0">$crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))</span></span></div>
+17:38-17:74: @2.Call: _4 = Arguments::new_v1(move _5, move _9) -&gt; [return: bb3, unwind: bb4]
+17:9-17:75: @3.Call: panic_fmt(move _4) -&gt; bb4"><span class="annotation">@0,1,2,3⦊</span>‸<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0">{</span></span>
+<span class="line"><span class="code" style="--layer: 0">        $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))</span></span>
+<span class="line"><span class="code" style="--layer: 0">    }</span></span></div>
 </body>
 </html>
diff -u --strip-trailing-cr -r expected_mir_dump.async/async.executor-block_on-VTABLE-{closure#1}.-------.InstrumentCoverage.0.html /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.async/async.executor-block_on-VTABLE-{closure#1}.-------.InstrumentCoverage.0.html
--- expected_mir_dump.async/async.executor-block_on-VTABLE-{closure#1}.-------.InstrumentCoverage.0.html 2021-01-24 05:47:43.823856200 +0000
+++ /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.async/async.executor-block_on-VTABLE-{closure#1}.-------.InstrumentCoverage.0.html 2021-01-24 06:50:05.169558900 +0000
@@ -69,7 +69,7 @@
 </style>
 </head>
 <body>
-<div class="code" style="counter-reset: line 16"><span class="line">        <span><span class="code even" style="--layer: 1" title="17:38-17:74: @1[3]: _13 = (move _14,)
+<div class="code" style="counter-reset: line 15"><span class="line">                                 <span><span class="code even" style="--layer: 1" title="17:38-17:74: @1[3]: _13 = (move _14,)
 17:38-17:74: @1[5]: FakeRead(ForMatchedPlace, _13)
 17:38-17:74: @1[7]: _25 = (_13.0: &amp;std::fmt::Arguments)
 17:38-17:74: @1[10]: _27 = &amp;(*_25)
@@ -79,6 +79,9 @@
 17:38-17:74: @2[5]: _11 = &amp;_12
 17:38-17:74: @2[6]: _10 = &amp;(*_11)
 17:38-17:74: @2[7]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-17:38-17:74: @2.Call: _4 = Arguments::new_v1(move _5, move _9) -&gt; [return: bb3, unwind: bb4]"><span class="annotation">@0,1,2,3⦊</span>‸<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0">$crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))</span></span></div>
+17:38-17:74: @2.Call: _4 = Arguments::new_v1(move _5, move _9) -&gt; [return: bb3, unwind: bb4]
+17:9-17:75: @3.Call: panic_fmt(move _4) -&gt; bb4"><span class="annotation">@0,1,2,3⦊</span>‸<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0">{</span></span>
+<span class="line"><span class="code" style="--layer: 0">        $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))</span></span>
+<span class="line"><span class="code" style="--layer: 0">    }</span></span></div>
 </body>
 </html>
diff -u --strip-trailing-cr -r expected_mir_dump.async/async.executor-block_on-VTABLE-{closure#2}.-------.InstrumentCoverage.0.html /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.async/async.executor-block_on-VTABLE-{closure#2}.-------.InstrumentCoverage.0.html
--- expected_mir_dump.async/async.executor-block_on-VTABLE-{closure#2}.-------.InstrumentCoverage.0.html 2021-01-24 05:47:43.823856200 +0000
+++ /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.async/async.executor-block_on-VTABLE-{closure#2}.-------.InstrumentCoverage.0.html 2021-01-24 06:50:05.170554600 +0000
@@ -69,7 +69,7 @@
 </style>
 </head>
 <body>
-<div class="code" style="counter-reset: line 16"><span class="line">        <span><span class="code even" style="--layer: 1" title="17:38-17:74: @1[3]: _13 = (move _14,)
+<div class="code" style="counter-reset: line 15"><span class="line">                                 <span><span class="code even" style="--layer: 1" title="17:38-17:74: @1[3]: _13 = (move _14,)
 17:38-17:74: @1[5]: FakeRead(ForMatchedPlace, _13)
 17:38-17:74: @1[7]: _25 = (_13.0: &amp;std::fmt::Arguments)
 17:38-17:74: @1[10]: _27 = &amp;(*_25)
@@ -79,6 +79,9 @@
 17:38-17:74: @2[5]: _11 = &amp;_12
 17:38-17:74: @2[6]: _10 = &amp;(*_11)
 17:38-17:74: @2[7]: _9 = move _10 as &amp;[std::fmt::ArgumentV1] (Pointer(Unsize))
-17:38-17:74: @2.Call: _4 = Arguments::new_v1(move _5, move _9) -&gt; [return: bb3, unwind: bb4]"><span class="annotation">@0,1,2,3⦊</span>‸<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0">$crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))</span></span></div>
+17:38-17:74: @2.Call: _4 = Arguments::new_v1(move _5, move _9) -&gt; [return: bb3, unwind: bb4]
+17:9-17:75: @3.Call: panic_fmt(move _4) -&gt; bb4"><span class="annotation">@0,1,2,3⦊</span>‸<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0">{</span></span>
+<span class="line"><span class="code" style="--layer: 0">        $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+))</span></span>
+<span class="line"><span class="code" style="--layer: 0">    }</span></span></div>
 </body>
 </html>
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make-fulldeps/coverage-spanview'
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: function is never used: `unused_private_function`
  --> ../coverage/lib/used_crate.rs:45:4
45 | fn unused_private_function() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

make[1]: *** [Makefile:81: async] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps\coverage-spanview

test result: FAILED. 180 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 60.34s

make: *** [Makefile:72: ci-subset-1] Error 1


command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--rust-demangler-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\rust-demangler.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\run-make-fulldeps" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-make-fulldeps" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.1\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.1\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin" "--cc" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.28.29333\\bin\\HostX64\\x64\\cl.exe" "--cxx" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.28.29333\\bin\\HostX64\\x64\\cl.exe" "--cflags" "-nologo -MT -Brepro" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:59:27
