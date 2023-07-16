plain
2020-01-22T23:30:51.3045489Z test [debuginfo-gdb] debuginfo\var-captured-in-stack-closure.rs ... FAILED
2020-01-22T23:30:51.3054446Z 
2020-01-22T23:30:51.3054920Z failures:
2020-01-22T23:30:51.3058378Z 
2020-01-22T23:30:51.3058682Z ---- [debuginfo-gdb] debuginfo\associated-types.rs stdout ----
2020-01-22T23:30:51.3058961Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3059167Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3059476Z error: gdb failed to execute
2020-01-22T23:30:51.3059768Z status: exit code: 1
2020-01-22T23:30:51.3059768Z status: exit code: 1
2020-01-22T23:30:51.3061581Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\associated-types.gdb\\associated-types.debugger.script"
2020-01-22T23:30:51.3063226Z ------------------------------------------
2020-01-22T23:30:51.3063226Z ------------------------------------------
2020-01-22T23:30:51.3063408Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3063598Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3063788Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3064106Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3064291Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3064491Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3064682Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3064860Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3065096Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3065271Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3065480Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3065679Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3065815Z 
2020-01-22T23:30:51.3065978Z For help, type "help".
2020-01-22T23:30:51.3066152Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3066502Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3066779Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3067068Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3067899Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3068186Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3068428Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3068625Z [New Thread 5816.0x1508]
2020-01-22T23:30:51.3068842Z [New Thread 5816.0x580]
2020-01-22T23:30:51.3069035Z [Thread 5816.0x580 exited with code 0]
2020-01-22T23:30:51.3069198Z [Thread 5816.0x1508 exited with code 0]
2020-01-22T23:30:51.3069442Z [Inferior 1 (process 5816) exited normally]
2020-01-22T23:30:51.3069885Z ------------------------------------------
2020-01-22T23:30:51.3070188Z stderr:
2020-01-22T23:30:51.3070424Z ------------------------------------------
2020-01-22T23:30:51.3070424Z ------------------------------------------
2020-01-22T23:30:51.3070676Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3070920Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3071175Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3071418Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3071694Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3071922Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3072096Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3072394Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\associated-types.gdb\associated-types.debugger.script:15: Error in sourced command file:
2020-01-22T23:30:51.3072660Z No symbol 'arg' in current context
2020-01-22T23:30:51.3073185Z ------------------------------------------
2020-01-22T23:30:51.3073350Z 
2020-01-22T23:30:51.3073519Z 
2020-01-22T23:30:51.3073720Z ---- [debuginfo-gdb] debuginfo\borrowed-basic.rs stdout ----
2020-01-22T23:30:51.3073720Z ---- [debuginfo-gdb] debuginfo\borrowed-basic.rs stdout ----
2020-01-22T23:30:51.3074020Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3074273Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3074591Z error: gdb failed to execute
2020-01-22T23:30:51.3074766Z status: exit code: 1
2020-01-22T23:30:51.3074766Z status: exit code: 1
2020-01-22T23:30:51.3081082Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-basic.gdb\\borrowed-basic.debugger.script"
2020-01-22T23:30:51.3083198Z ------------------------------------------
2020-01-22T23:30:51.3083198Z ------------------------------------------
2020-01-22T23:30:51.3083329Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3083436Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3083625Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3083728Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3083845Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3083936Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3084040Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3084139Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3084240Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3084316Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3084423Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3084536Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3084588Z 
2020-01-22T23:30:51.3084658Z For help, type "help".
2020-01-22T23:30:51.3084766Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3084905Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3085005Z [Thread 960.0xdf0 exited with code 0]
2020-01-22T23:30:51.3085095Z [New Thread 960.0x14dc]
2020-01-22T23:30:51.3085169Z [Inferior 1 (process 960) exited normally]
2020-01-22T23:30:51.3085293Z ------------------------------------------
2020-01-22T23:30:51.3085525Z stderr:
2020-01-22T23:30:51.3085588Z ------------------------------------------
2020-01-22T23:30:51.3085588Z ------------------------------------------
2020-01-22T23:30:51.3085690Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3085777Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3085915Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-basic.gdb\borrowed-basic.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3086107Z No symbol 'bool_ref' in current context
2020-01-22T23:30:51.3086222Z ------------------------------------------
2020-01-22T23:30:51.3086289Z 
2020-01-22T23:30:51.3086322Z 
2020-01-22T23:30:51.3086415Z ---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
2020-01-22T23:30:51.3086415Z ---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
2020-01-22T23:30:51.3086512Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3086619Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3086732Z error: gdb failed to execute
2020-01-22T23:30:51.3086826Z status: exit code: 1
2020-01-22T23:30:51.3086826Z status: exit code: 1
2020-01-22T23:30:51.3088884Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-c-style-enum.gdb\\borrowed-c-style-enum.debugger.script"
2020-01-22T23:30:51.3090303Z ------------------------------------------
2020-01-22T23:30:51.3090303Z ------------------------------------------
2020-01-22T23:30:51.3090394Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3090563Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3090672Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3090793Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3090887Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3090990Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3091122Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3091224Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3091306Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3091397Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3091484Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3091592Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3091643Z 
2020-01-22T23:30:51.3091725Z For help, type "help".
2020-01-22T23:30:51.3091816Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3091983Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3092106Z [New Thread 5636.0x17d8]
2020-01-22T23:30:51.3092178Z [New Thread 5636.0x1238]
2020-01-22T23:30:51.3092263Z [Thread 5636.0x17d8 exited with code 0]
2020-01-22T23:30:51.3092335Z [Thread 5636.0x1238 exited with code 0]
2020-01-22T23:30:51.3092438Z [Inferior 1 (process 5636) exited normally]
2020-01-22T23:30:51.3092564Z ------------------------------------------
2020-01-22T23:30:51.3092635Z stderr:
2020-01-22T23:30:51.3092711Z ------------------------------------------
2020-01-22T23:30:51.3092711Z ------------------------------------------
2020-01-22T23:30:51.3092795Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3092893Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3093019Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-c-style-enum.gdb\borrowed-c-style-enum.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3093161Z No symbol 'the_a_ref' in current context
2020-01-22T23:30:51.3108585Z ------------------------------------------
2020-01-22T23:30:51.3108633Z 
2020-01-22T23:30:51.3108686Z 
2020-01-22T23:30:51.3108761Z ---- [debuginfo-gdb] debuginfo\borrowed-enum.rs stdout ----
2020-01-22T23:30:51.3108761Z ---- [debuginfo-gdb] debuginfo\borrowed-enum.rs stdout ----
2020-01-22T23:30:51.3108889Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3108985Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3109115Z error: gdb failed to execute
2020-01-22T23:30:51.3109202Z status: exit code: 1
2020-01-22T23:30:51.3109202Z status: exit code: 1
2020-01-22T23:30:51.3112039Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-enum.gdb\\borrowed-enum.debugger.script"
2020-01-22T23:30:51.3113591Z ------------------------------------------
2020-01-22T23:30:51.3113591Z ------------------------------------------
2020-01-22T23:30:51.3113665Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3113763Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3113872Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3113976Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3114087Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3114183Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3114283Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3114371Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3114469Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3114546Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3114652Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3114771Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3114822Z 
2020-01-22T23:30:51.3114882Z For help, type "help".
2020-01-22T23:30:51.3114982Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3115108Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3115206Z [New Thread 656.0x15e0]
2020-01-22T23:30:51.3115295Z [Thread 656.0x374 exited with code 0]
2020-01-22T23:30:51.3115372Z [Inferior 1 (process 656) exited normally]
2020-01-22T23:30:51.3115503Z ------------------------------------------
2020-01-22T23:30:51.3115591Z stderr:
2020-01-22T23:30:51.3115654Z ------------------------------------------
2020-01-22T23:30:51.3115654Z ------------------------------------------
2020-01-22T23:30:51.3115755Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3115841Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3115987Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-enum.gdb\borrowed-enum.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3116096Z No symbol 'the_a_ref' in current context
2020-01-22T23:30:51.3116224Z ------------------------------------------
2020-01-22T23:30:51.3116286Z 
2020-01-22T23:30:51.3116319Z 
2020-01-22T23:30:51.3116392Z ---- [debuginfo-gdb] debuginfo\borrowed-struct.rs stdout ----
2020-01-22T23:30:51.3116392Z ---- [debuginfo-gdb] debuginfo\borrowed-struct.rs stdout ----
2020-01-22T23:30:51.3116501Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3116609Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3116772Z error: gdb failed to execute
2020-01-22T23:30:51.3117004Z status: exit code: 1
2020-01-22T23:30:51.3117004Z status: exit code: 1
2020-01-22T23:30:51.3118545Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-struct.gdb\\borrowed-struct.debugger.script"
2020-01-22T23:30:51.3120000Z ------------------------------------------
2020-01-22T23:30:51.3120000Z ------------------------------------------
2020-01-22T23:30:51.3120091Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3120170Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3120280Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3120395Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3120490Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3120599Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3120687Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3120790Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3120872Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3120962Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3121049Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3139078Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3139212Z 
2020-01-22T23:30:51.3139292Z For help, type "help".
2020-01-22T23:30:51.3139371Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3139489Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3139592Z [New Thread 3660.0x370]
2020-01-22T23:30:51.3139657Z [New Thread 3660.0x1654]
2020-01-22T23:30:51.3139788Z [Thread 3660.0x1654 exited with code 0]
2020-01-22T23:30:51.3139859Z [Thread 3660.0x370 exited with code 0]
2020-01-22T23:30:51.3139948Z [Inferior 1 (process 3660) exited normally]
2020-01-22T23:30:51.3140065Z ------------------------------------------
2020-01-22T23:30:51.3140132Z stderr:
2020-01-22T23:30:51.3140206Z ------------------------------------------
2020-01-22T23:30:51.3140206Z ------------------------------------------
2020-01-22T23:30:51.3140284Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3140379Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3140498Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-struct.gdb\borrowed-struct.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3140614Z No symbol 'stack_val_ref' in current context
2020-01-22T23:30:51.3140734Z ------------------------------------------
2020-01-22T23:30:51.3140777Z 
2020-01-22T23:30:51.3140808Z 
2020-01-22T23:30:51.3140901Z ---- [debuginfo-gdb] debuginfo\borrowed-tuple.rs stdout ----
2020-01-22T23:30:51.3140901Z ---- [debuginfo-gdb] debuginfo\borrowed-tuple.rs stdout ----
2020-01-22T23:30:51.3141001Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3141087Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3141203Z error: gdb failed to execute
2020-01-22T23:30:51.3141267Z status: exit code: 1
2020-01-22T23:30:51.3141267Z status: exit code: 1
2020-01-22T23:30:51.3142709Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-tuple.gdb\\borrowed-tuple.debugger.script"
2020-01-22T23:30:51.3144033Z ------------------------------------------
2020-01-22T23:30:51.3144033Z ------------------------------------------
2020-01-22T23:30:51.3144102Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3144187Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3144275Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3144384Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3144490Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3144571Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3144665Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3144745Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3144836Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3144906Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3145007Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3145092Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3145152Z 
2020-01-22T23:30:51.3145207Z For help, type "help".
2020-01-22T23:30:51.3145303Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3145933Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3146041Z [New Thread 4312.0x4d0]
2020-01-22T23:30:51.3146132Z [New Thread 4312.0x138c]
2020-01-22T23:30:51.3146198Z [Thread 4312.0x13b4 exited with code 0]
2020-01-22T23:30:51.3146281Z [Thread 4312.0x138c exited with code 0]
2020-01-22T23:30:51.3146354Z [Inferior 1 (process 4312) exited normally]
2020-01-22T23:30:51.3146471Z ------------------------------------------
2020-01-22T23:30:51.3146551Z stderr:
2020-01-22T23:30:51.3146609Z ------------------------------------------
2020-01-22T23:30:51.3146609Z ------------------------------------------
2020-01-22T23:30:51.3146716Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3146799Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3146926Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-tuple.gdb\borrowed-tuple.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3147029Z No symbol 'stack_val_ref' in current context
2020-01-22T23:30:51.3166139Z ------------------------------------------
2020-01-22T23:30:51.3166190Z 
2020-01-22T23:30:51.3166266Z 
2020-01-22T23:30:51.3166345Z ---- [debuginfo-gdb] debuginfo\borrowed-unique-basic.rs stdout ----
2020-01-22T23:30:51.3166345Z ---- [debuginfo-gdb] debuginfo\borrowed-unique-basic.rs stdout ----
2020-01-22T23:30:51.3166460Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3166556Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3166683Z error: gdb failed to execute
2020-01-22T23:30:51.3166770Z status: exit code: 1
2020-01-22T23:30:51.3166770Z status: exit code: 1
2020-01-22T23:30:51.3168442Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-unique-basic.gdb\\borrowed-unique-basic.debugger.script"
2020-01-22T23:30:51.3169904Z ------------------------------------------
2020-01-22T23:30:51.3169904Z ------------------------------------------
2020-01-22T23:30:51.3169995Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3170079Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3170199Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3170417Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3170520Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3170615Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3170697Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3170790Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3170865Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3170958Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3171041Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3171141Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3171187Z 
2020-01-22T23:30:51.3171257Z For help, type "help".
2020-01-22T23:30:51.3171336Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3171460Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3171549Z [New Thread 1748.0x9bc]
2020-01-22T23:30:51.3171628Z [New Thread 1748.0x174c]
2020-01-22T23:30:51.3171695Z [Thread 1748.0x174c exited with code 0]
2020-01-22T23:30:51.3171777Z [Thread 1748.0x9bc exited with code 0]
2020-01-22T23:30:51.3171860Z [Inferior 1 (process 1748) exited normally]
2020-01-22T23:30:51.3171963Z ------------------------------------------
2020-01-22T23:30:51.3172093Z stderr:
2020-01-22T23:30:51.3172152Z ------------------------------------------
2020-01-22T23:30:51.3172152Z ------------------------------------------
2020-01-22T23:30:51.3172244Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3172337Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3172453Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-unique-basic.gdb\borrowed-unique-basic.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3172607Z No symbol 'bool_ref' in current context
2020-01-22T23:30:51.3172727Z ------------------------------------------
2020-01-22T23:30:51.3172770Z 
2020-01-22T23:30:51.3172801Z 
2020-01-22T23:30:51.3172878Z ---- [debuginfo-gdb] debuginfo\box.rs stdout ----
2020-01-22T23:30:51.3172878Z ---- [debuginfo-gdb] debuginfo\box.rs stdout ----
2020-01-22T23:30:51.3172961Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3173060Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3173176Z error: gdb failed to execute
2020-01-22T23:30:51.3173250Z status: exit code: 1
2020-01-22T23:30:51.3173250Z status: exit code: 1
2020-01-22T23:30:51.3174629Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\box.gdb\\box.debugger.script"
2020-01-22T23:30:51.3175917Z ------------------------------------------
2020-01-22T23:30:51.3175917Z ------------------------------------------
2020-01-22T23:30:51.3175986Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3176077Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3176203Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3176313Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3176400Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3176500Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3176594Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3176715Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3176807Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3176879Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3176973Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3177058Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3177117Z 
2020-01-22T23:30:51.3177172Z For help, type "help".
2020-01-22T23:30:51.3177265Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3177376Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3177479Z [New Thread 824.0x17e0]
2020-01-22T23:30:51.3177544Z [New Thread 824.0x1544]
2020-01-22T23:30:51.3177625Z [Thread 824.0x8c4 exited with code 0]
2020-01-22T23:30:51.3177695Z [Thread 824.0x1544 exited with code 0]
2020-01-22T23:30:51.3177778Z [Inferior 1 (process 824) exited normally]
2020-01-22T23:30:51.3177900Z ------------------------------------------
2020-01-22T23:30:51.3177966Z stderr:
2020-01-22T23:30:51.3178040Z ------------------------------------------
2020-01-22T23:30:51.3178040Z ------------------------------------------
2020-01-22T23:30:51.3178116Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3178212Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3178331Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\box.gdb\box.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3178429Z No symbol 'a' in current context
2020-01-22T23:30:51.3178545Z ------------------------------------------
2020-01-22T23:30:51.3178591Z 
2020-01-22T23:30:51.3178637Z 
2020-01-22T23:30:51.3178705Z ---- [debuginfo-gdb] debuginfo\boxed-struct.rs stdout ----
2020-01-22T23:30:51.3178705Z ---- [debuginfo-gdb] debuginfo\boxed-struct.rs stdout ----
2020-01-22T23:30:51.3178808Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3178894Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3179022Z error: gdb failed to execute
2020-01-22T23:30:51.3179100Z status: exit code: 1
2020-01-22T23:30:51.3179100Z status: exit code: 1
2020-01-22T23:30:51.3180511Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\boxed-struct.gdb\\boxed-struct.debugger.script"
2020-01-22T23:30:51.3181822Z ------------------------------------------
2020-01-22T23:30:51.3181822Z ------------------------------------------
2020-01-22T23:30:51.3181898Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3181985Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3182089Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3182183Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3182287Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3182369Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3182466Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3182553Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3182650Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3182736Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3182818Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3182918Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3182964Z 
2020-01-22T23:30:51.3183026Z For help, type "help".
2020-01-22T23:30:51.3183120Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3183237Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3183326Z [New Thread 2316.0xfc8]
2020-01-22T23:30:51.3183406Z [New Thread 2316.0x15c4]
2020-01-22T23:30:51.3183471Z [Thread 2316.0x15c4 exited with code 0]
2020-01-22T23:30:51.3183552Z [Thread 2316.0xfc8 exited with code 0]
2020-01-22T23:30:51.3183631Z [Inferior 1 (process 2316) exited normally]
2020-01-22T23:30:51.3183748Z ------------------------------------------
2020-01-22T23:30:51.3183828Z stderr:
2020-01-22T23:30:51.3183888Z ------------------------------------------
2020-01-22T23:30:51.3183888Z ------------------------------------------
2020-01-22T23:30:51.3183979Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3184057Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3184187Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\boxed-struct.gdb\boxed-struct.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3184310Z No symbol 'boxed_with_padding' in current context
2020-01-22T23:30:51.3184414Z ------------------------------------------
2020-01-22T23:30:51.3184471Z 
2020-01-22T23:30:51.3184502Z 
2020-01-22T23:30:51.3184598Z ---- [debuginfo-gdb] debuginfo\by-value-self-argument-in-trait-impl.rs stdout ----
2020-01-22T23:30:51.3184598Z ---- [debuginfo-gdb] debuginfo\by-value-self-argument-in-trait-impl.rs stdout ----
2020-01-22T23:30:51.3184692Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3184836Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3184956Z error: gdb failed to execute
2020-01-22T23:30:51.3185021Z status: exit code: 1
2020-01-22T23:30:51.3185021Z status: exit code: 1
2020-01-22T23:30:51.3187282Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\by-value-self-argument-in-trait-impl.gdb\\by-value-self-argument-in-trait-impl.debugger.script"
2020-01-22T23:30:51.3188700Z ------------------------------------------
2020-01-22T23:30:51.3188700Z ------------------------------------------
2020-01-22T23:30:51.3188791Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3188888Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3188983Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3189102Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3189203Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3189308Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3189393Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3189496Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3189577Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3189669Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3189772Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3190158Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3190209Z 
2020-01-22T23:30:51.3190283Z For help, type "help".
2020-01-22T23:30:51.3190386Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3190496Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3190687Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3190813Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3190922Z [New Thread 1644.0x145c]
2020-01-22T23:30:51.3190991Z [New Thread 1644.0x141c]
2020-01-22T23:30:51.3191079Z [Thread 1644.0x1430 exited with code 0]
2020-01-22T23:30:51.3191169Z [Thread 1644.0x141c exited with code 0]
2020-01-22T23:30:51.3191246Z [Inferior 1 (process 1644) exited normally]
2020-01-22T23:30:51.3191382Z ------------------------------------------
2020-01-22T23:30:51.3191454Z stderr:
2020-01-22T23:30:51.3191533Z ------------------------------------------
2020-01-22T23:30:51.3191533Z ------------------------------------------
2020-01-22T23:30:51.3191634Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3191722Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3191825Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3191915Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3192073Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\by-value-self-argument-in-trait-impl.gdb\by-value-self-argument-in-trait-impl.debugger.script:12: Error in sourced command file:
2020-01-22T23:30:51.3192195Z No symbol 'self' in current context
2020-01-22T23:30:51.3192321Z ------------------------------------------
2020-01-22T23:30:51.3192384Z 
2020-01-22T23:30:51.3192417Z 
2020-01-22T23:30:51.3192492Z ---- [debuginfo-gdb] debuginfo\c-style-enum-in-composite.rs stdout ----
2020-01-22T23:30:51.3192492Z ---- [debuginfo-gdb] debuginfo\c-style-enum-in-composite.rs stdout ----
2020-01-22T23:30:51.3192614Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3192723Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3192836Z error: gdb failed to execute
2020-01-22T23:30:51.3192924Z status: exit code: 1
2020-01-22T23:30:51.3192924Z status: exit code: 1
2020-01-22T23:30:51.3194465Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\c-style-enum-in-composite.gdb\\c-style-enum-in-composite.debugger.script"
2020-01-22T23:30:51.3195989Z ------------------------------------------
2020-01-22T23:30:51.3195989Z ------------------------------------------
2020-01-22T23:30:51.3196071Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3196145Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3196255Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3196364Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3196453Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3196548Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3196627Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3196720Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3196797Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3196888Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3196974Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3197076Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3197122Z 
2020-01-22T23:30:51.3197191Z For help, type "help".
2020-01-22T23:30:51.3197267Z Type "apropos word" to search for commands related to "word".
2020-01-22T23:30:51.3197390Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-22T23:30:51.3197493Z [New Thread 1300.0xfa4]
2020-01-22T23:30:51.3197558Z [New Thread 1300.0x16b8]
2020-01-22T23:30:51.3197640Z [Thread 1300.0x16b8 exited with code 0]
2020-01-22T23:30:51.3197708Z [Thread 1300.0xfa4 exited with code 0]
2020-01-22T23:30:51.3197795Z [Inferior 1 (process 1300) exited normally]
2020-01-22T23:30:51.3197914Z ------------------------------------------
2020-01-22T23:30:51.3197980Z stderr:
2020-01-22T23:30:51.3198051Z ------------------------------------------
2020-01-22T23:30:51.3198051Z ------------------------------------------
2020-01-22T23:30:51.3198135Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-22T23:30:51.3198231Z No symbol table is loaded.  Use the "file" command.
2020-01-22T23:30:51.3198349Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\c-style-enum-in-composite.gdb\c-style-enum-in-composite.debugger.script:10: Error in sourced command file:
2020-01-22T23:30:51.3198478Z No symbol 'tuple_interior_padding' in current context
2020-01-22T23:30:51.3198607Z ------------------------------------------
2020-01-22T23:30:51.3198649Z 
2020-01-22T23:30:51.3198680Z 
2020-01-22T23:30:51.3198767Z ---- [debuginfo-gdb] debuginfo\closure-in-generic-function.rs stdout ----
2020-01-22T23:30:51.3198767Z ---- [debuginfo-gdb] debuginfo\closure-in-generic-function.rs stdout ----
2020-01-22T23:30:51.3198874Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-22T23:30:51.3198961Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-22T23:30:51.3199076Z error: gdb failed to execute
2020-01-22T23:30:51.3199189Z status: exit code: 1
2020-01-22T23:30:51.3199189Z status: exit code: 1
2020-01-22T23:30:51.3200620Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.3\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\closure-in-generic-function.gdb\\closure-in-generic-function.debugger.script"
2020-01-22T23:30:51.3201902Z ------------------------------------------
2020-01-22T23:30:51.3201902Z ------------------------------------------
2020-01-22T23:30:51.3201970Z GNU gdb (GDB) 8.3.1
2020-01-22T23:30:51.3202070Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-22T23:30:51.3202157Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-22T23:30:51.3202265Z This is free software: you are free to change and redistribute it.
2020-01-22T23:30:51.3202367Z There is NO WARRANTY, to the extent permitted by law.
2020-01-22T23:30:51.3202448Z Type "show copying" and "show warranty" for details.
2020-01-22T23:30:51.3202549Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-22T23:30:51.3202629Z Type "show configuration" for configuration details.
2020-01-22T23:30:51.3202722Z For bug reporting instructions, please see:
2020-01-22T23:30:51.3202793Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-22T23:30:51.3202889Z Find the GDB manual and other documentation resources online at:
2020-01-22T23:30:51.3202975Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-22T23:30:51.3203038Z 
2020-01-22T23:30:51.3203094Z For help, type "help".
2020-01-22T23:30:51.3203228Z Type "apropos word" to search for commands related to "word".
---
2020-01-22T23:30:51.3608907Z test result: FAILED. 111 passed; 71 failed; 50 ignored; 0 measured; 0 filtered out
2020-01-22T23:30:51.3608966Z 
2020-01-22T23:30:51.3608994Z 
2020-01-22T23:30:51.3609021Z 
2020-01-22T23:30:51.3609627Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\debuginfo" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-22T23:30:51.3610241Z 
2020-01-22T23:30:51.3610270Z 
2020-01-22T23:30:51.3610270Z 
2020-01-22T23:30:51.3883246Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2020-01-22T23:30:51.3883607Z Build completed unsuccessfully in 1:14:01
2020-01-22T23:30:51.4721949Z make: *** [Makefile:80: ci-subset-1] Error 1
2020-01-22T23:30:51.5466254Z   local time: Wed Jan 22 23:30:51 CUT 2020
2020-01-22T23:30:51.8532869Z   network time: Wed, 22 Jan 2020 23:30:51 GMT
2020-01-22T23:30:51.8546141Z == end clock drift check ==
2020-01-22T23:30:51.8707962Z 
2020-01-22T23:30:51.8707962Z 
2020-01-22T23:30:52.1437912Z ##[error]Bash exited with code '2'.
2020-01-22T23:30:52.1800663Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-22T23:30:52.2496053Z ==============================================================================
2020-01-22T23:30:52.2496181Z Task         : Get sources
2020-01-22T23:30:52.2496270Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
