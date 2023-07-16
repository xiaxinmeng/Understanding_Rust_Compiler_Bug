
failures:
---- [debuginfo-gdb] debuginfo\by-value-non-immediate-argument.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7010001
error: line not found in debugger output: $5 = {__0 = 7, __1 = 8, __2 = 9.5, __3 = 10.5}
status: exit code: 0
command: PATH="C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\lib\rustlib\x86_64-pc-windows-gnu\lib;C:\projects\rust\build\x86_64-pc-windows-gnu\llvm\build\bin;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\projects\rust\mingw64\bin;C:\msys64\usr\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\dotnet;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Tools\vcpkg;C:\Program Files (x86)\Microsoft SQL Server\140\Tools\Binn;C:\Program Files\Microsoft SQL Server\140\Tools\Binn;C:\Program Files\Microsoft SQL Server\140\DTS\Binn;C:\Program Files\erl9.2\bin;C:\Program Files (x86)\Yarn\bin;C:\Program Files (x86)\NSIS;C:\Tools\Octopus;C:\Program Files\PowerShell\6.0.2;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\LLVM\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Program Files (x86)\nodejs;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\.dotnet\tools;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo\\by-value-non-immediate-argument\\by-value-non-immediate-argument.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 7.10.1
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-w64-mingw32".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x40161f: file C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs, line 94.
Breakpoint 2 at 0x40165b: file C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs, line 98.
Breakpoint 3 at 0x401674: file C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs, line 102.
Breakpoint 4 at 0x401684: file C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs, line 108.
Breakpoint 5 at 0x401694: file C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs, line 120.
[New Thread 3308.0xe14]
Breakpoint 1, by_value_non_immediate_argument::fun::h5655355dea57288f (s=...) at C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs:94
94	    zzz(); // #break
$1 = {a = 1, b = 2.5}
Breakpoint 2, by_value_non_immediate_argument::fun_fun::hebfe7f6888071e13 () at C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs:98
98	    zzz(); // #break
$2 = {a = 3, b = 4.5}
$3 = 5
$4 = 6.5
Breakpoint 3, by_value_non_immediate_argument::tup::hb447d1a1503f7364 () at C:\projects\rust\src/test\debuginfo\by-value-non-immediate-argument.rs:102
102	    zzz(); // #break
------------------------------------------
stderr:
------------------------------------------
Warning: C:projectsrust./src/etc: No such file or directory.
warning: SHIMVIEW: ShimInfo(Complete)
C:\projects\rust\build\x86_64-pc-windows-gnu\test\debuginfo\by-value-non-immediate-argument\by-value-non-immediate-argument.debugger.script:19: Error in sourced command file:
No symbol "a" in current context.
------------------------------------------
thread '[debuginfo-gdb] debuginfo\by-value-non-immediate-argument.rs' panicked at 'explicit panic', tools\compiletest\src\runtest.rs:3149:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
---- [debuginfo-gdb] debuginfo\function-arg-initialization.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7010001
error: line not found in debugger output: $4 = {a = 3, b = 4, c = 5, d = 6, e = 7, f = 8, g = 9, h = 10}
status: exit code: 0
command: PATH="C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\lib\rustlib\x86_64-pc-windows-gnu\lib;C:\projects\rust\build\x86_64-pc-windows-gnu\llvm\build\bin;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\projects\rust\mingw64\bin;C:\msys64\usr\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\dotnet;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Tools\vcpkg;C:\Program Files (x86)\Microsoft SQL Server\140\Tools\Binn;C:\Program Files\Microsoft SQL Server\140\Tools\Binn;C:\Program Files\Microsoft SQL Server\140\DTS\Binn;C:\Program Files\erl9.2\bin;C:\Program Files (x86)\Yarn\bin;C:\Program Files (x86)\NSIS;C:\Tools\Octopus;C:\Program Files\PowerShell\6.0.2;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\LLVM\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Program Files (x86)\nodejs;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\.dotnet\tools;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo\\function-arg-initialization\\function-arg-initialization.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 7.10.1
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-w64-mingw32".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x401556: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 234.
Breakpoint 2 at 0x401574: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 249.
Breakpoint 3 at 0x4015b1: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 253.
Breakpoint 4 at 0x401621: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 258.
Breakpoint 5 at 0x401674: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 263.
Breakpoint 6 at 0x401693: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 267.
Breakpoint 7 at 0x4016b3: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 271.
Breakpoint 8 at 0x4016d4: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 275.
Breakpoint 9 at 0x401724: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 279.
Breakpoint 10 at 0x401793: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 287.
Breakpoint 11 at 0x401823: file C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs, line 295.
[New Thread 1120.0x1330]
Breakpoint 1, function_arg_initialization::immediate_args::h4b590aedb652992c (a=1, b=true, c=2.5) at C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs:234
234	    zzz(); // #break
$1 = 1
$2 = true
$3 = 2.5
Breakpoint 2, function_arg_initialization::non_immediate_args::hae8b42f5983dc5c0 () at C:\projects\rust\src/test\debuginfo\function-arg-initialization.rs:249
249	    zzz(); // #break
------------------------------------------
stderr:
------------------------------------------
Warning: C:projectsrust./src/etc: No such file or directory.
warning: SHIMVIEW: ShimInfo(Complete)
C:\projects\rust\build\x86_64-pc-windows-gnu\test\debuginfo\function-arg-initialization\function-arg-initialization.debugger.script:23: Error in sourced command file:
No symbol "a" in current context.
------------------------------------------
thread '[debuginfo-gdb] debuginfo\function-arg-initialization.rs' panicked at 'explicit panic', tools\compiletest\src\runtest.rs:3149:9
failures:
    [debuginfo-gdb] debuginfo\by-value-non-immediate-argument.rs
    [debuginfo-gdb] debuginfo\function-arg-initialization.rs
test result: FAILED. 81 passed; 2 failed; 26 ignored; 0 measured; 0 filtered out
thread 'main' panicked at 'Some tests failed', tools\compiletest\src\main.rs:497:22
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "--rustc-path" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "C:\\projects\\rust\\src/test\\debuginfo" "--build-base" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files (x86)\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27\\python.exe" "--lldb-python" "C:\\Python27\\python.exe" "--gdb" "C:\\projects\\rust\\mingw64\\bin\\gdb" "--lldb-version" "lldb version 6.0.0" "--lldb-python-dir" "<COULD NOT FIND PATH>" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
expected success, got: exit code: 101
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap test
