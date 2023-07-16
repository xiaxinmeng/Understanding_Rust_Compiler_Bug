plain
2019-10-18T17:36:38.8142047Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-18T17:36:38.8143090Z 
2019-10-18T17:36:38.8143666Z   git checkout -b <new-branch-name>
2019-10-18T17:36:38.8144139Z 
2019-10-18T17:36:38.8144605Z HEAD is now at 3623bcc1f Auto merge of #65565 - tmandry:rollup-s8n9bie, r=tmandry
2019-10-18T17:36:38.8523842Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-18T17:36:38.8648242Z ==============================================================================
2019-10-18T17:36:38.8648364Z Task         : Bash
2019-10-18T17:36:38.8648447Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-18T20:30:55.0447497Z failures:
2019-10-18T20:30:55.0447758Z 
2019-10-18T20:30:55.0447894Z ---- [ui] ui\coherence\impl[t]-foreign-for-(local, t).rs stdout ----
2019-10-18T20:30:55.0447955Z 
2019-10-18T20:30:55.0448207Z error: auxiliary build of "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" failed to compile: 
2019-10-18T20:30:55.0448382Z status: exit code: 1
2019-10-18T20:30:55.0450407Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\mingw64\bin;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.158.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--crate-type" "dylib" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary"
2019-10-18T20:30:55.0451869Z ------------------------------------------
2019-10-18T20:30:55.0451924Z 
2019-10-18T20:30:55.0452017Z ------------------------------------------
2019-10-18T20:30:55.0452092Z stderr:
2019-10-18T20:30:55.0452092Z stderr:
2019-10-18T20:30:55.0452180Z ------------------------------------------
2019-10-18T20:30:55.0452268Z error: linking with `gcc` failed: exit code: 1
2019-10-18T20:30:55.0452366Z    |
2019-10-18T20:30:55.0453342Z    = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\dllcrt2.o" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.coherence_lib.3a1fbbbh-cgu.0.rcgu.o" "-o" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.dll" "-Wl,--version-script=C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustc0XRuhh\\list" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.2gio4epld6q3b7cu.rcgu.o" "-nodefaultlibs" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,--start-group" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lstd-12406dc401bf420a" "-Wl,--end-group" "-Wl,-Bstatic" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustc0XRuhh\\libcompiler_builtins-c2b1964029640072.rlib" "-Wl,-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-shared" "-Wl,--out-implib,D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.dll.lib" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
2019-10-18T20:30:55.0454434Z    = note: D:/a/1/s/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/6.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find  t)\auxiliary\coherence_lib.dll.lib: No such file or directory
2019-10-18T20:30:55.0454567Z            collect2.exe: error: ld returned 1 exit status
2019-10-18T20:30:55.0454706Z 
2019-10-18T20:30:55.0454792Z error: aborting due to previous error
2019-10-18T20:30:55.0454839Z 
2019-10-18T20:30:55.0454899Z 
---
2019-10-18T20:30:55.0497549Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:537:22
2019-10-18T20:30:55.0498316Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T20:30:55.0528476Z 
2019-10-18T20:30:55.0528913Z 
2019-10-18T20:30:55.0536944Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-gnu" "--mode" "ui" "--target" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw64\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-18T20:30:55.0538643Z 
2019-10-18T20:30:55.0538687Z 
2019-10-18T20:30:55.1549883Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-10-18T20:30:55.1550722Z Build completed unsuccessfully in 2:43:35
2019-10-18T20:30:55.1550722Z Build completed unsuccessfully in 2:43:35
2019-10-18T20:30:55.2098720Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-10-18T20:30:55.2702468Z   local time: Fri Oct 18 20:30:55 CUT 2019
2019-10-18T20:30:55.9251293Z   network time: Fri, 18 Oct 2019 20:30:55 GMT
2019-10-18T20:30:55.9260613Z == end clock drift check ==
2019-10-18T20:30:56.0239078Z 
2019-10-18T20:30:56.0239078Z 
2019-10-18T20:30:56.3620117Z ##[error]Bash exited with code '2'.
2019-10-18T20:30:56.4738574Z ##[section]Starting: Upload CPU usage statistics
2019-10-18T20:30:56.5485085Z ==============================================================================
2019-10-18T20:30:56.5485220Z Task         : Bash
2019-10-18T20:30:56.5485468Z Description  : Run a Bash script on macOS, Linux, or Windows
