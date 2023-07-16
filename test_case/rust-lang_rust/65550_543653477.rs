plain
2019-10-18T07:23:36.8807076Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-18T07:23:36.8807177Z 
2019-10-18T07:23:36.8809050Z   git checkout -b <new-branch-name>
2019-10-18T07:23:36.8809434Z 
2019-10-18T07:23:36.8809903Z HEAD is now at 027776544 Auto merge of #65550 - tmandry:rollup-ft7n1b3, r=tmandry
2019-10-18T07:23:36.9257109Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-18T07:23:36.9373279Z ==============================================================================
2019-10-18T07:23:36.9373407Z Task         : Bash
2019-10-18T07:23:36.9373492Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-18T09:37:23.3089712Z test [ui] ui\coherence\coherence_local_err_tuple.rs#old ... ok
2019-10-18T09:37:23.6716707Z test [ui] ui\coherence\coherence_local_ref.rs#old ... ok
2019-10-18T09:37:23.6966434Z test [ui] ui\coherence\coherence_local_ref.rs#re ... ok
2019-10-18T09:37:30.1754991Z test [ui] ui\coherence\conflicting-impl-with-err.rs ... ok
2019-10-18T09:37:30.1756592Z test [ui] ui\coherence\impl-foreign-for-foreign.rs ... ok
2019-10-18T09:37:30.1756894Z test [ui] ui\coherence\impl-foreign-for-foreign[foreign].rs ... ok
2019-10-18T09:37:30.1757119Z test [ui] ui\coherence\impl-foreign-for-foreign[local].rs ... ok
2019-10-18T09:37:30.1757350Z test [ui] ui\coherence\impl-foreign-for-fundamental[foreign].rs ... ok
2019-10-18T09:37:30.1757569Z test [ui] ui\coherence\impl-foreign-for-fundamental[local].rs ... ok
2019-10-18T09:37:30.1758048Z test [ui] ui\coherence\impl-foreign-for-local.rs ... ok
2019-10-18T09:37:30.1758549Z test [ui] ui\coherence\impl-foreign[foreign]-for-local.rs ... ok
2019-10-18T09:37:30.1758549Z test [ui] ui\coherence\impl-foreign[foreign]-for-local.rs ... ok
2019-10-18T09:37:30.1758770Z test [ui] ui\coherence\impl-foreign[fundemental[foreign]]-for-foreign.rs ... ok
2019-10-18T09:37:30.1758977Z test [ui] ui\coherence\impl-foreign[fundemental[local]]-for-foreign.rs ... ok
2019-10-18T09:37:30.1759197Z test [ui] ui\coherence\impl[t]-foreign-for-(local, t).rs ... FAILED
2019-10-18T09:37:30.1759391Z test [ui] ui\coherence\impl[t]-foreign-for-foreign[t].rs ... ok
2019-10-18T09:37:30.1759604Z test [ui] ui\coherence\impl[t]-foreign-for-fundamental[t].rs ... ok
2019-10-18T09:37:30.1760019Z test [ui] ui\coherence\impl[t]-foreign[foreign]-for-fundamental[t].rs ... ok
2019-10-18T09:37:30.1760235Z test [ui] ui\coherence\impl[t]-foreign[foreign]-for-t.rs ... ok
2019-10-18T09:37:30.1760462Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-foreign.rs ... ok
2019-10-18T09:37:30.1760671Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-10-18T09:37:30.1760671Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-10-18T09:37:30.1760895Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-local.rs ... ok
2019-10-18T09:37:30.1761090Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-t.rs ... ok
2019-10-18T09:37:30.1761308Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]_local]-for-foreign.rs ... ok
2019-10-18T09:37:30.1761530Z test [ui] ui\coherence\impl[t]-foreign[fundemental[local]]-for-foreign[t].rs ... ok
2019-10-18T09:37:30.1761941Z test [ui] ui\coherence\impl[t]-foreign[local]-for-foreign[t].rs ... ok
2019-10-18T09:37:30.1761941Z test [ui] ui\coherence\impl[t]-foreign[local]-for-foreign[t].rs ... ok
2019-10-18T09:37:30.1762159Z test [ui] ui\coherence\impl[t]-foreign[local]-for-fundamental[foreign[t]].rs ... ok
2019-10-18T09:37:30.1762963Z test [ui] ui\coherence\impl[t]-foreign[local]-for-local.rs ... ok
2019-10-18T09:37:30.1763222Z test [ui] ui\coherence\impl[t]-foreign[local]-for-t.rs ... ok
2019-10-18T09:37:30.1763425Z test [ui] ui\coherence\impl[t]-foreign[local_fundamental[t]]-for-foreign.rs ... ok
2019-10-18T09:37:30.1763646Z test [ui] ui\coherence\impl[t]-foreign[t]-for-foreign.rs ... ok
---
2019-10-18T10:12:17.4479179Z test [ui] ui\zero-sized\zero-sized-vec-push.rs ... ok
2019-10-18T10:12:17.4484657Z 
2019-10-18T10:12:17.4485008Z failures:
2019-10-18T10:12:17.4485501Z 
2019-10-18T10:12:17.4485767Z ---- [ui] ui\coherence\impl[t]-foreign-for-(local, t).rs stdout ----
2019-10-18T10:12:17.4485927Z 
2019-10-18T10:12:17.4486156Z error: auxiliary build of "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" failed to compile: 
2019-10-18T10:12:17.4486387Z status: exit code: 1
2019-10-18T10:12:17.4488328Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\mingw64\bin;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.158.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--crate-type" "dylib" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary"
2019-10-18T10:12:17.4490475Z ------------------------------------------
2019-10-18T10:12:17.4490648Z 
2019-10-18T10:12:17.4490830Z ------------------------------------------
2019-10-18T10:12:17.4491039Z stderr:
2019-10-18T10:12:17.4491039Z stderr:
2019-10-18T10:12:17.4491234Z ------------------------------------------
2019-10-18T10:12:17.4491428Z error: linking with `gcc` failed: exit code: 1
2019-10-18T10:12:17.4491622Z    |
2019-10-18T10:12:17.4492893Z    = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\dllcrt2.o" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.coherence_lib.3a1fbbbh-cgu.0.rcgu.o" "-o" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.dll" "-Wl,--version-script=C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcXDevNb\\list" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.2gio4epld6q3b7cu.rcgu.o" "-nodefaultlibs" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,--start-group" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lstd-12406dc401bf420a" "-Wl,--end-group" "-Wl,-Bstatic" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcXDevNb\\libcompiler_builtins-c2b1964029640072.rlib" "-Wl,-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-shared" "-Wl,--out-implib,D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign-for-(local, t)\\auxiliary\\coherence_lib.dll.lib" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
2019-10-18T10:12:17.4494040Z    = note: D:/a/1/s/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/6.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find  t)\auxiliary\coherence_lib.dll.lib: No such file or directory
2019-10-18T10:12:17.4494297Z            collect2.exe: error: ld returned 1 exit status
2019-10-18T10:12:17.4494646Z 
2019-10-18T10:12:17.4494814Z error: aborting due to previous error
2019-10-18T10:12:17.4495255Z 
2019-10-18T10:12:17.4495399Z 
2019-10-18T10:12:17.4495399Z 
2019-10-18T10:12:17.4495596Z ------------------------------------------
2019-10-18T10:12:17.4495893Z 
2019-10-18T10:12:17.4496190Z 
2019-10-18T10:12:17.4496453Z 
2019-10-18T10:12:17.4496632Z failures:
2019-10-18T10:12:17.4496834Z     [ui] ui\coherence\impl[t]-foreign-for-(local, t).rs
2019-10-18T10:12:17.4497238Z test result: FAILED. 9150 passed; 1 failed; 59 ignored; 0 measured; 0 filtered out
2019-10-18T10:12:17.4497432Z 
2019-10-18T10:12:17.4520605Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:537:22
2019-10-18T10:12:17.4521952Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T10:12:17.4521952Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T10:12:17.4577510Z 
2019-10-18T10:12:17.4577837Z 
2019-10-18T10:12:17.4580111Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-gnu" "--mode" "ui" "--target" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw64\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-18T10:12:17.4580876Z 
2019-10-18T10:12:17.4580950Z 
2019-10-18T10:12:17.7530064Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-10-18T10:12:17.7530377Z Build completed unsuccessfully in 2:38:17
2019-10-18T10:12:17.7530377Z Build completed unsuccessfully in 2:38:17
2019-10-18T10:12:17.8022660Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-10-18T10:12:17.8726693Z   local time: Fri Oct 18 10:12:17 CUT 2019
2019-10-18T10:12:18.5318208Z   network time: Fri, 18 Oct 2019 10:12:18 GMT
2019-10-18T10:12:18.5333899Z == end clock drift check ==
2019-10-18T10:12:18.6725951Z 
2019-10-18T10:12:18.6725951Z 
2019-10-18T10:12:19.2006927Z ##[error]Bash exited with code '2'.
2019-10-18T10:12:19.2646013Z ##[section]Starting: Upload CPU usage statistics
2019-10-18T10:12:19.3512340Z ==============================================================================
2019-10-18T10:12:19.3512503Z Task         : Bash
2019-10-18T10:12:19.3512619Z Description  : Run a Bash script on macOS, Linux, or Windows
