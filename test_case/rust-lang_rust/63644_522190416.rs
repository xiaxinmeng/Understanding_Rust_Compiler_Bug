plain
2019-08-16T22:13:02.1014161Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T22:13:02.1014356Z 
2019-08-16T22:13:02.1014596Z   git checkout -b <new-branch-name>
2019-08-16T22:13:02.1014783Z 
2019-08-16T22:13:02.1015024Z HEAD is now at 8f517b54e Auto merge of #63644 - Centril:rollup-kapx76n, r=Centril
2019-08-16T22:13:02.1366837Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T22:13:02.1463436Z ==============================================================================
2019-08-16T22:13:02.1463648Z Task         : Bash
2019-08-16T22:13:02.1463846Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T00:21:54.2784733Z test [ui] ui\command-exec.rs ... ignored
2019-08-17T00:21:54.3707287Z test [ui] ui\command-line-diagnostics.rs ... ok
2019-08-17T00:21:54.3707477Z test [ui] ui\command-pre-exec.rs ... ignored
2019-08-17T00:21:54.7000760Z test [ui] ui\coherence\re-rebalance-coherence.rs ... ok
2019-08-17T00:21:54.7426094Z test [ui] ui\commandline-argfile-badutf8.rs ... ok
2019-08-17T00:21:54.7638144Z test [ui] ui\commandline-argfile-missing.rs ... FAILED
2019-08-17T00:21:54.8585328Z test [ui] ui\compare-method\proj-outlives-region.rs ... ok
2019-08-17T00:21:54.9505759Z test [ui] ui\compare-method\region-extra-2.rs ... ok
2019-08-17T00:21:55.0265542Z test [ui] ui\compare-method\region-extra.rs ... ok
2019-08-17T00:21:55.1042146Z test [ui] ui\compare-method\region-unrelated.rs ... ok
---
2019-08-17T00:54:31.6615026Z 
2019-08-17T00:54:31.6615118Z ---- [ui] ui\commandline-argfile-missing.rs stdout ----
2019-08-17T00:54:31.6615204Z diff of stderr:
2019-08-17T00:54:31.6615241Z 
2019-08-17T00:54:31.6615341Z - error: Argument $N is not valid: IO Error: $DIR/commandline-argfile-missing.args: No such file or directory (os error $ERR)
2019-08-17T00:54:31.6615453Z + error: Argument $N is not valid: IO Error: $DIR/commandline-argfile-missing.args: The system cannot find the file specified. (os error $ERR)
2019-08-17T00:54:31.6616092Z 3 
2019-08-17T00:54:31.6616127Z 
2019-08-17T00:54:31.6616179Z 
2019-08-17T00:54:31.6616248Z The actual stderr differed from the expected stderr.
2019-08-17T00:54:31.6616248Z The actual stderr differed from the expected stderr.
2019-08-17T00:54:31.6616374Z Actual stderr saved to D:\a\1\s\build\x86_64-pc-windows-msvc\test\ui\commandline-argfile-missing\commandline-argfile-missing.stderr
2019-08-17T00:54:31.6616483Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T00:54:31.6616765Z To only update this specific test, also pass `--test-args commandline-argfile-missing.rs`
2019-08-17T00:54:31.6616920Z error: 1 errors occurred comparing output.
2019-08-17T00:54:31.6616991Z status: exit code: 1
2019-08-17T00:54:31.6616991Z status: exit code: 1
2019-08-17T00:54:31.6618382Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\citools\msys64\mingw64\bin;D:\a\1\s\citools\msys64\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.155.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.3\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\commandline-argfile-missing.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\commandline-argfile-missing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--cfg" "cmdline_set" "@D:\\a\\1\\s\\src/test\\ui/commandline-argfile-missing.args" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\commandline-argfile-missing\\auxiliary" "-A" "unused"
2019-08-17T00:54:31.6619832Z ------------------------------------------
2019-08-17T00:54:31.6619874Z 
2019-08-17T00:54:31.6619947Z ------------------------------------------
2019-08-17T00:54:31.6620007Z stderr:
2019-08-17T00:54:31.6620007Z stderr:
2019-08-17T00:54:31.6620077Z ------------------------------------------
2019-08-17T00:54:31.6620172Z error: Argument 18 is not valid: IO Error: D:\a\1\s\src/test\ui/commandline-argfile-missing.args: The system cannot find the file specified. (os error 2)
2019-08-17T00:54:31.6620281Z 
2019-08-17T00:54:31.6620425Z ------------------------------------------
2019-08-17T00:54:31.6620528Z 
2019-08-17T00:54:31.6620559Z 
---
2019-08-17T00:54:31.6650688Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:536:22
2019-08-17T00:54:31.6650844Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-17T00:54:31.6691097Z 
2019-08-17T00:54:31.6691291Z 
2019-08-17T00:54:31.6696121Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--mode" "ui" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\citools\\msys64\\mingw64\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-17T00:54:31.6696786Z 
2019-08-17T00:54:31.7590797Z 
2019-08-17T00:54:31.7591121Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2019-08-17T00:54:31.7591213Z Build completed unsuccessfully in 2:29:06
2019-08-17T00:54:31.7591213Z Build completed unsuccessfully in 2:29:06
2019-08-17T00:54:31.7864390Z make: *** [Makefile:82: ci-subset-2] Error 1
2019-08-17T00:54:31.8380120Z   local time: Sat Aug 17 00:54:31 CUT 2019
2019-08-17T00:54:32.0730638Z   network time: Sat, 17 Aug 2019 00:54:32 GMT
2019-08-17T00:54:32.0756547Z == end clock drift check ==
2019-08-17T00:54:32.0756547Z == end clock drift check ==
2019-08-17T00:54:32.4710388Z ##[error]Bash exited with code '2'.
2019-08-17T00:54:32.5127990Z ##[section]Starting: Upload CPU usage statistics
2019-08-17T00:54:32.6554867Z ==============================================================================
2019-08-17T00:54:32.6554966Z Task         : Bash
2019-08-17T00:54:32.6555054Z Description  : Run a Bash script on macOS, Linux, or Windows
