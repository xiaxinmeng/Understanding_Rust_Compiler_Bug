plain
2020-03-08T20:39:53.5702772Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/test/ui", "src/test/compile-fail"]
2020-03-08T20:39:53.5902668Z Check compiletest suite=codegen mode=codegen (i686-pc-windows-gnu -> i686-pc-windows-gnu)
2020-03-08T20:39:53.7811073Z 
2020-03-08T20:39:53.7811889Z running 178 tests
2020-03-08T20:40:00.0403649Z ..i...i................i..iiii..........ii.......................ii.......i........ii.....i........i 100/178
2020-03-08T20:40:04.1912134Z .....i.iii.iiii.iiiiiiiiiiiiiiii.........................ii.i.i.........i.....
2020-03-08T20:40:04.1913751Z 
2020-03-08T20:40:04.1924813Z  finished in 10.602
2020-03-08T20:40:04.1938152Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/test/ui", "src/test/compile-fail"]
2020-03-08T20:40:04.2141453Z Check compiletest suite=codegen-units mode=codegen-units (i686-pc-windows-gnu -> i686-pc-windows-gnu)
---
2020-03-08T20:40:09.8077253Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/test/ui", "src/test/compile-fail"]
2020-03-08T20:40:09.8281292Z Check compiletest suite=assembly mode=assembly (i686-pc-windows-gnu -> i686-pc-windows-gnu)
2020-03-08T20:40:09.9608986Z 
2020-03-08T20:40:09.9609890Z running 9 tests
2020-03-08T20:40:09.9610275Z iiiiiiiii
2020-03-08T20:40:09.9611170Z 
2020-03-08T20:40:09.9625998Z  finished in 0.134
2020-03-08T20:40:09.9632066Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/test/ui", "src/test/compile-fail"]
2020-03-08T20:40:09.9876773Z Check compiletest suite=incremental mode=incremental (i686-pc-windows-gnu -> i686-pc-windows-gnu)
---
2020-03-08T20:40:59.0501476Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/test/ui", "src/test/compile-fail"]
2020-03-08T20:40:59.0839053Z Check compiletest suite=debuginfo mode=debuginfo (i686-pc-windows-gnu -> i686-pc-windows-gnu)
2020-03-08T20:40:59.2759159Z 
2020-03-08T20:40:59.2760173Z running 115 tests
2020-03-08T20:41:33.6525125Z iiiiiFFFFFFFFiFFiFF.i.Fi.iiFFFiFFiFFFFFFFFFFFFiF.i..FFFF.FFFFiiiiFFFFF.FFFiFFiFiii..FFFFF.FiiFiFiFFF 100/115
2020-03-08T20:41:39.7357850Z F.FFiiFF.FFFiiF
2020-03-08T20:41:39.7705648Z 
2020-03-08T20:41:39.7706265Z ---- [debuginfo-gdb] debuginfo\associated-types.rs stdout ----
2020-03-08T20:41:39.7706265Z ---- [debuginfo-gdb] debuginfo\associated-types.rs stdout ----
2020-03-08T20:41:39.7706867Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7707245Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7707509Z 
2020-03-08T20:41:39.7708036Z error: line not found in debugger output: $1 = associated_types::Struct<i32> {b: -1, b1: 0}
2020-03-08T20:41:39.7709178Z status: exit code: 0
2020-03-08T20:41:39.7717617Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\associated-types.gdb\\associated-types.debugger.script"
2020-03-08T20:41:39.7727648Z ------------------------------------------
2020-03-08T20:41:39.7727648Z ------------------------------------------
2020-03-08T20:41:39.7727940Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7728204Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7728922Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7729665Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7730252Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7730781Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7731100Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7731845Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7732168Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7732514Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7733131Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7733498Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7733714Z 
2020-03-08T20:41:39.7735118Z For help, type "help".
2020-03-08T20:41:39.7735488Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7735936Z Breakpoint 1 at 0x40167e: file D:\a\1\s\src/test\debuginfo\associated-types.rs, line 111.
2020-03-08T20:41:39.7736522Z Breakpoint 2 at 0x40171e: file D:\a\1\s\src/test\debuginfo\associated-types.rs, line 118.
2020-03-08T20:41:39.7737035Z Breakpoint 3 at 0x401777: file D:\a\1\s\src/test\debuginfo\associated-types.rs, line 122.
2020-03-08T20:41:39.7738189Z Breakpoint 4 at 0x40180e: file D:\a\1\s\src/test\debuginfo\associated-types.rs, line 130.
2020-03-08T20:41:39.7738708Z Breakpoint 5 at 0x40188d: file D:\a\1\s\src/test\debuginfo\associated-types.rs, line 137.
2020-03-08T20:41:39.7739278Z Breakpoint 6 at 0x40186e: file D:\a\1\s\src/test\debuginfo\associated-types.rs, line 140.
2020-03-08T20:41:39.7739696Z [New Thread 5320.0xf6c]
2020-03-08T20:41:39.7739923Z [New Thread 5320.0x1768]
2020-03-08T20:41:39.7740201Z [New Thread 5320.0x1fc]
2020-03-08T20:41:39.7741592Z gdb: unknown target exception 0x4000001f at 0x40167e
2020-03-08T20:41:39.7741907Z 
2020-03-08T20:41:39.7742133Z Thread 1 received signal ?, Unknown signal.
2020-03-08T20:41:39.7742439Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7742884Z ------------------------------------------
2020-03-08T20:41:39.7743111Z stderr:
2020-03-08T20:41:39.7743396Z ------------------------------------------
2020-03-08T20:41:39.7743396Z ------------------------------------------
2020-03-08T20:41:39.7743711Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7745144Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7745706Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7746486Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7747624Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7748420Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7748913Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7749288Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7749713Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7750176Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7750994Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7751413Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7751776Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7752191Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7752526Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7752952Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7753356Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7753974Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7755261Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\associated-types.gdb\associated-types.debugger.script:15: Error in sourced command file:
2020-03-08T20:41:39.7755799Z No symbol 'arg' in current context
2020-03-08T20:41:39.7756238Z ------------------------------------------
2020-03-08T20:41:39.7759175Z 
2020-03-08T20:41:39.7759311Z 
2020-03-08T20:41:39.7759555Z ---- [debuginfo-gdb] debuginfo\borrowed-basic.rs stdout ----
2020-03-08T20:41:39.7759555Z ---- [debuginfo-gdb] debuginfo\borrowed-basic.rs stdout ----
2020-03-08T20:41:39.7759971Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7760382Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7761182Z error: line not found in debugger output: $1 = true
2020-03-08T20:41:39.7761520Z status: exit code: 0
2020-03-08T20:41:39.7761520Z status: exit code: 0
2020-03-08T20:41:39.7768919Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\borrowed-basic.gdb\\borrowed-basic.debugger.script"
2020-03-08T20:41:39.7775917Z ------------------------------------------
2020-03-08T20:41:39.7775917Z ------------------------------------------
2020-03-08T20:41:39.7776362Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7776615Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7777400Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7777885Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7778355Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7778823Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7779151Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7779518Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7779835Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7780174Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7780604Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7781179Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7782167Z 
2020-03-08T20:41:39.7782411Z For help, type "help".
2020-03-08T20:41:39.7782877Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7783557Z Breakpoint 1 at 0x40173d: file D:\a\1\s\src/test\debuginfo\borrowed-basic.rs, line 162.
2020-03-08T20:41:39.7785846Z [New Thread 1864.0x358]
2020-03-08T20:41:39.7786096Z [New Thread 1864.0x1718]
2020-03-08T20:41:39.7786374Z [New Thread 1864.0x179c]
2020-03-08T20:41:39.7786668Z gdb: unknown target exception 0x4000001f at 0x40173d
2020-03-08T20:41:39.7788138Z 
2020-03-08T20:41:39.7788402Z Thread 1 received signal ?, Unknown signal.
2020-03-08T20:41:39.7789662Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7791129Z ------------------------------------------
2020-03-08T20:41:39.7791437Z stderr:
2020-03-08T20:41:39.7791875Z ------------------------------------------
2020-03-08T20:41:39.7791875Z ------------------------------------------
2020-03-08T20:41:39.7792178Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7792564Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7793077Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7793797Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7795253Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7795781Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7796261Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7796638Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7797041Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7797473Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7797837Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7798223Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7798624Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7799033Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7799370Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7799782Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7800223Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7800731Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7801515Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\borrowed-basic.gdb\borrowed-basic.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.7802039Z No symbol 'bool_ref' in current context
2020-03-08T20:41:39.7802475Z ------------------------------------------
2020-03-08T20:41:39.7802654Z 
2020-03-08T20:41:39.7802831Z 
2020-03-08T20:41:39.7803074Z ---- [debuginfo-gdb] debuginfo\borrowed-enum.rs stdout ----
2020-03-08T20:41:39.7803074Z ---- [debuginfo-gdb] debuginfo\borrowed-enum.rs stdout ----
2020-03-08T20:41:39.7803481Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7803871Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7804141Z 
2020-03-08T20:41:39.7804650Z error: line not found in debugger output: $1 = borrowed_enum::ABC::TheA{x: 0, y: 8970181431921507452}
2020-03-08T20:41:39.7805167Z status: exit code: 0
2020-03-08T20:41:39.7812684Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\borrowed-enum.gdb\\borrowed-enum.debugger.script"
2020-03-08T20:41:39.7819568Z ------------------------------------------
2020-03-08T20:41:39.7819568Z ------------------------------------------
2020-03-08T20:41:39.7819852Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7820113Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7820550Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7820983Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7821388Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7821758Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7822065Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7822416Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7822744Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7823073Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7823397Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7823791Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7824000Z 
2020-03-08T20:41:39.7824212Z For help, type "help".
2020-03-08T20:41:39.7824496Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7825036Z Breakpoint 1 at 0x40171e: file D:\a\1\s\src/test\debuginfo\borrowed-enum.rs, line 71.
2020-03-08T20:41:39.7825391Z [New Thread 1584.0x1510]
2020-03-08T20:41:39.7825715Z gdb: unknown target exception 0x4000001f at 0x40171e
2020-03-08T20:41:39.7825919Z 
2020-03-08T20:41:39.7826173Z Program received signal ?, Unknown signal.
2020-03-08T20:41:39.7826410Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7826803Z ------------------------------------------
2020-03-08T20:41:39.7827070Z stderr:
2020-03-08T20:41:39.7827283Z ------------------------------------------
2020-03-08T20:41:39.7827283Z ------------------------------------------
2020-03-08T20:41:39.7827629Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7827965Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7828525Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7829240Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7829908Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7830548Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7830985Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7831347Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7831745Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7832132Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7832764Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7833117Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7833540Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7833948Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7834298Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7834716Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7835082Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7835648Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7836526Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\borrowed-enum.gdb\borrowed-enum.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.7837056Z No symbol 'the_a_ref' in current context
2020-03-08T20:41:39.7837525Z ------------------------------------------
2020-03-08T20:41:39.7837704Z 
2020-03-08T20:41:39.7837852Z 
2020-03-08T20:41:39.7838102Z ---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
2020-03-08T20:41:39.7838102Z ---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
2020-03-08T20:41:39.7838652Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7839007Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7839261Z 
2020-03-08T20:41:39.7839570Z error: line not found in debugger output: $1 = borrowed_c_style_enum::ABC::TheA
2020-03-08T20:41:39.7839984Z status: exit code: 0
2020-03-08T20:41:39.7849906Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\borrowed-c-style-enum.gdb\\borrowed-c-style-enum.debugger.script"
2020-03-08T20:41:39.7858001Z ------------------------------------------
2020-03-08T20:41:39.7858001Z ------------------------------------------
2020-03-08T20:41:39.7858619Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7858932Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7859372Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7860002Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7860837Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7861173Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7861550Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7861867Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7862230Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7862574Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7862914Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7863326Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7863545Z 
2020-03-08T20:41:39.7863769Z For help, type "help".
2020-03-08T20:41:39.7864238Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7864722Z Breakpoint 1 at 0x4016c4: file D:\a\1\s\src/test\debuginfo\borrowed-c-style-enum.rs, line 53.
2020-03-08T20:41:39.7865090Z [New Thread 1096.0x16d4]
2020-03-08T20:41:39.7865359Z [New Thread 1096.0x1620]
2020-03-08T20:41:39.7865636Z gdb: unknown target exception 0x4000001f at 0x4016c4
2020-03-08T20:41:39.7865882Z 
2020-03-08T20:41:39.7866091Z Thread 1 received signal ?, Unknown signal.
2020-03-08T20:41:39.7866375Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7866762Z ------------------------------------------
2020-03-08T20:41:39.7866979Z stderr:
2020-03-08T20:41:39.7867419Z ------------------------------------------
2020-03-08T20:41:39.7867419Z ------------------------------------------
2020-03-08T20:41:39.7867731Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7868129Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7869262Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7870059Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7870808Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7871348Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7871798Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7872174Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7872588Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7873036Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7874007Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7874459Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7875785Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7876222Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7876696Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7877121Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7877752Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7878279Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7879087Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\borrowed-c-style-enum.gdb\borrowed-c-style-enum.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.7879784Z No symbol 'the_a_ref' in current context
2020-03-08T20:41:39.7880216Z ------------------------------------------
2020-03-08T20:41:39.7880450Z 
2020-03-08T20:41:39.7880552Z 
2020-03-08T20:41:39.7880792Z ---- [debuginfo-gdb] debuginfo\borrowed-struct.rs stdout ----
2020-03-08T20:41:39.7880792Z ---- [debuginfo-gdb] debuginfo\borrowed-struct.rs stdout ----
2020-03-08T20:41:39.7881203Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7881604Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7881815Z 
2020-03-08T20:41:39.7882153Z error: line not found in debugger output: $1 = borrowed_struct::SomeStruct {x: 10, y: 23.5}
2020-03-08T20:41:39.7882578Z status: exit code: 0
2020-03-08T20:41:39.7889297Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\borrowed-struct.gdb\\borrowed-struct.debugger.script"
2020-03-08T20:41:39.7896359Z ------------------------------------------
2020-03-08T20:41:39.7896359Z ------------------------------------------
2020-03-08T20:41:39.7896813Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7897083Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7897735Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7898165Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7898584Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7898954Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7899262Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7899612Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7900357Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7900899Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7901232Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7901631Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7901842Z 
2020-03-08T20:41:39.7902056Z For help, type "help".
2020-03-08T20:41:39.7902341Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7902811Z Breakpoint 1 at 0x401925: file D:\a\1\s\src/test\debuginfo\borrowed-struct.rs, line 87.
2020-03-08T20:41:39.7903206Z [New Thread 5852.0x14e4]
2020-03-08T20:41:39.7903424Z [New Thread 5852.0x1158]
2020-03-08T20:41:39.7903681Z [New Thread 5852.0x38c]
2020-03-08T20:41:39.7903954Z gdb: unknown target exception 0x4000001f at 0x401925
2020-03-08T20:41:39.7904201Z 
2020-03-08T20:41:39.7904406Z Thread 1 received signal ?, Unknown signal.
2020-03-08T20:41:39.7904692Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7905036Z ------------------------------------------
2020-03-08T20:41:39.7905297Z stderr:
2020-03-08T20:41:39.7905562Z ------------------------------------------
2020-03-08T20:41:39.7905562Z ------------------------------------------
2020-03-08T20:41:39.7905863Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7906244Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7906761Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7907476Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7908192Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7908710Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7909141Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7909504Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7909985Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7910382Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7910793Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7911181Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7911543Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7911937Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7912272Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7912942Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7913290Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7913846Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7914596Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\borrowed-struct.gdb\borrowed-struct.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.7915100Z No symbol 'stack_val_ref' in current context
2020-03-08T20:41:39.7915612Z ------------------------------------------
2020-03-08T20:41:39.7915790Z 
2020-03-08T20:41:39.7915941Z 
2020-03-08T20:41:39.7916181Z ---- [debuginfo-gdb] debuginfo\borrowed-tuple.rs stdout ----
2020-03-08T20:41:39.7916181Z ---- [debuginfo-gdb] debuginfo\borrowed-tuple.rs stdout ----
2020-03-08T20:41:39.7916592Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7916947Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7917204Z 
2020-03-08T20:41:39.7917457Z error: line not found in debugger output: $1 = (-14, -19)
2020-03-08T20:41:39.7917972Z status: exit code: 0
2020-03-08T20:41:39.7924091Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\borrowed-tuple.gdb\\borrowed-tuple.debugger.script"
2020-03-08T20:41:39.7930239Z ------------------------------------------
2020-03-08T20:41:39.7930239Z ------------------------------------------
2020-03-08T20:41:39.7930514Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7930766Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7931186Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7931603Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7931995Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7932306Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7932655Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7932996Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7933286Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7933603Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7933977Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7934368Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7934566Z 
2020-03-08T20:41:39.7934771Z For help, type "help".
2020-03-08T20:41:39.7935049Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7935503Z Breakpoint 1 at 0x401baa: file D:\a\1\s\src/test\debuginfo\borrowed-tuple.rs, line 52.
2020-03-08T20:41:39.7935838Z [New Thread 4776.0x108c]
2020-03-08T20:41:39.7936091Z [New Thread 4776.0x1590]
2020-03-08T20:41:39.7936298Z [New Thread 4776.0x151c]
2020-03-08T20:41:39.7936608Z gdb: unknown target exception 0x4000001f at 0x401baa
2020-03-08T20:41:39.7936803Z 
2020-03-08T20:41:39.7937047Z Thread 1 received signal ?, Unknown signal.
2020-03-08T20:41:39.7937279Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7937652Z ------------------------------------------
2020-03-08T20:41:39.7937913Z stderr:
2020-03-08T20:41:39.7938117Z ------------------------------------------
2020-03-08T20:41:39.7938117Z ------------------------------------------
2020-03-08T20:41:39.7938446Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7938937Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7939460Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7940117Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7941153Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7944762Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7945878Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7946262Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7946682Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7947242Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7947912Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7948417Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7948828Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7949401Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7949750Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7950167Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7950783Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7951507Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7952180Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\borrowed-tuple.gdb\borrowed-tuple.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.7952713Z No symbol 'stack_val_ref' in current context
2020-03-08T20:41:39.7953130Z ------------------------------------------
2020-03-08T20:41:39.7953301Z 
2020-03-08T20:41:39.7953443Z 
2020-03-08T20:41:39.7953653Z ---- [debuginfo-gdb] debuginfo\box.rs stdout ----
2020-03-08T20:41:39.7953653Z ---- [debuginfo-gdb] debuginfo\box.rs stdout ----
2020-03-08T20:41:39.7954027Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7954371Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7954844Z error: line not found in debugger output: $1 = 1
2020-03-08T20:41:39.7955155Z status: exit code: 0
2020-03-08T20:41:39.7955155Z status: exit code: 0
2020-03-08T20:41:39.7961191Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\box.gdb\\box.debugger.script"
2020-03-08T20:41:39.7970080Z ------------------------------------------
2020-03-08T20:41:39.7970080Z ------------------------------------------
2020-03-08T20:41:39.7970321Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.7970631Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.7971016Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.7971496Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.7971993Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.7972331Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.7972687Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.7972999Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.7973350Z For bug reporting instructions, please see:
2020-03-08T20:41:39.7973677Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.7974007Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.7974407Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.7974617Z 
2020-03-08T20:41:39.7974781Z For help, type "help".
2020-03-08T20:41:39.7975114Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.7975558Z Breakpoint 1 at 0x4016de: file D:\a\1\s\src/test\debuginfo\box.rs, line 35.
2020-03-08T20:41:39.7976120Z [New Thread 2272.0x12e0]
2020-03-08T20:41:39.7976613Z gdb: unknown target exception 0x4000001f at 0x4016de
2020-03-08T20:41:39.7976818Z 
2020-03-08T20:41:39.7977072Z Program received signal ?, Unknown signal.
2020-03-08T20:41:39.7977310Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.7977767Z ------------------------------------------
2020-03-08T20:41:39.7978197Z stderr:
2020-03-08T20:41:39.7978404Z ------------------------------------------
2020-03-08T20:41:39.7978404Z ------------------------------------------
2020-03-08T20:41:39.7978738Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.7979065Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.7979630Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7980269Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7980972Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7981523Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7981894Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7982293Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7982633Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.7983054Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.7983447Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7983771Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7984164Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7984498Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7984873Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.7985215Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.7985596Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.7986126Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.7986927Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\box.gdb\box.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.7987419Z No symbol 'a' in current context
2020-03-08T20:41:39.7987846Z ------------------------------------------
2020-03-08T20:41:39.7988032Z 
2020-03-08T20:41:39.7988137Z 
2020-03-08T20:41:39.7988602Z ---- [debuginfo-gdb] debuginfo\borrowed-unique-basic.rs stdout ----
2020-03-08T20:41:39.7988602Z ---- [debuginfo-gdb] debuginfo\borrowed-unique-basic.rs stdout ----
2020-03-08T20:41:39.7988966Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.7989395Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.7989885Z error: line not found in debugger output: $1 = true
2020-03-08T20:41:39.7990222Z status: exit code: 0
2020-03-08T20:41:39.7990222Z status: exit code: 0
2020-03-08T20:41:39.7996389Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\borrowed-unique-basic.gdb\\borrowed-unique-basic.debugger.script"
2020-03-08T20:41:39.8004494Z ------------------------------------------
2020-03-08T20:41:39.8004494Z ------------------------------------------
2020-03-08T20:41:39.8004796Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.8005117Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.8005520Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.8006029Z This is free software: you are free to change and redistribute it.
2020-03-08T20:41:39.8007127Z There is NO WARRANTY, to the extent permitted by law.
2020-03-08T20:41:39.8007483Z Type "show copying" and "show warranty" for details.
2020-03-08T20:41:39.8007822Z This GDB was configured as "x86_64-pc-msys".
2020-03-08T20:41:39.8008108Z Type "show configuration" for configuration details.
2020-03-08T20:41:39.8008614Z For bug reporting instructions, please see:
2020-03-08T20:41:39.8008889Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-08T20:41:39.8009248Z Find the GDB manual and other documentation resources online at:
2020-03-08T20:41:39.8009582Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-03-08T20:41:39.8009825Z 
2020-03-08T20:41:39.8009985Z For help, type "help".
2020-03-08T20:41:39.8010305Z Type "apropos word" to search for commands related to "word".
2020-03-08T20:41:39.8011047Z Breakpoint 1 at 0x4020bd: file D:\a\1\s\src/test\debuginfo\borrowed-unique-basic.rs, line 166.
2020-03-08T20:41:39.8011559Z [New Thread 5428.0x178c]
2020-03-08T20:41:39.8011787Z [New Thread 5428.0x15f4]
2020-03-08T20:41:39.8012119Z gdb: unknown target exception 0x4000001f at 0x4020bd
2020-03-08T20:41:39.8012327Z 
2020-03-08T20:41:39.8012581Z Thread 1 received signal ?, Unknown signal.
2020-03-08T20:41:39.8012821Z 0x0009e058 in ?? ()
2020-03-08T20:41:39.8013542Z ------------------------------------------
2020-03-08T20:41:39.8013914Z stderr:
2020-03-08T20:41:39.8014128Z ------------------------------------------
2020-03-08T20:41:39.8014128Z ------------------------------------------
2020-03-08T20:41:39.8014480Z Warning: /d/a/1/s/D: No such file or directory.
2020-03-08T20:41:39.8014816Z Warning: /d/a/1/s/a1s./src/etc: No such file or directory.
2020-03-08T20:41:39.8015571Z warning: `/c/windows/SYSTEM32/ntdll.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.8016311Z warning: `/c/windows/System32/wow64.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.8017009Z warning: `/c/windows/System32/wow64win.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.8017685Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.8018267Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.8018978Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.8019543Z warning: dll path for "WOW64_IMAGE_SECTION" can not be evaluated
2020-03-08T20:41:39.8020116Z warning: Could not load shared library symbols for WOW64_IMAGE_SECTION.
2020-03-08T20:41:39.8020696Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.8021035Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.8021443Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.8022676Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.8024029Z warning: dll path for "NOT_AN_IMAGE" can not be evaluated
2020-03-08T20:41:39.8024671Z warning: Could not load shared library symbols for NOT_AN_IMAGE.
2020-03-08T20:41:39.8025034Z Do you need "set solib-search-path" or "set sysroot"?
2020-03-08T20:41:39.8025611Z warning: `/c/windows/System32/wow64cpu.dll': Shared library architecture i386:x86-64 is not compatible with target architecture i386.
2020-03-08T20:41:39.8026516Z D:\a\1\s\build\i686-pc-windows-gnu\test\debuginfo\borrowed-unique-basic.gdb\borrowed-unique-basic.debugger.script:10: Error in sourced command file:
2020-03-08T20:41:39.8027563Z No symbol 'bool_ref' in current context
2020-03-08T20:41:39.8028276Z ------------------------------------------
2020-03-08T20:41:39.8028463Z 
2020-03-08T20:41:39.8028616Z 
2020-03-08T20:41:39.8028908Z ---- [debuginfo-gdb] debuginfo\by-value-self-argument-in-trait-impl.rs stdout ----
2020-03-08T20:41:39.8028908Z ---- [debuginfo-gdb] debuginfo\by-value-self-argument-in-trait-impl.rs stdout ----
2020-03-08T20:41:39.8029711Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-08T20:41:39.8030087Z NOTE: compiletest thinks it is using GDB version 8002001
2020-03-08T20:41:39.8030738Z error: line not found in debugger output: $1 = 1111
2020-03-08T20:41:39.8031086Z status: exit code: 0
2020-03-08T20:41:39.8031086Z status: exit code: 0
2020-03-08T20:41:39.8040049Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;D:\a\1\s\mingw32\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo\\by-value-self-argument-in-trait-impl.gdb\\by-value-self-argument-in-trait-impl.debugger.script"
2020-03-08T20:41:39.8047959Z ------------------------------------------
2020-03-08T20:41:39.8047959Z ------------------------------------------
2020-03-08T20:41:39.8048219Z GNU gdb (GDB) 8.2.1
2020-03-08T20:41:39.8048545Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-08T20:41:39.8048993Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-08T20:41:39.8049449Z This is free software: you are free to change and redistribute it.
---
2020-03-08T20:41:40.1215316Z test result: FAILED. 13 passed; 70 failed; 32 ignored; 0 measured; 0 filtered out
2020-03-08T20:41:40.1215733Z 
2020-03-08T20:41:40.1215831Z 
2020-03-08T20:41:40.1215945Z 
2020-03-08T20:41:40.1219761Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\debuginfo" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\debuginfo" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "debuginfo" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\usr\\bin\\gdb" "--quiet" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-08T20:41:40.1223660Z 
2020-03-08T20:41:40.1223783Z 
2020-03-08T20:41:40.1224641Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2020-03-08T20:41:40.1225102Z Build completed unsuccessfully in 1:42:45
2020-03-08T20:41:40.1225102Z Build completed unsuccessfully in 1:42:45
2020-03-08T20:41:40.1225434Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2020-03-08T20:41:40.1226059Z   local time: Sun Mar  8 20:41:40 CUT 2020
2020-03-08T20:41:40.6008893Z   network time: Sun, 08 Mar 2020 20:41:40 GMT
2020-03-08T20:41:40.6009257Z == end clock drift check ==
2020-03-08T20:41:40.7316190Z 
2020-03-08T20:41:40.7316190Z 
2020-03-08T20:41:40.9618007Z ##[error]Bash exited with code '2'.
2020-03-08T20:41:41.0261917Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-03-08T20:41:41.0966072Z ==============================================================================
2020-03-08T20:41:41.0966854Z Task         : Get sources
2020-03-08T20:41:41.0967199Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
