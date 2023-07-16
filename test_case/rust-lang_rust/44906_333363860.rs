
failures:
---- [run-make] run-make\issue-26092 stdout ----
	
error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
PATH="/c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/issue-26092.stage2-x86_64-pc-windows-gnu:C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\bin:/c/projects/rust/build/x86_64-pc-windows-gnu/stage0-tools/x86_64-pc-windows-gnu/release/deps:/c/projects/rust/build/x86_64-pc-windows-gnu/stage0-sysroot/lib/rustlib/x86_64-pc-windows-gnu/lib:/c/Program Files (x86)/Inno Setup 5:/c/Python27:/c/projects/rust/mingw64/bin:/usr/bin:/c/Perl/site/bin:/c/Perl/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Program Files/7-Zip:/c/Program Files/Microsoft/Web Platform Installer:/c/Tools/GitVersion:/c/Tools/PsTools:/c/Program Files/Git LFS:/c/Program Files (x86)/Subversion/bin:/c/Program Files/Microsoft SQL Server/120/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/110/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/Tools/Binn:/c/Program Files/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/Tools/Binn/ManagementStudio:/c/Tools/WebDriver:/c/Program Files (x86)/Microsoft SDKs/TypeScript/1.4:/c/Program Files (x86)/Microsoft Visual Studio 12.0/Common7/IDE/PrivateAssemblies:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI/wbin:/c/Ruby193/bin:/c/Tools/NUnit/bin:/c/Tools/xUnit:/c/Tools/MSpec:/c/Tools/Coverity/bin:/c/Program Files (x86)/CMake/bin:/c/go/bin:/c/Program Files/Java/jdk1.8.0/bin:/c/Python27:/c/Program Files/nodejs:/c/Program Files (x86)/iojs:/c/Program Files/iojs:/c/Users/appveyor/AppData/Roaming/npm:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/MSBuild/14.0/Bin:/c/Tools/NuGet:/c/Program Files (x86)/Microsoft Visual Studio 14.0/Common7/IDE/CommonExtensions/Microsoft/TestWindow:/c/Program Files/Microsoft DNX/Dnvm:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Apache/Maven/bin:/c/Python27/Scripts:/c/Tools/NUnit3:/c/Program Files/Mercurial:/c/Program Files/LLVM/bin:/c/Program Files/dotnet:/c/Program Files/erl8.3/bin:/c/Tools/curl/bin:/c/Program Files/Amazon/AWSCLI:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft Visual Studio 14.0/Common7/IDE/Extensions/Microsoft/SQLDB/DAC/140:/c/Program Files (x86)/Yarn/bin:/c/Program Files/Git/cmd:/c/Program Files/Git/usr/bin:/c/ProgramData/chocolatey/bin:/c/Tools/vcpkg:/c/Program Files (x86)/nodejs:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Users/appveyor/AppData/Local/Yarn/bin:/c/Users/appveyor/AppData/Roaming/npm:/c/Program Files/AppVeyor/BuildAgent:/c/projects/rust:/c/projects/rust/handle" 'C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/issue-26092.stage2-x86_64-pc-windows-gnu -L /c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/issue-26092.stage2-x86_64-pc-windows-gnu  -o "" blank.rs 2>&1 | \
		grep -i 'No such file or directory'
------------------------------------------
stderr:
------------------------------------------
make: *** [Makefile:4: all] Error 1
------------------------------------------
thread '[run-make] run-make\issue-26092' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2433:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-make] run-make\issue-26092
test result: FAILED. 160 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
