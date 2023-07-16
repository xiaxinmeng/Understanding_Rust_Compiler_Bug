
---- [debuginfo-gdb] debuginfo-gdb\basic-types-globals-metadata.rs stdout ----
	NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001
error: line not found in debugger output: type = bool
status: exit code: 0
command: PATH="C:\projects\rust\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;C:\projects\rust\build\i686-pc-windows-gnu\stage2-tools\i686-pc-windows-gnu\release\deps;C:\projects\rust\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\projects\rust\mingw32\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\120;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Amazon\AWSCLI;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\ProgramData\chocolatey\bin;C:\Program Files\Mercurial;C:\Program Files (x86)\Yarn\bin;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Program Files (x86)\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\Tools\curl\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Users\appveyor\AppData\Local\Yarn\.bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" C:\projects\rust\mingw32\bin\gdb -quiet -batch -nx -command=C:\projects\rust\build\i686-pc-windows-gnu\test\debuginfo\basic-types-globals-metadata.debugger.script
stdout:
------------------------------------------
GNU gdb (GDB) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "i686-w64-mingw32".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x40157c: file C:\projects\rust\src/test\debuginfo/basic-types-globals-metadata.rs, line 81.
[New Thread 2700.0x244]
Exception condition detected on fd 0
error detected on stdin
A debugging session is active.
	Inferior 1 [process 2700] will be killed.
Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------
Warning: C:projectsrust./src/etc: No such file or directory.
warning: SHIMVIEW: ShimInfo(Complete)
Failed to resume program execution (ContinueDebugEvent failed, error 87)
