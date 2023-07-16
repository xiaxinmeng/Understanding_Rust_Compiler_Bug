plain
test [run-make] src/test\run-make\track-path-dep-info ... ok

failures:

---- [run-make] src/test\run-make\rlib-format-packed-bundled-libs-2 stdout ----
error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs-2'
# Build strange-named dep.
PATH="/d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin:/d/a/rust/rust/build/i686-pc-windows-gnu/stage0-bootstrap-tools/i686-pc-windows-gnu/release/deps:/d/a/rust/rust/build/i686-pc-windows-gnu/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/mingw32/bin:/c/hostedtoolcache/windows/Python/3.10.6/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.6/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.4.2/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.345-1/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  native_dep.rs --crate-type=staticlib -o /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/native_dep.ext
Some tests failed in compiletest suite=run-make mode=run-make host=i686-pc-windows-gnu target=i686-pc-windows-gnu
PATH="/d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin:/d/a/rust/rust/build/i686-pc-windows-gnu/stage0-bootstrap-tools/i686-pc-windows-gnu/release/deps:/d/a/rust/rust/build/i686-pc-windows-gnu/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/mingw32/bin:/c/hostedtoolcache/windows/Python/3.10.6/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.6/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.4.2/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.345-1/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  rust_dep.rs --crate-type=rlib -Zpacked_bundled_libs
"D:\a\rust\rust\build\i686-pc-windows-gnu\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "U native_f1"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep.rust_dep.f8908835-cgu.0.rcgu.o:
         U __ZN4core9panicking5panic17hfa8f2b8e42b737ecE
-------- T __ZN8rust_dep8rust_dep17h46da1ffdf057d344E
         U _native_f1

[[[ end stdout ]]]
Error: cannot match: U native_f1
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs-2'
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted
warning: 1 warning emitted

D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib:lib.rmeta: no symbols
make[1]: *** [Makefile:15: all] Error 1


---- [run-make] src/test\run-make\rlib-format-packed-bundled-libs stdout ----


error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs'
gcc.exe -ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer -v -c -o /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o native_dep_1.c
ar crus /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.a /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o
gcc.exe -ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer -v -c -o /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o native_dep_2.c
ar crus /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.a /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o
gcc.exe -ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer -v -c -o /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o native_dep_3.c
ar crus /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.a /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o
PATH="/d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin:/d/a/rust/rust/build/i686-pc-windows-gnu/stage0-bootstrap-tools/i686-pc-windows-gnu/release/deps:/d/a/rust/rust/build/i686-pc-windows-gnu/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/mingw32/bin:/c/hostedtoolcache/windows/Python/3.10.6/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.6/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.4.2/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.13/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.345-1/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  rust_dep_up.rs --crate-type=rlib -Zpacked_bundled_libs
"D:\a\rust\rust\build\i686-pc-windows-gnu\llvm\build\bin"/llvm-nm /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "D:\a\rust\rust/src/etc/cat-and-grep.sh" "U native_f2"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
-------- T __ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
         U __ZN4core9panicking5panic17hfa8f2b8e42b737ecE
         U _native_f2
         U _native_f3

