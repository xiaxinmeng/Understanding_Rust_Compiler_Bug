plain
Updating files:  98% (35084/35800)
Updating files:  99% (35442/35800)
Updating files: 100% (35800/35800)
Updating files: 100% (35800/35800), done.
Note: switching to 'refs/remotes/pull/100101/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at a0cae097 Merge c35be550f53c01dc5306c07e3b6378bbf5c81d28 into 6c943bad02626dddc5e5135b23c77429b6e4a063
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'a0cae097af4c84fa99f190461b07f435811f5dae'
'a0cae097af4c84fa99f190461b07f435811f5dae'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
    Finished release [optimized] target(s) in 21.02s
Check compiletest suite=run-make mode=run-make (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)

running 65 tests
.ii......ii.....ii.i...i..ii..........F.iii.i.iiiiiii.iiii......test [run-make] src/test\run-make\coverage-reports has been running for over 60 seconds
failures:


---- [run-make] src/test\run-make\rlib-format-packed-bundled-libs stdout ----
error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs'
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\cl.exe' -nologo -MT -Brepro -c -Fo:`cygpath -w /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o` native_dep_1.c
native_dep_1.c
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\lib.exe' -nologo -out:`cygpath -w /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/native_dep_1.lib` /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\cl.exe' -nologo -MT -Brepro -c -Fo:`cygpath -w /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o` native_dep_2.c
native_dep_2.c
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\lib.exe' -nologo -out:`cygpath -w /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/native_dep_2.lib` /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\cl.exe' -nologo -MT -Brepro -c -Fo:`cygpath -w /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o` native_dep_3.c
native_dep_3.c
'C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64\lib.exe' -nologo -out:`cygpath -w /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/native_dep_3.lib` /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o
# Build new rlibs
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.22000.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.10.6/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.6/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.4.1/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.345-1/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  rust_dep_up.rs --crate-type=rlib -Zpacked_bundled_libs
"D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "U native_f2"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U _ZN4core9panicking5panic17h8ad0b5584a702396E
                 U native_f2
                 U native_f3

[[[ end stdout ]]]
"D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "U native_f3"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U _ZN4core9panicking5panic17h8ad0b5584a702396E
                 U native_f2
                 U native_f3

[[[ end stdout ]]]
"D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" -e "T.*rust_dep_up"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U _ZN4core9panicking5panic17h8ad0b5584a702396E
                 U native_f2
                 U native_f3

[[[ end stdout ]]]
ar t /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "native_dep_2"
[[[ begin stdout ]]]
lib.rmeta
rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o
native_dep_2.lib
native_dep_3.lib
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc

[[[ end stdout ]]]
ar t /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "native_dep_3"
[[[ begin stdout ]]]
lib.rmeta
rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o
native_dep_2.lib
native_dep_3.lib

[[[ end stdout ]]]
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.22000.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.10.6/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.6/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.4.1/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.345-1/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  rust_dep_local.rs --extern rlib=/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib -Zpacked_bundled_libs --crate-type=rlib
"D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "U native_f1"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o:
                 U _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
---------------- T _ZN14rust_dep_local14rust_dep_local17h9562439c45fb1788E
                 U _ZN4core9panicking5panic17h8ad0b5584a702396E
                 U native_f1

[[[ end stdout ]]]
"D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" -e "T.*rust_dep_local"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o:
                 U _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
---------------- T _ZN14rust_dep_local14rust_dep_local17h9562439c45fb1788E
                 U _ZN4core9panicking5panic17h8ad0b5584a702396E
                 U native_f1

[[[ end stdout ]]]
ar t /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "native_dep_1"
[[[ begin stdout ]]]
lib.rmeta
rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o
native_dep_1.lib

[[[ end stdout ]]]
rm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o
rm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o
rm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.22000.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.10.6/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.6/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.4.1/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.345-1/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  main.rs --extern lib=/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib --crate-type=bin -Zpacked_bundled_libs --print link-args | "D:\a\rust\rust/src/etc/cat-and-grep.sh" -e "native_dep_1.*native_dep_2.*native_dep_3"
[[[ begin stdout ]]]
"C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcWmDnMS\\symbols.o" "D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.main.cbcd4161-cgu.0.rcgu.o" "D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.main.cbcd4161-cgu.1.rcgu.o" "D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.main.cbcd4161-cgu.2.rcgu.o" "D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.main.cbcd4161-cgu.3.rcgu.o" "D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.main.cbcd4161-cgu.4.rcgu.o" "D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.4g86d6gt786od2ny.rcgu.o" "/LIBPATH:D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-make\\rlib-format-packed-bundled-libs\\rlib-format-packed-bundled-libs\\librust_dep_local.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcWmDnMS\\native_dep_1.lib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-make\\rlib-format-packed-bundled-libs\\rlib-format-packed-bundled-libs\\librust_dep_up.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcWmDnMS\\native_dep_2.lib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcWmDnMS\\native_dep_3.lib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd-908ec837dccc6f59.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libpanic_unwind-297e2491bcfb8fbe.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_demangle-c480a0eabf04c8cb.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd_detect-1d72b0ee1eaad8b3.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libhashbrown-84a39cc58b9dc8a2.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libminiz_oxide-45faf690652fe18a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libadler-baacf459997bd84f.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_alloc-d2480d64abe2992a.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunwind-e81448a99e06d322.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcfg_if-c971b63f4d1bb1d7.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liblibc-9465ef77957dd0fb.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc-db8917cca783901c.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_core-4f1f79de8cc44aa9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-c5f000ceda76d9c0.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-37bd5d701b9c5d5c.rlib" "advapi32.lib" "userenv.lib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "msvcrt.lib" "legacy_stdio_definitions.lib" "/NXCOMPAT" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs\\main.exe" "/OPT:REF,NOICF" "/DEBUG"

[[[ end stdout ]]]
# Build bin
"D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "T native_f1"
[[[ begin stdout ]]]

[[[ end stdout ]]]
Error: cannot match: T native_f1
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs'
--- stderr -------------------------------
--- stderr -------------------------------
D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib:lib.rmeta: no symbols
D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib:lib.rmeta: no symbols
D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\bin\llvm-nm.exe: error: D:/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main: no such file or directory
make[1]: *** [Makefile:28: all] Error 1



failures:
failures:
    [run-make] src/test\run-make\rlib-format-packed-bundled-libs
test result: FAILED. 39 passed; 1 failed; 25 ignored; 0 measured; 0 filtered out; finished in 86.82s

Build completed unsuccessfully in 0:33:44
Build completed unsuccessfully in 0:33:44
make: *** [Makefile:73: ci-subset-1] Error 1
