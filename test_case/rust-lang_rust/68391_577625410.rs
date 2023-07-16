plain
2020-01-23T08:43:50.3003627Z ========================== Starting Command Output ===========================
2020-01-23T08:43:50.3004127Z [command]"C:\Program Files\Git\usr\bin\bash.exe" --noprofile --norc /d/a/_temp/77e30c00-22de-4d2c-a7fd-6340df539430.sh
2020-01-23T08:43:50.3004152Z 
2020-01-23T08:43:50.3006037Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T08:43:50.3020791Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68391/merge to s
2020-01-23T08:43:50.3119244Z Task         : Get sources
2020-01-23T08:43:50.3119275Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T08:43:50.3119306Z Version      : 1.0.0
2020-01-23T08:43:50.3119351Z Author       : Microsoft
---
2020-01-23T08:43:53.2978507Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T08:43:53.2989824Z ##[command]git config gc.auto 0
2020-01-23T08:43:53.3000434Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T08:43:53.3481947Z ##[command]git config --get-all http.proxy
2020-01-23T08:43:53.3902734Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68391/merge:refs/remotes/pull/68391/merge
---
2020-01-23T10:24:25.6211516Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/test/ui", "src/test/compile-fail", "src/tools/linkchecker"]
2020-01-23T10:24:25.6314825Z Check compiletest suite=codegen mode=codegen (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
2020-01-23T10:24:25.8814094Z 
2020-01-23T10:24:25.8814650Z running 169 tests
2020-01-23T10:24:30.8650041Z ..i....................iiii...i......ii.....................i..i.......i........ii.............i.... 100/169
2020-01-23T10:24:34.3783642Z .i..iii.iii..iiiiiiiiii.........................ii.............i.....
2020-01-23T10:24:34.3784173Z 
2020-01-23T10:24:34.3784227Z  finished in 8.319
2020-01-23T10:24:34.3784323Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/test/ui", "src/test/compile-fail", "src/tools/linkchecker"]
2020-01-23T10:24:34.3784463Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
---
2020-01-23T10:24:40.3367157Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/test/ui", "src/test/compile-fail", "src/tools/linkchecker"]
2020-01-23T10:24:40.3466916Z Check compiletest suite=assembly mode=assembly (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
2020-01-23T10:24:40.5489149Z 
2020-01-23T10:24:40.5489782Z running 9 tests
2020-01-23T10:24:40.5490847Z iiiiiiiii
2020-01-23T10:24:40.5492013Z 
2020-01-23T10:24:40.5500521Z  finished in 0.203
2020-01-23T10:24:40.5525162Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/test/ui", "src/test/compile-fail", "src/tools/linkchecker"]
2020-01-23T10:24:40.5617912Z Check compiletest suite=incremental mode=incremental (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
---
2020-01-23T10:25:29.0276984Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/test/ui", "src/test/compile-fail", "src/tools/linkchecker"]
2020-01-23T10:25:29.0491320Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
2020-01-23T10:25:29.3110197Z 
2020-01-23T10:25:29.3110681Z running 232 tests
2020-01-23T10:26:12.2995869Z ....i........i......i..i.ii...i..i............ii...............ii..........i..i.i.i............i.... 100/232
2020-01-23T10:26:41.8521423Z .............i..iiiiiFFFFFFFFiFFiF.Fi.Fi.iiFFFiFFiFFFFFFFFFFFFiF.i.F.FFF.FFFFiiiiFFFFF.FFFFiFFiFiii. 200/232
2020-01-23T10:26:50.8632066Z .FFFFF.FiiFiFiFFF.FFFiiFF.FFFiiF
2020-01-23T10:26:50.8647819Z 
2020-01-23T10:26:50.8648437Z ---- [debuginfo-gdb] debuginfo\associated-types.rs stdout ----
2020-01-23T10:26:50.8648437Z ---- [debuginfo-gdb] debuginfo\associated-types.rs stdout ----
2020-01-23T10:26:50.8649281Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8649874Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8650594Z error: gdb failed to execute
2020-01-23T10:26:50.8651070Z status: exit code: 1
2020-01-23T10:26:50.8651070Z status: exit code: 1
2020-01-23T10:26:50.8652458Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\associated-types.gdb\\associated-types.debugger.script"
2020-01-23T10:26:50.8654116Z ------------------------------------------
2020-01-23T10:26:50.8654116Z ------------------------------------------
2020-01-23T10:26:50.8654487Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8654936Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8655315Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8683538Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8683628Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8683691Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8683807Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8683863Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8683919Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8684020Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8684079Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8684140Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8684176Z 
2020-01-23T10:26:50.8684273Z For help, type "help".
2020-01-23T10:26:50.8684360Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8684425Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8684550Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8684616Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8684737Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8684803Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8684867Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8684977Z [New Thread 1896.0x148c]
2020-01-23T10:26:50.8685027Z [New Thread 1896.0x1538]
2020-01-23T10:26:50.8685078Z [Thread 1896.0x1538 exited with code 0]
2020-01-23T10:26:50.8685202Z [Thread 1896.0x148c exited with code 0]
2020-01-23T10:26:50.8685259Z [Inferior 1 (process 1896) exited normally]
2020-01-23T10:26:50.8685340Z ------------------------------------------
2020-01-23T10:26:50.8685537Z stderr:
2020-01-23T10:26:50.8685587Z ------------------------------------------
2020-01-23T10:26:50.8685587Z ------------------------------------------
2020-01-23T10:26:50.8685642Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8685749Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8685804Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8685858Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8685965Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8686019Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8686073Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8686282Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\associated-types.gdb\associated-types.debugger.script:15: Error in sourced command file:
2020-01-23T10:26:50.8686486Z No symbol 'arg' in current context
2020-01-23T10:26:50.8686570Z ------------------------------------------
2020-01-23T10:26:50.8686659Z 
2020-01-23T10:26:50.8686688Z 
2020-01-23T10:26:50.8686740Z ---- [debuginfo-gdb] debuginfo\borrowed-basic.rs stdout ----
2020-01-23T10:26:50.8686740Z ---- [debuginfo-gdb] debuginfo\borrowed-basic.rs stdout ----
2020-01-23T10:26:50.8686800Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8686913Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8687203Z error: gdb failed to execute
2020-01-23T10:26:50.8687328Z status: exit code: 1
2020-01-23T10:26:50.8687328Z status: exit code: 1
2020-01-23T10:26:50.8695084Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-basic.gdb\\borrowed-basic.debugger.script"
2020-01-23T10:26:50.8695831Z ------------------------------------------
2020-01-23T10:26:50.8695831Z ------------------------------------------
2020-01-23T10:26:50.8696009Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8696068Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8696131Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8696243Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8696303Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8696564Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8696678Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8696734Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8696787Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8696888Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8696947Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8697004Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8697149Z 
2020-01-23T10:26:50.8697242Z For help, type "help".
2020-01-23T10:26:50.8697299Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8697362Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8697474Z [New Thread 916.0x16b4]
2020-01-23T10:26:50.8697525Z [New Thread 916.0x1718]
2020-01-23T10:26:50.8697588Z [Thread 916.0x16ec exited with code 0]
2020-01-23T10:26:50.8697697Z [Thread 916.0x1718 exited with code 0]
2020-01-23T10:26:50.8697750Z [Inferior 1 (process 916) exited normally]
2020-01-23T10:26:50.8697883Z ------------------------------------------
2020-01-23T10:26:50.8697930Z stderr:
2020-01-23T10:26:50.8697980Z ------------------------------------------
2020-01-23T10:26:50.8697980Z ------------------------------------------
2020-01-23T10:26:50.8698034Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8698138Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8698204Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-basic.gdb\borrowed-basic.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8698307Z No symbol 'bool_ref' in current context
2020-01-23T10:26:50.8698388Z ------------------------------------------
2020-01-23T10:26:50.8698419Z 
2020-01-23T10:26:50.8698446Z 
2020-01-23T10:26:50.8698556Z ---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
2020-01-23T10:26:50.8698556Z ---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
2020-01-23T10:26:50.8698623Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8698683Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8698855Z error: gdb failed to execute
2020-01-23T10:26:50.8698904Z status: exit code: 1
2020-01-23T10:26:50.8698904Z status: exit code: 1
2020-01-23T10:26:50.8700232Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-c-style-enum.gdb\\borrowed-c-style-enum.debugger.script"
2020-01-23T10:26:50.8701027Z ------------------------------------------
2020-01-23T10:26:50.8701027Z ------------------------------------------
2020-01-23T10:26:50.8701234Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8701306Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8701428Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8701489Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8701547Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8701657Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8701714Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8701769Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8701880Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8701934Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8702009Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8702123Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8702156Z 
2020-01-23T10:26:50.8702203Z For help, type "help".
2020-01-23T10:26:50.8702258Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8702376Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8702432Z [New Thread 2980.0x1754]
2020-01-23T10:26:50.8702481Z [New Thread 2980.0x15dc]
2020-01-23T10:26:50.8702590Z [Thread 2980.0x15dc exited with code 0]
2020-01-23T10:26:50.8702641Z [Thread 2980.0x1754 exited with code 0]
2020-01-23T10:26:50.8702697Z [Inferior 1 (process 2980) exited normally]
2020-01-23T10:26:50.8756347Z ------------------------------------------
2020-01-23T10:26:50.8756387Z stderr:
2020-01-23T10:26:50.8756453Z ------------------------------------------
2020-01-23T10:26:50.8756453Z ------------------------------------------
2020-01-23T10:26:50.8756520Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8756564Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8756617Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-c-style-enum.gdb\borrowed-c-style-enum.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8756687Z No symbol 'the_a_ref' in current context
2020-01-23T10:26:50.8756755Z ------------------------------------------
2020-01-23T10:26:50.8756795Z 
2020-01-23T10:26:50.8756817Z 
2020-01-23T10:26:50.8756858Z ---- [debuginfo-gdb] debuginfo\borrowed-enum.rs stdout ----
2020-01-23T10:26:50.8756858Z ---- [debuginfo-gdb] debuginfo\borrowed-enum.rs stdout ----
2020-01-23T10:26:50.8756906Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8757319Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8757390Z error: gdb failed to execute
2020-01-23T10:26:50.8757454Z status: exit code: 1
2020-01-23T10:26:50.8757454Z status: exit code: 1
2020-01-23T10:26:50.8758235Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-enum.gdb\\borrowed-enum.debugger.script"
2020-01-23T10:26:50.8758860Z ------------------------------------------
2020-01-23T10:26:50.8758860Z ------------------------------------------
2020-01-23T10:26:50.8758902Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8758976Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8759452Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8759524Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8759610Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8759660Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8759710Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8759784Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8759832Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8759879Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8759956Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8760007Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8760037Z 
2020-01-23T10:26:50.8760078Z For help, type "help".
2020-01-23T10:26:50.8760154Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8760369Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8760456Z [Thread 2744.0x10dc exited with code 0]
2020-01-23T10:26:50.8760503Z [New Thread 2744.0xb8c]
2020-01-23T10:26:50.8760550Z [Inferior 1 (process 2744) exited normally]
2020-01-23T10:26:50.8760648Z ------------------------------------------
2020-01-23T10:26:50.8760689Z stderr:
2020-01-23T10:26:50.8760733Z ------------------------------------------
2020-01-23T10:26:50.8760733Z ------------------------------------------
2020-01-23T10:26:50.8760800Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8760850Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8760908Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-enum.gdb\borrowed-enum.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8760990Z No symbol 'the_a_ref' in current context
2020-01-23T10:26:50.8761060Z ------------------------------------------
2020-01-23T10:26:50.8761103Z 
2020-01-23T10:26:50.8761159Z 
2020-01-23T10:26:50.8761207Z ---- [debuginfo-gdb] debuginfo\borrowed-struct.rs stdout ----
2020-01-23T10:26:50.8761207Z ---- [debuginfo-gdb] debuginfo\borrowed-struct.rs stdout ----
2020-01-23T10:26:50.8761259Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8761312Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8761399Z error: gdb failed to execute
2020-01-23T10:26:50.8761443Z status: exit code: 1
2020-01-23T10:26:50.8761443Z status: exit code: 1
2020-01-23T10:26:50.8762113Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-struct.gdb\\borrowed-struct.debugger.script"
2020-01-23T10:26:50.8762936Z ------------------------------------------
2020-01-23T10:26:50.8762936Z ------------------------------------------
2020-01-23T10:26:50.8762981Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8763027Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8763110Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8763160Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8763297Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8763345Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8763390Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8763539Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8763630Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8763671Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8763715Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8763787Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8763811Z 
2020-01-23T10:26:50.8763847Z For help, type "help".
2020-01-23T10:26:50.8763895Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8763945Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8763989Z [Thread 5860.0x67c exited with code 0]
2020-01-23T10:26:50.8764051Z [New Thread 5860.0xbd0]
2020-01-23T10:26:50.8764092Z [Inferior 1 (process 5860) exited normally]
2020-01-23T10:26:50.8764153Z ------------------------------------------
2020-01-23T10:26:50.8764211Z stderr:
2020-01-23T10:26:50.8764249Z ------------------------------------------
2020-01-23T10:26:50.8764249Z ------------------------------------------
2020-01-23T10:26:50.8764303Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8764367Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8764419Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-struct.gdb\borrowed-struct.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8764466Z No symbol 'stack_val_ref' in current context
2020-01-23T10:26:50.8764546Z ------------------------------------------
2020-01-23T10:26:50.8764569Z 
2020-01-23T10:26:50.8764589Z 
2020-01-23T10:26:50.8764655Z ---- [debuginfo-gdb] debuginfo\borrowed-tuple.rs stdout ----
2020-01-23T10:26:50.8764655Z ---- [debuginfo-gdb] debuginfo\borrowed-tuple.rs stdout ----
2020-01-23T10:26:50.8764701Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8764745Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8764827Z error: gdb failed to execute
2020-01-23T10:26:50.8764865Z status: exit code: 1
2020-01-23T10:26:50.8764865Z status: exit code: 1
2020-01-23T10:26:50.8765937Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-tuple.gdb\\borrowed-tuple.debugger.script"
2020-01-23T10:26:50.8766512Z ------------------------------------------
2020-01-23T10:26:50.8766512Z ------------------------------------------
2020-01-23T10:26:50.8766551Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8766595Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8766674Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8766728Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8766773Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8766833Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8766876Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8766919Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8766984Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8767026Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8767069Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8767113Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8767156Z 
2020-01-23T10:26:50.8767191Z For help, type "help".
2020-01-23T10:26:50.8767265Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8767344Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8767395Z [Thread 5836.0x1080 exited with code 0]
2020-01-23T10:26:50.8767434Z [New Thread 5836.0xb5c]
2020-01-23T10:26:50.8767491Z [Inferior 1 (process 5836) exited normally]
2020-01-23T10:26:50.8767553Z ------------------------------------------
2020-01-23T10:26:50.8767588Z stderr:
2020-01-23T10:26:50.8767646Z ------------------------------------------
2020-01-23T10:26:50.8767646Z ------------------------------------------
2020-01-23T10:26:50.8767687Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8767729Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8767809Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-tuple.gdb\borrowed-tuple.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8767855Z No symbol 'stack_val_ref' in current context
2020-01-23T10:26:50.8767939Z ------------------------------------------
2020-01-23T10:26:50.8768323Z 
2020-01-23T10:26:50.8768400Z 
2020-01-23T10:26:50.8768451Z ---- [debuginfo-gdb] debuginfo\borrowed-unique-basic.rs stdout ----
2020-01-23T10:26:50.8768451Z ---- [debuginfo-gdb] debuginfo\borrowed-unique-basic.rs stdout ----
2020-01-23T10:26:50.8768531Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8768577Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8768638Z error: gdb failed to execute
2020-01-23T10:26:50.8768818Z status: exit code: 1
2020-01-23T10:26:50.8768818Z status: exit code: 1
2020-01-23T10:26:50.8769836Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\borrowed-unique-basic.gdb\\borrowed-unique-basic.debugger.script"
2020-01-23T10:26:50.8770369Z ------------------------------------------
2020-01-23T10:26:50.8770369Z ------------------------------------------
2020-01-23T10:26:50.8770440Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8770491Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8770546Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8770624Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8770676Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8770725Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8770800Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8770848Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8771043Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8771118Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8771169Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8771219Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8771249Z 
2020-01-23T10:26:50.8771322Z For help, type "help".
2020-01-23T10:26:50.8771371Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8771427Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8771507Z [New Thread 184.0x320]
2020-01-23T10:26:50.8771552Z [New Thread 184.0x12f4]
2020-01-23T10:26:50.8771598Z [Thread 184.0x1328 exited with code 0]
2020-01-23T10:26:50.8771663Z [Thread 184.0x12f4 exited with code 0]
2020-01-23T10:26:50.8771710Z [Inferior 1 (process 184) exited normally]
2020-01-23T10:26:50.8771781Z ------------------------------------------
2020-01-23T10:26:50.8771869Z stderr:
2020-01-23T10:26:50.8771914Z ------------------------------------------
2020-01-23T10:26:50.8771914Z ------------------------------------------
2020-01-23T10:26:50.8771962Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8772027Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8772088Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\borrowed-unique-basic.gdb\borrowed-unique-basic.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8772141Z No symbol 'bool_ref' in current context
2020-01-23T10:26:50.8772240Z ------------------------------------------
2020-01-23T10:26:50.8772267Z 
2020-01-23T10:26:50.8772291Z 
2020-01-23T10:26:50.8772357Z ---- [debuginfo-gdb] debuginfo\box.rs stdout ----
2020-01-23T10:26:50.8772357Z ---- [debuginfo-gdb] debuginfo\box.rs stdout ----
2020-01-23T10:26:50.8772408Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8772458Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8772557Z error: gdb failed to execute
2020-01-23T10:26:50.8772601Z status: exit code: 1
2020-01-23T10:26:50.8772601Z status: exit code: 1
2020-01-23T10:26:50.8773259Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\box.gdb\\box.debugger.script"
2020-01-23T10:26:50.8773831Z ------------------------------------------
2020-01-23T10:26:50.8773831Z ------------------------------------------
2020-01-23T10:26:50.8773876Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8773939Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8774021Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8774075Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8774127Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8774194Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8774244Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8774293Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8774375Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8774422Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8774473Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8774540Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8774569Z 
2020-01-23T10:26:50.8774610Z For help, type "help".
2020-01-23T10:26:50.8774671Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8774753Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8774802Z [New Thread 4500.0x378]
2020-01-23T10:26:50.8774847Z [New Thread 4500.0x16f4]
2020-01-23T10:26:50.8774985Z [Thread 4500.0x1568 exited with code 0]
2020-01-23T10:26:50.8775031Z [Thread 4500.0x16f4 exited with code 0]
2020-01-23T10:26:50.8775077Z [Inferior 1 (process 4500) exited normally]
2020-01-23T10:26:50.8775282Z ------------------------------------------
2020-01-23T10:26:50.8775322Z stderr:
2020-01-23T10:26:50.8775365Z ------------------------------------------
2020-01-23T10:26:50.8775365Z ------------------------------------------
2020-01-23T10:26:50.8775557Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8775603Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8775655Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\box.gdb\box.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8775723Z No symbol 'a' in current context
2020-01-23T10:26:50.8775790Z ------------------------------------------
2020-01-23T10:26:50.8775815Z 
2020-01-23T10:26:50.8775845Z 
2020-01-23T10:26:50.8775888Z ---- [debuginfo-gdb] debuginfo\boxed-struct.rs stdout ----
2020-01-23T10:26:50.8775888Z ---- [debuginfo-gdb] debuginfo\boxed-struct.rs stdout ----
2020-01-23T10:26:50.8775938Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8775985Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8776065Z error: gdb failed to execute
2020-01-23T10:26:50.8776106Z status: exit code: 1
2020-01-23T10:26:50.8776106Z status: exit code: 1
2020-01-23T10:26:50.8776717Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\boxed-struct.gdb\\boxed-struct.debugger.script"
2020-01-23T10:26:50.8777332Z ------------------------------------------
2020-01-23T10:26:50.8777332Z ------------------------------------------
2020-01-23T10:26:50.8777371Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8777413Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8777470Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8777517Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8777576Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8777620Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8777674Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8777717Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8777767Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8777808Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8777851Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8777904Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8777929Z 
2020-01-23T10:26:50.8777964Z For help, type "help".
2020-01-23T10:26:50.8778014Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8778064Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8778106Z [New Thread 2080.0x1678]
2020-01-23T10:26:50.8778153Z [Thread 2080.0x1678 exited with code 0]
2020-01-23T10:26:50.8778193Z [Thread 2080.0x834 exited with code 0]
2020-01-23T10:26:50.8778232Z [New Thread 2080.0x1040]
2020-01-23T10:26:50.8778367Z [Inferior 1 (process 2080) exited normally]
2020-01-23T10:26:50.8778446Z ------------------------------------------
2020-01-23T10:26:50.8778481Z stderr:
2020-01-23T10:26:50.8778529Z ------------------------------------------
2020-01-23T10:26:50.8778529Z ------------------------------------------
2020-01-23T10:26:50.8778571Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8778613Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8778786Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\boxed-struct.gdb\boxed-struct.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8778839Z No symbol 'boxed_with_padding' in current context
2020-01-23T10:26:50.8778906Z ------------------------------------------
2020-01-23T10:26:50.8778940Z 
2020-01-23T10:26:50.8778962Z 
2020-01-23T10:26:50.8779768Z ---- [debuginfo-gdb] debuginfo\by-value-self-argument-in-trait-impl.rs stdout ----
2020-01-23T10:26:50.8779768Z ---- [debuginfo-gdb] debuginfo\by-value-self-argument-in-trait-impl.rs stdout ----
2020-01-23T10:26:50.8779863Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8779930Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8780000Z error: gdb failed to execute
2020-01-23T10:26:50.8780059Z status: exit code: 1
2020-01-23T10:26:50.8780059Z status: exit code: 1
2020-01-23T10:26:50.8780761Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\by-value-self-argument-in-trait-impl.gdb\\by-value-self-argument-in-trait-impl.debugger.script"
2020-01-23T10:26:50.8781313Z ------------------------------------------
2020-01-23T10:26:50.8781313Z ------------------------------------------
2020-01-23T10:26:50.8781367Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8781416Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8781468Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8781531Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8781583Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8781632Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8781689Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8781738Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8781786Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8781832Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8781904Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8781955Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8781983Z 
2020-01-23T10:26:50.8782030Z For help, type "help".
2020-01-23T10:26:50.8782079Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8782136Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8782205Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8782263Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8782310Z [New Thread 5088.0x1770]
2020-01-23T10:26:50.8782363Z [Thread 5088.0x4b8 exited with code 0]
2020-01-23T10:26:50.8782406Z [New Thread 5088.0x1410]
2020-01-23T10:26:50.8782451Z [Thread 5088.0x1410 exited with code 0]
2020-01-23T10:26:50.8782512Z [Inferior 1 (process 5088) exited normally]
2020-01-23T10:26:50.8782590Z ------------------------------------------
2020-01-23T10:26:50.8782631Z stderr:
2020-01-23T10:26:50.8782684Z ------------------------------------------
2020-01-23T10:26:50.8782684Z ------------------------------------------
2020-01-23T10:26:50.8782731Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8782779Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8782836Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8782884Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8782948Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\by-value-self-argument-in-trait-impl.gdb\by-value-self-argument-in-trait-impl.debugger.script:12: Error in sourced command file:
2020-01-23T10:26:50.8783120Z No symbol 'self' in current context
2020-01-23T10:26:50.8783188Z ------------------------------------------
2020-01-23T10:26:50.8783214Z 
2020-01-23T10:26:50.8783244Z 
2020-01-23T10:26:50.8783303Z ---- [debuginfo-gdb] debuginfo\c-style-enum-in-composite.rs stdout ----
2020-01-23T10:26:50.8783303Z ---- [debuginfo-gdb] debuginfo\c-style-enum-in-composite.rs stdout ----
2020-01-23T10:26:50.8783354Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8783415Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8783482Z error: gdb failed to execute
2020-01-23T10:26:50.8783524Z status: exit code: 1
2020-01-23T10:26:50.8783524Z status: exit code: 1
2020-01-23T10:26:50.8784169Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\c-style-enum-in-composite.gdb\\c-style-enum-in-composite.debugger.script"
2020-01-23T10:26:50.8784819Z ------------------------------------------
2020-01-23T10:26:50.8784819Z ------------------------------------------
2020-01-23T10:26:50.8784860Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8784915Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8784964Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8785013Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8785184Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8785228Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8785270Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8785335Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8785377Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8785430Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8785493Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8785538Z     <http://www.gnu.org/software/gdb/documentation/>.
2020-01-23T10:26:50.8785563Z 
2020-01-23T10:26:50.8785598Z For help, type "help".
2020-01-23T10:26:50.8785648Z Type "apropos word" to search for commands related to "word".
2020-01-23T10:26:50.8785696Z Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
2020-01-23T10:26:50.8785738Z [New Thread 6004.0x338]
2020-01-23T10:26:50.8785803Z [Thread 6004.0x338 exited with code 0]
2020-01-23T10:26:50.8785842Z [Thread 6004.0x146c exited with code 0]
2020-01-23T10:26:50.8785880Z [New Thread 6004.0x123c]
2020-01-23T10:26:50.8785928Z [Inferior 1 (process 6004) exited normally]
2020-01-23T10:26:50.8785990Z ------------------------------------------
2020-01-23T10:26:50.8786082Z stderr:
2020-01-23T10:26:50.8786172Z ------------------------------------------
2020-01-23T10:26:50.8786172Z ------------------------------------------
2020-01-23T10:26:50.8786220Z Warning: D:a1s./src/etc: No such file or directory.
2020-01-23T10:26:50.8786263Z No symbol table is loaded.  Use the "file" command.
2020-01-23T10:26:50.8786325Z D:\a\1\s\build\x86_64-pc-windows-msvc\test\debuginfo\c-style-enum-in-composite.gdb\c-style-enum-in-composite.debugger.script:10: Error in sourced command file:
2020-01-23T10:26:50.8786375Z No symbol 'tuple_interior_padding' in current context
2020-01-23T10:26:50.8786452Z ------------------------------------------
2020-01-23T10:26:50.8786475Z 
2020-01-23T10:26:50.8786496Z 
2020-01-23T10:26:50.8786537Z ---- [debuginfo-gdb] debuginfo\closure-in-generic-function.rs stdout ----
2020-01-23T10:26:50.8786537Z ---- [debuginfo-gdb] debuginfo\closure-in-generic-function.rs stdout ----
2020-01-23T10:26:50.8786596Z NOTE: compiletest thinks it is using GDB with native rust support
2020-01-23T10:26:50.8786641Z NOTE: compiletest thinks it is using GDB version 8003001
2020-01-23T10:26:50.8786715Z error: gdb failed to execute
2020-01-23T10:26:50.8786766Z status: exit code: 1
2020-01-23T10:26:50.8786766Z status: exit code: 1
2020-01-23T10:26:50.8787305Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw64\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.164.6\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\13" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo\\closure-in-generic-function.gdb\\closure-in-generic-function.debugger.script"
2020-01-23T10:26:50.8787918Z ------------------------------------------
2020-01-23T10:26:50.8787918Z ------------------------------------------
2020-01-23T10:26:50.8788029Z GNU gdb (GDB) 8.3.1
2020-01-23T10:26:50.8788117Z Copyright (C) 2019 Free Software Foundation, Inc.
2020-01-23T10:26:50.8788168Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-01-23T10:26:50.8788223Z This is free software: you are free to change and redistribute it.
2020-01-23T10:26:50.8788268Z There is NO WARRANTY, to the extent permitted by law.
2020-01-23T10:26:50.8788311Z Type "show copying" and "show warranty" for details.
2020-01-23T10:26:50.8788362Z This GDB was configured as "x86_64-w64-mingw32".
2020-01-23T10:26:50.8788405Z Type "show configuration" for configuration details.
2020-01-23T10:26:50.8788446Z For bug reporting instructions, please see:
2020-01-23T10:26:50.8788496Z <http://www.gnu.org/software/gdb/bugs/>.
2020-01-23T10:26:50.8788541Z Find the GDB manual and other documentation resources online at:
2020-01-23T10:26:50.8788585Z     <http://www.gnu.org/software/gdb/documentation/>.
---
2020-01-23T10:26:50.9018101Z test result: FAILED. 111 passed; 71 failed; 50 ignored; 0 measured; 0 filtered out
2020-01-23T10:26:50.9018132Z 
2020-01-23T10:26:50.9018153Z 
2020-01-23T10:26:50.9018174Z 
2020-01-23T10:26:50.9018407Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\debuginfo" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\debuginfo" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--mode" "debuginfo" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "--quiet" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-23T10:26:50.9018677Z 
2020-01-23T10:26:50.9018700Z 
2020-01-23T10:26:50.9018700Z 
2020-01-23T10:26:50.9617400Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2020-01-23T10:26:50.9617581Z Build completed unsuccessfully in 1:30:47
2020-01-23T10:26:51.0813884Z make: *** [Makefile:80: ci-subset-1] Error 1
2020-01-23T10:26:51.1447830Z   local time: Thu Jan 23 10:26:51 CUT 2020
2020-01-23T10:26:51.3834079Z   network time: Thu, 23 Jan 2020 10:26:51 GMT
2020-01-23T10:26:51.3849571Z == end clock drift check ==
2020-01-23T10:26:51.5187530Z 
2020-01-23T10:26:51.5187530Z 
2020-01-23T10:26:51.8393360Z ##[error]Bash exited with code '2'.
2020-01-23T10:26:51.8542654Z ##[section]Finishing: Run build
2020-01-23T10:26:51.8724460Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68391/merge to s
2020-01-23T10:26:51.9305031Z Task         : Get sources
2020-01-23T10:26:51.9305074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T10:26:51.9305133Z Version      : 1.0.0
2020-01-23T10:26:51.9305172Z Author       : Microsoft
2020-01-23T10:26:51.9305172Z Author       : Microsoft
2020-01-23T10:26:51.9305212Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T10:26:51.9305276Z ==============================================================================
2020-01-23T10:26:52.7042604Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T10:26:52.7128856Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68391/merge to s
2020-01-23T10:26:52.7453648Z Cleaning up task key
2020-01-23T10:26:52.7454570Z Start cleaning up orphan processes.
2020-01-23T10:26:53.0413120Z Terminate orphan process: pid (4948) (python)
2020-01-23T10:26:53.0737493Z Terminate orphan process: pid (6068) (sccache)