[[[ end stdout ]]]
Error: cannot match: U native_f2
rm /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o /d/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs'
--- stderr -------------------------------
Using built-in specs.
Using built-in specs.
COLLECT_GCC=D:\a\rust\rust\mingw32\bin\gcc.exe
Target: i686-w64-mingw32
Configured with: ../../../src/gcc-6.3.0/configure --host=i686-w64-mingw32 --build=i686-w64-mingw32 --target=i686-w64-mingw32 --prefix=/mingw32 --with-sysroot=/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32 --enable-shared --enable-static --disable-multilib --enable-languages=c,c++,fortran,lto --enable-libstdcxx-time=yes --enable-threads=posix --enable-libgomp --enable-libatomic --enable-lto --enable-graphite --enable-checking=release --enable-fully-dynamic-string --enable-version-specific-runtime-libs --enable-libstdcxx-filesystem-ts=yes --disable-sjlj-exceptions --with-dwarf2 --disable-libstdcxx-pch --disable-libstdcxx-debug --disable-bootstrap --disable-rpath --disable-win32-registry --disable-nls --disable-werror --disable-symvers --with-gnu-as --with-gnu-ld --with-arch=i686 --with-tune=generic --with-libiconv --with-system-zlib --with-gmp=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-mpfr=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-mpc=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-isl=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-pkgversion='i686-posix-dwarf-rev2, Built by MinGW-W64 project' --with-bugurl=http://sourceforge.net/projects/mingw-w64 CFLAGS='-O2 -pipe -fno-ident -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' CXXFLAGS='-O2 -pipe -fno-ident -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' CPPFLAGS=' -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' LDFLAGS='-pipe -fno-ident -L/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/lib -L/c/mingw630/prerequisites/i686-zlib-static/lib -L/c/mingw630/prerequisites/i686-w64-mingw32-static/lib -Wl,--large-address-aware'
Thread model: posix
gcc version 6.3.0 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) 
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o' '-mtune=generic' '-march=i686'
 D:/a/rust/rust/mingw32/bin/../libexec/gcc/i686-w64-mingw32/6.3.0/cc1.exe -quiet -v -iprefix D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/ -D_REENTRANT native_dep_1.c -quiet -dumpbase native_dep_1.c -m32 -mtune=generic -march=i686 -auxbase-strip D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o -version -ffunction-sections -fdata-sections -fno-omit-frame-pointer -o C:\Users\RUNNER~1\AppData\Local\Temp\ccKg48gz.s
GNU C11 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) version 6.3.0 (i686-w64-mingw32)
 compiled by GNU C version 6.2.0, GMP version 6.1.0, MPFR version 3.1.4, MPC version 1.0.3, isl version 0.15
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/include"
ignoring nonexistent directory "C:/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32C:/msys64/mingw32/lib/gcc/i686-w64-mingw32/6.3.0/../../../../include"
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/include-fixed"
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/include"
ignoring nonexistent directory "C:/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/mingw/include"
#include "..." search starts here:
#include <...> search starts here:
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/include
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/include-fixed
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/include
End of search list.
GNU C11 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) version 6.3.0 (i686-w64-mingw32)
 compiled by GNU C version 6.2.0, GMP version 6.1.0, MPFR version 3.1.4, MPC version 1.0.3, isl version 0.15
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: 7cead2934c4afc04ec8ebe53d86f8372
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o' '-mtune=generic' '-march=i686'
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/as.exe -v --32 -o D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o C:\Users\RUNNER~1\AppData\Local\Temp\ccKg48gz.s
GNU assembler version 2.27 (i686-w64-mingw32) using BFD version (GNU Binutils) 2.27
COMPILER_PATH=D:/a/rust/rust/mingw32/bin/../libexec/gcc/i686-w64-mingw32/6.3.0/;D:/a/rust/rust/mingw32/bin/../libexec/gcc/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/
LIBRARY_PATH=D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/;D:/a/rust/rust/mingw32/bin/../lib/gcc/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/lib/../lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o' '-mtune=generic' '-march=i686'
Using built-in specs.
COLLECT_GCC=D:\a\rust\rust\mingw32\bin\gcc.exe
Target: i686-w64-mingw32
Configured with: ../../../src/gcc-6.3.0/configure --host=i686-w64-mingw32 --build=i686-w64-mingw32 --target=i686-w64-mingw32 --prefix=/mingw32 --with-sysroot=/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32 --enable-shared --enable-static --disable-multilib --enable-languages=c,c++,fortran,lto --enable-libstdcxx-time=yes --enable-threads=posix --enable-libgomp --enable-libatomic --enable-lto --enable-graphite --enable-checking=release --enable-fully-dynamic-string --enable-version-specific-runtime-libs --enable-libstdcxx-filesystem-ts=yes --disable-sjlj-exceptions --with-dwarf2 --disable-libstdcxx-pch --disable-libstdcxx-debug --disable-bootstrap --disable-rpath --disable-win32-registry --disable-nls --disable-werror --disable-symvers --with-gnu-as --with-gnu-ld --with-arch=i686 --with-tune=generic --with-libiconv --with-system-zlib --with-gmp=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-mpfr=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-mpc=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-isl=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-pkgversion='i686-posix-dwarf-rev2, Built by MinGW-W64 project' --with-bugurl=http://sourceforge.net/projects/mingw-w64 CFLAGS='-O2 -pipe -fno-ident -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' CXXFLAGS='-O2 -pipe -fno-ident -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' CPPFLAGS=' -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' LDFLAGS='-pipe -fno-ident -L/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/lib -L/c/mingw630/prerequisites/i686-zlib-static/lib -L/c/mingw630/prerequisites/i686-w64-mingw32-static/lib -Wl,--large-address-aware'
Thread model: posix
gcc version 6.3.0 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) 
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o' '-mtune=generic' '-march=i686'
 D:/a/rust/rust/mingw32/bin/../libexec/gcc/i686-w64-mingw32/6.3.0/cc1.exe -quiet -v -iprefix D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/ -D_REENTRANT native_dep_2.c -quiet -dumpbase native_dep_2.c -m32 -mtune=generic -march=i686 -auxbase-strip D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o -version -ffunction-sections -fdata-sections -fno-omit-frame-pointer -o C:\Users\RUNNER~1\AppData\Local\Temp\ccMbsEdI.s
