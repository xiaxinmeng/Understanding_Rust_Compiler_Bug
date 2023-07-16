
[00:57:39] failures:
[00:57:39]
[00:57:39] ---- [codegen] codegen\stack-probes.rs stdout ----
[00:57:39]
[00:57:39] error: verification with 'FileCheck' failed
[00:57:39] status: exit code: 1
[00:57:39] command: PATH=";C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\x86_64-pc-window
s-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps;C:\projects\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw64\bin;C:\msys64\usr\bin;C:
\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:
\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\B
inn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Vi
sual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Progra
m Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0
\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program
Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Mi
crosoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Prog
ram Files\Amazon\AWSCLI;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\P
rogramData\chocolatey\bin;C:\Program Files (x86)\nodejs;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager
;C:\Users\appveyor\AppData\Roaming\npm;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" C:\projects\rust\build\x86_64-pc-windows-msvc\llvm\build\bin\FileCheck.exe -inp
ut-file=C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\stack-probes.ll C:\projects\rust\src/test\codegen\stack-probes.rs
[00:57:39] stdout:
[00:57:39] ------------------------------------------
[00:57:39]
[00:57:39] ------------------------------------------
[00:57:39] stderr:
[00:57:39] ------------------------------------------
[00:57:39] C:\projects\rust\src/test\codegen\stack-probes.rs:22:11: error: expected string not found in input
[00:57:39] // CHECK: attributes #0 = { uwtable "probe-stack"="__rust_probestack" }
[00:57:39]           ^
[00:57:39] C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\stack-probes.ll:7:36: note: scanning from here
[00:57:39] define void @foo() unnamed_addr #0 {
[00:57:39]                                    ^
[00:57:39] C:\projects\rust\build\x86_64-pc-windows-msvc\test\codegen\stack-probes.ll:12:1: note: possible intended match here
[00:57:39] attributes #0 = { uwtable }
[00:57:39] ^
[00:57:39]
[00:57:39] ------------------------------------------
[00:57:39]
[00:57:39] thread '[codegen] codegen\stack-probes.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2479
[00:57:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:39]
[00:57:39]
[00:57:39] failures:
[00:57:39]     [codegen] codegen\stack-probes.rs
[00:57:39]
[00:57:39] test result: FAILED. 37 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
