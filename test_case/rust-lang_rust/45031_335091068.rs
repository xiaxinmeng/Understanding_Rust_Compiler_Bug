
[01:11:22] ---- [codegen] codegen\nounwind.rs stdout ----
[01:11:22] 	
[01:11:22] error: verification with 'FileCheck' failed
[01:11:22] status: exit code: 1
[01:11:22] command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-tools\x86_64-pc-windows-msvc\release\deps;C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-sysroot\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw64\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Tools\vcpkg;C:\Program Files (x86)\nodejs;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\nounwind.ll" "C:\\projects\\rust\\src/test\\codegen\\nounwind.rs"
[01:11:22] stdout:
[01:11:22] ------------------------------------------
[01:11:22] 
[01:11:22] ------------------------------------------
[01:11:22] stderr:
[01:11:22] ------------------------------------------
[01:11:22] C:\projects\rust\src/test\codegen\nounwind.rs:22:11: error: expected string not found in input
[01:11:22] // CHECK: @bar() unnamed_addr #0
[01:11:22]           ^
[01:11:22] C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\nounwind.ll:7:36: note: scanning from here
[01:11:22] define void @foo() unnamed_addr #0 {
[01:11:22]                                    ^
[01:11:22] C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\nounwind.ll:17:14: note: possible intended match here
[01:11:22] declare void @bar() unnamed_addr #1
[01:11:22]              ^
[01:11:22] 
[01:11:22] ------------------------------------------
[01:11:22] 
[01:11:22] thread '[codegen] codegen\nounwind.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2433:8
[01:11:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:22] 
[01:11:22] ---- [codegen] codegen\panic-abort-windows.rs stdout ----
[01:11:22] 	
[01:11:22] error: verification with 'FileCheck' failed
[01:11:22] status: exit code: 1
[01:11:22] command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-tools\x86_64-pc-windows-msvc\release\deps;C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-sysroot\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw64\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Tools\vcpkg;C:\Program Files (x86)\nodejs;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--input-file" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\codegen\\panic-abort-windows.ll" "C:\\projects\\rust\\src/test\\codegen\\panic-abort-windows.rs"
[01:11:22] stdout:
[01:11:22] ------------------------------------------
[01:11:22] 
[01:11:22] ------------------------------------------
[01:11:22] stderr:
[01:11:22] ------------------------------------------
[01:11:22] C:\projects\rust\src/test\codegen\panic-abort-windows.rs:31:11: error: expected string not found in input
[01:11:22] // CHECK: Function Attrs: uwtable
[01:11:22]           ^
[01:11:22] C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\panic-abort-windows.ll:1:1: note: scanning from here
[01:11:22] ; ModuleID = 'panic_abort_windows0-8cd878b7c8d78940dfe6697baf5b88ec.rs'
[01:11:22] ^
[01:11:22] C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\panic-abort-windows.ll:6:3: note: possible intended match here
[01:11:22] ; Function Attrs: nounwind uwtable
[01:11:22]   ^
[01:11:22] 
[01:11:22] ------------------------------------------
[01:11:22] 
[01:11:22] thread '[codegen] codegen\panic-abort-windows.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2433:8
[01:11:22] 
[01:11:22] 
[01:11:22] failures:
[01:11:22]     [codegen] codegen\nounwind.rs
[01:11:22]     [codegen] codegen\panic-abort-windows.rs
[01:11:22] 
[01:11:22] test result: FAILED. 42 passed; 2 failed; 9 ignored; 0 measured; 0 filtered out
[01:11:22] 
[01:11:22] thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:323:21
