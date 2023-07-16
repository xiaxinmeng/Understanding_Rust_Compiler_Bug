
[01:23:05] failures:
[01:23:05] 
[01:23:05] ---- [run-pass] run-pass\backtrace.rs stdout ----
[01:23:05] 	
[01:23:05] error: test run failed!
[01:23:05] status: exit code: 101
[01:23:05] command: PATH="C:\projects\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x86;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\i686-pc-windows-msvc\stage0-tools\i686-pc-windows-msvc\release\deps;C:\projects\rust\build\i686-pc-windows-msvc\stage0-sysroot\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Tools\vcpkg;C:\Program Files (x86)\nodejs;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\i686-pc-windows-msvc\\test\\run-pass\\backtrace.stage2-i686-pc-windows-msvc.exe"
[01:23:05] stdout:
[01:23:05] ------------------------------------------
[01:23:05] 
[01:23:05] ------------------------------------------
[01:23:05] stderr:
[01:23:05] ------------------------------------------
[01:23:05] thread 'main' panicked at 'bad output: thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace.rs:24:8
[01:23:05] stack backtrace:
[01:23:05]    0: std::ffi::c_str::CString::into_inner
[01:23:05] ', C:\projects\rust\src/test\run-pass\backtrace.rs:59:4
[01:23:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:23:05] 
[01:23:05] ------------------------------------------
[01:23:05] 
[01:23:05] thread '[run-pass] run-pass\backtrace.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2433:8
[01:23:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:23:05] 
[01:23:05] ---- [run-pass] run-pass\backtrace-debuginfo.rs stdout ----
[01:23:05] 	
[01:23:05] error: test run failed!
[01:23:05] status: exit code: 101
[01:23:05] command: PATH="C:\projects\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x86;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\i686-pc-windows-msvc\stage0-tools\i686-pc-windows-msvc\release\deps;C:\projects\rust\build\i686-pc-windows-msvc\stage0-sysroot\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Tools\vcpkg;C:\Program Files (x86)\nodejs;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\i686-pc-windows-msvc\\test\\run-pass\\backtrace-debuginfo.stage2-i686-pc-windows-msvc.exe"
[01:23:05] stdout:
[01:23:05] ------------------------------------------
[01:23:05] 
[01:23:05] ------------------------------------------
[01:23:05] stderr:
[01:23:05] ------------------------------------------
[01:23:05] thread 'main' panicked at 'trace does not match position list: test case 0
[01:23:05] thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:78:4
[01:23:05] stack backtrace:
[01:23:05]    0: 0x73e1131b - std::ffi::c_str::CString::into_inner::h65c9063b40555ca0
[01:23:05] 
[01:23:05] ---
[01:23:05] backtrace-debuginfo.rs:167
[01:23:05] ', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:133:4
[01:23:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:23:05] 
[01:23:05] ------------------------------------------
[01:23:05] 
[01:23:05] thread '[run-pass] run-pass\backtrace-debuginfo.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2433:8
[01:23:05] 
[01:23:05] 
[01:23:05] failures:
[01:23:05]     [run-pass] run-pass\backtrace-debuginfo.rs
[01:23:05]     [run-pass] run-pass\backtrace.rs
[01:23:05] 
[01:23:05] test result: FAILED. 2747 passed; 2 failed; 19 ignored; 0 measured; 0 filtered out
