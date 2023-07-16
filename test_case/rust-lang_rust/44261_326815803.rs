
[01:05:18] failures:
[01:05:18] 
[01:05:18] ---- [debuginfo-gdb] debuginfo-gdb\boxed-struct.rs stdout ----
[01:05:18] 	NOTE: compiletest thinks it is using GDB without native rust support
[01:05:18] NOTE: compiletest thinks it is using GDB version 7010001
[01:05:18] 
[01:05:18] error: gdb failed to execute
[01:05:18] status: exit code: 1816
[01:05:18] command: PATH="C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\lib\rustlib\x86_64-pc-windows-gnu\lib;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0-tools\x86_64-pc-windows-gnu\release\deps;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0-sysroot\lib\rustlib\x86_64-pc-windows-gnu\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\projects\rust\mingw64\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\ProgramData\chocolatey\bin;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files (x86)\nodejs;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo\\boxed-struct.debugger.script"
[01:05:18] stdout:
[01:05:18] ------------------------------------------
[01:05:18] 
[01:05:18] ------------------------------------------
[01:05:18] stderr:
[01:05:18] ------------------------------------------
[01:05:18] 
[01:05:18] ------------------------------------------
[01:05:18] 
[01:05:18] thread '[debuginfo-gdb] debuginfo-gdb\boxed-struct.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2435:8
[01:05:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:18] 
[01:05:18] 
[01:05:18] failures:
[01:05:18]     [debuginfo-gdb] debuginfo-gdb\boxed-struct.rs
[01:05:18] 
[01:05:18] test result: FAILED. 96 passed; 1 failed; 11 ignored; 0 measured; 0 filtered out
[01:05:18] 
[01:05:18] thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:323:21
[01:05:18] 
[01:05:18] 
[01:05:18] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage0-tools\\x86_64-pc-windows-gnu\\release\\compiletest.exe" "--compile-lib-path" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "--rustc-path" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "C:\\projects\\rust\\src/test\\debuginfo" "--build-base" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files (x86)\\nodejs\\node" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27\\python.exe" "--lldb-python" "C:\\Python27\\python.exe" "--gdb" "C:\\projects\\rust\\mingw64\\bin\\gdb" "--llvm-version" "4.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:18] expected success, got: exit code: 101