GNU C11 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) version 6.3.0 (i686-w64-mingw32)
 compiled by GNU C version 6.2.0, GMP version 6.1.0, MPFR version 3.1.4, MPC version 1.0.3, isl version 0.15
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/include"
ignoring nonexistent directory "C:/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32C:/msys64/mingw32/lib/gcc/i686-w64-mingw32/6.3.0/../../../../include"
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/include-fixed"
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/include"
ignoring nonexistent directory "C:/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/mingw/include"
#include "..." search starts here:
#include <...> search starts here:
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/include
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/include-fixed
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/include
End of search list.
GNU C11 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) version 6.3.0 (i686-w64-mingw32)
 compiled by GNU C version 6.2.0, GMP version 6.1.0, MPFR version 3.1.4, MPC version 1.0.3, isl version 0.15
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: 7cead2934c4afc04ec8ebe53d86f8372
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o' '-mtune=generic' '-march=i686'
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/as.exe -v --32 -o D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o C:\Users\RUNNER~1\AppData\Local\Temp\ccMbsEdI.s
GNU assembler version 2.27 (i686-w64-mingw32) using BFD version (GNU Binutils) 2.27
COMPILER_PATH=D:/a/rust/rust/mingw32/bin/../libexec/gcc/i686-w64-mingw32/6.3.0/;D:/a/rust/rust/mingw32/bin/../libexec/gcc/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/
LIBRARY_PATH=D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/;D:/a/rust/rust/mingw32/bin/../lib/gcc/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/lib/../lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o' '-mtune=generic' '-march=i686'
Using built-in specs.
COLLECT_GCC=D:\a\rust\rust\mingw32\bin\gcc.exe
Target: i686-w64-mingw32
Configured with: ../../../src/gcc-6.3.0/configure --host=i686-w64-mingw32 --build=i686-w64-mingw32 --target=i686-w64-mingw32 --prefix=/mingw32 --with-sysroot=/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32 --enable-shared --enable-static --disable-multilib --enable-languages=c,c++,fortran,lto --enable-libstdcxx-time=yes --enable-threads=posix --enable-libgomp --enable-libatomic --enable-lto --enable-graphite --enable-checking=release --enable-fully-dynamic-string --enable-version-specific-runtime-libs --enable-libstdcxx-filesystem-ts=yes --disable-sjlj-exceptions --with-dwarf2 --disable-libstdcxx-pch --disable-libstdcxx-debug --disable-bootstrap --disable-rpath --disable-win32-registry --disable-nls --disable-werror --disable-symvers --with-gnu-as --with-gnu-ld --with-arch=i686 --with-tune=generic --with-libiconv --with-system-zlib --with-gmp=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-mpfr=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-mpc=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-isl=/c/mingw630/prerequisites/i686-w64-mingw32-static --with-pkgversion='i686-posix-dwarf-rev2, Built by MinGW-W64 project' --with-bugurl=http://sourceforge.net/projects/mingw-w64 CFLAGS='-O2 -pipe -fno-ident -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' CXXFLAGS='-O2 -pipe -fno-ident -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' CPPFLAGS=' -I/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/include -I/c/mingw630/prerequisites/i686-zlib-static/include -I/c/mingw630/prerequisites/i686-w64-mingw32-static/include' LDFLAGS='-pipe -fno-ident -L/c/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/opt/lib -L/c/mingw630/prerequisites/i686-zlib-static/lib -L/c/mingw630/prerequisites/i686-w64-mingw32-static/lib -Wl,--large-address-aware'
Thread model: posix
gcc version 6.3.0 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) 
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o' '-mtune=generic' '-march=i686'
 D:/a/rust/rust/mingw32/bin/../libexec/gcc/i686-w64-mingw32/6.3.0/cc1.exe -quiet -v -iprefix D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/ -D_REENTRANT native_dep_3.c -quiet -dumpbase native_dep_3.c -m32 -mtune=generic -march=i686 -auxbase-strip D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o -version -ffunction-sections -fdata-sections -fno-omit-frame-pointer -o C:\Users\RUNNER~1\AppData\Local\Temp\ccygvhLO.s
