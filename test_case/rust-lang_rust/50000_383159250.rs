
Check compiletest suite=run-make mode=run-make (i686-pc-windows-gnu -> i686-pc-windows-gnu)
running 4 tests
test [run-make] run-make\wasm-custom-section ... ok
test [run-make] run-make\wasm-import-module ... ok
test [run-make] run-make\wasm-panic-small ... ok
test [run-make] run-make\cross-lang-lto ... FAILED
failures:
---- [run-make] run-make\cross-lang-lto stdout ----
	
error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
make[1]: Entering directory '/c/projects/rust/src/test/run-make/cross-lang-lto'
PATH="/c/projects/rust/build/i686-pc-windows-gnu/test/run-make/cross-lang-lto.stage2-i686-pc-windows-gnu:C:\projects\rust\build\i686-pc-windows-gnu\stage2\bin:/c/projects/rust/build/i686-pc-windows-gnu/llvm/build/bin:/c/Program Files (x86)/Inno Setup 5:/c/Python27:/c/projects/rust/mingw32/bin:/usr/bin:/c/Perl/site/bin:/c/Perl/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Program Files/7-Zip:/c/Program Files/Microsoft/Web Platform Installer:/c/Tools/GitVersion:/c/Tools/PsTools:/c/Program Files/Git LFS:/c/Program Files (x86)/Subversion/bin:/c/Program Files/Microsoft SQL Server/120/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/110/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/Tools/Binn:/c/Program Files/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/Tools/Binn/ManagementStudio:/c/Tools/WebDriver:/c/Program Files (x86)/Microsoft SDKs/TypeScript/1.4:/c/Program Files (x86)/Microsoft Visual Studio 12.0/Common7/IDE/PrivateAssemblies:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI/wbin:/c/Ruby193/bin:/c/Tools/NUnit/bin:/c/Tools/xUnit:/c/Tools/MSpec:/c/Tools/Coverity/bin:/c/Program Files (x86)/CMake/bin:/c/go/bin:/c/Program Files/Java/jdk1.8.0/bin:/c/Python27:/c/Program Files/nodejs:/c/Program Files (x86)/iojs:/c/Program Files/iojs:/c/Users/appveyor/AppData/Roaming/npm:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/MSBuild/14.0/Bin:/c/Tools/NuGet:/c/Program Files (x86)/Microsoft Visual Studio 14.0/Common7/IDE/CommonExtensions/Microsoft/TestWindow:/c/Program Files/Microsoft DNX/Dnvm:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Apache/Maven/bin:/c/Python27/Scripts:/c/Tools/NUnit3:/c/Program Files/Mercurial:/c/Program Files/LLVM/bin:/c/Program Files/dotnet:/c/Tools/curl/bin:/c/Program Files/Amazon/AWSCLI:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft Visual Studio 14.0/Common7/IDE/Extensions/Microsoft/SQLDB/DAC/140:/c/Tools/vcpkg:/c/Program Files (x86)/Microsoft SQL Server/140/Tools/Binn:/c/Program Files/Microsoft SQL Server/140/Tools/Binn:/c/Program Files/Microsoft SQL Server/140/DTS/Binn:/c/Program Files/PowerShell/6.0.0:/c/Program Files/erl9.2/bin:/c/Program Files (x86)/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/usr/bin:/c/Program Files (x86)/Yarn/bin:/c/Program Files (x86)/NSIS:/c/Tools/Octopus:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/ProgramData/chocolatey/bin:/c/Users/appveyor/AppData/Roaming/npm:/c/Users/appveyor/AppData/Local/Yarn/bin:/c/Program Files/AppVeyor/BuildAgent:/c/projects/rust:/c/projects/rust/handle" 'C:\projects\rust\build\i686-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /c/projects/rust/build/i686-pc-windows-gnu/test/run-make/cross-lang-lto.stage2-i686-pc-windows-gnu -L /c/projects/rust/build/i686-pc-windows-gnu/test/run-make/cross-lang-lto.stage2-i686-pc-windows-gnu  lib.rs -Copt-level=2 -Z cross-lang-lto -Ccodegen-units=1 --crate-type=staticlib
[ "$(llvm-objdump -section-headers /c/projects/rust/build/i686-pc-windows-gnu/test/run-make/cross-lang-lto.stage2-i686-pc-windows-gnu/liblib.a | grep -c \\.llvmbc)" -ne "0" ]
make[1]: Leaving directory '/c/projects/rust/src/test/run-make/cross-lang-lto'
------------------------------------------
stderr:
------------------------------------------
C:\projects\rust\build\i686-pc-windows-gnu\llvm\build\bin\llvm-objdump.exe: 'C:/projects/rust/build/i686-pc-windows-gnu/test/run-make/cross-lang-lto.stage2-i686-pc-windows-gnu/liblib.a': No such file or directory
make[1]: *** [Makefile:28: staticlib] Error 1
------------------------------------------
thread '[run-make] run-make\cross-lang-lto' panicked at 'explicit panic', tools\compiletest\src\runtest.rs:2936:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-make] run-make\cross-lang-lto
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
thread 'main' panicked at 'Some tests failed', tools\compiletest\src\main.rs:488:22
