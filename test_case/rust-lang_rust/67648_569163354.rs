plain
2019-12-26T22:31:22.7479074Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-26T22:31:22.7479151Z 
2019-12-26T22:31:22.7479218Z   git checkout -b <new-branch-name>
2019-12-26T22:31:22.7479268Z 
2019-12-26T22:31:22.7479615Z HEAD is now at 36b595112 Auto merge of #67648 - Mark-Simulacrum:rollup-dttcu1y, r=Mark-Simulacrum
2019-12-26T22:31:22.7891988Z ##[section]Starting: Setup environment
2019-12-26T22:31:22.8014967Z ==============================================================================
2019-12-26T22:31:22.8015409Z Task         : Bash
2019-12-26T22:31:22.8015498Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-27T01:39:49.2907822Z ---- [ui] ui\backtrace.rs stdout ----
2019-12-27T01:39:49.2907887Z 
2019-12-27T01:39:49.2907948Z error: test run failed!
2019-12-27T01:39:49.2908035Z status: exit code: 101
2019-12-27T01:39:49.2916629Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\mingw32\bin;C:\Python27amd64;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.163.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files " "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui\\backtrace\\a.exe"
2019-12-27T01:39:49.2917952Z ------------------------------------------
2019-12-27T01:39:49.2918617Z 
2019-12-27T01:39:49.2918720Z ------------------------------------------
2019-12-27T01:39:49.2918792Z stderr:
2019-12-27T01:39:49.2918792Z stderr:
2019-12-27T01:39:49.2918874Z ------------------------------------------
2019-12-27T01:39:49.2918980Z thread 'main' panicked at 'bad output: thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace.rs:18:9
2019-12-27T01:39:49.2919177Z    0: _set_invalid_parameter_handler
2019-12-27T01:39:49.2919268Z    1: _set_invalid_parameter_handler
2019-12-27T01:39:49.2919355Z    2: _set_invalid_parameter_handler
2019-12-27T01:39:49.2919427Z    3: _set_invalid_parameter_handler
---
2019-12-27T01:39:49.2920163Z    8: _set_invalid_parameter_handler
2019-12-27T01:39:49.2920234Z    9: _set_invalid_parameter_handler
2019-12-27T01:39:49.2920325Z   10: _set_invalid_parameter_handler
2019-12-27T01:39:49.2920410Z   11: main
2019-12-27T01:39:49.2920473Z   12: __tmainCRTStartup
2019-12-27T01:39:49.2920687Z   13: _set_invalid_parameter_handler
2019-12-27T01:39:49.2921032Z   14: _set_invalid_parameter_handler
2019-12-27T01:39:49.2921212Z   15: _set_invalid_parameter_handler
2019-12-27T01:39:49.2921498Z note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
2019-12-27T01:39:49.2921692Z ', D:\a\1\s\src/test\ui\backtrace.rs:68:5
2019-12-27T01:39:49.2922128Z 
2019-12-27T01:39:49.2922293Z ------------------------------------------
2019-12-27T01:39:49.2922419Z 
2019-12-27T01:39:49.2922533Z 
---
2019-12-27T01:39:49.2946516Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:385:22
2019-12-27T01:39:49.2947089Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-27T01:39:49.2981330Z 
2019-12-27T01:39:49.2981453Z 
2019-12-27T01:39:49.2982069Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "ui" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-27T01:39:49.2982636Z 
2019-12-27T01:39:49.2982691Z 
2019-12-27T01:39:49.3687389Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-12-27T01:39:49.3687614Z Build completed unsuccessfully in 2:58:29
2019-12-27T01:39:49.3687614Z Build completed unsuccessfully in 2:58:29
2019-12-27T01:39:49.4130140Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-12-27T01:39:49.4751779Z   local time: Fri Dec 27 01:39:49 CUT 2019
2019-12-27T01:39:50.1157635Z   network time: Fri, 27 Dec 2019 01:39:50 GMT
2019-12-27T01:39:50.1157884Z == end clock drift check ==
2019-12-27T01:39:50.2120740Z 
2019-12-27T01:39:50.2120740Z 
2019-12-27T01:39:50.5361189Z ##[error]Bash exited with code '2'.
2019-12-27T01:39:50.6016339Z ##[section]Starting: Checkout
2019-12-27T01:39:50.6742905Z ==============================================================================
2019-12-27T01:39:50.6743059Z Task         : Get sources
2019-12-27T01:39:50.6743156Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