GNU C11 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) version 6.3.0 (i686-w64-mingw32)
 compiled by GNU C version 6.2.0, GMP version 6.1.0, MPFR version 3.1.4, MPC version 1.0.3, isl version 0.15
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/include"
ignoring nonexistent directory "C:/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32C:/msys64/mingw32/lib/gcc/i686-w64-mingw32/6.3.0/../../../../include"
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/include-fixed"
ignoring duplicate directory "D:/a/rust/rust/mingw32/lib/gcc/../../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/include"
ignoring nonexistent directory "C:/mingw630/i686-630-posix-dwarf-rt_v5-rev2/mingw32/mingw/include"
#include "..." search starts here:
#include <...> search starts here:
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/include
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/include-fixed
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/include
End of search list.
GNU C11 (i686-posix-dwarf-rev2, Built by MinGW-W64 project) version 6.3.0 (i686-w64-mingw32)
 compiled by GNU C version 6.2.0, GMP version 6.1.0, MPFR version 3.1.4, MPC version 1.0.3, isl version 0.15
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: 7cead2934c4afc04ec8ebe53d86f8372
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o' '-mtune=generic' '-march=i686'
 D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/as.exe -v --32 -o D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o C:\Users\RUNNER~1\AppData\Local\Temp\ccygvhLO.s
GNU assembler version 2.27 (i686-w64-mingw32) using BFD version (GNU Binutils) 2.27
COMPILER_PATH=D:/a/rust/rust/mingw32/bin/../libexec/gcc/i686-w64-mingw32/6.3.0/;D:/a/rust/rust/mingw32/bin/../libexec/gcc/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/
LIBRARY_PATH=D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/;D:/a/rust/rust/mingw32/bin/../lib/gcc/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/lib/../lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/lib/;D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-m32' '-fno-omit-frame-pointer' '-v' '-c' '-o' 'D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o' '-mtune=generic' '-march=i686'
D:/a/rust/rust/build/i686-pc-windows-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
make[1]: *** [Makefile:13: all] Error 1



failures:
failures:
    [run-make] src/test\run-make\rlib-format-packed-bundled-libs
    [run-make] src/test\run-make\rlib-format-packed-bundled-libs-2

test result: FAILED. 39 passed; 2 failed; 25 ignored; 0 measured; 0 filtered out; finished in 9.52s

Build completed unsuccessfully in 1:13:46
make: *** [Makefile:83: ci-mingw-subset-1] Error 1
