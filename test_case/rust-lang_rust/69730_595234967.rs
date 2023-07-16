plain
2020-03-05T13:38:59.2197677Z 
2020-03-05T13:38:59.2198220Z failures:
2020-03-05T13:38:59.2198594Z 
2020-03-05T13:38:59.2199115Z ---- [ui] ui\parser\mod_file_not_exist_windows.rs stdout ----
2020-03-05T13:38:59.2201818Z $DIR\mod_file_not_exist_windows.rs
2020-03-05T13:38:59.2202544Z $DIR\not_a_real_file.rs
2020-03-05T13:38:59.2203391Z 
2020-03-05T13:38:59.2204956Z error: D:/a/1/s/src/test/ui/parser/mod_file_not_exist_windows.rs:3: unexpected help message: '3:5: 3:20: to create the module `not_a_real_file`, create file "D:\a\1\s\src/test\ui\parser\not_a_real_file.rs"'
2020-03-05T13:38:59.2205885Z 
2020-03-05T13:38:59.2207360Z error: D:/a/1/s/src/test/ui/parser/mod_file_not_exist_windows.rs:3: expected help message not found: name the file either not_a_real_file.rs or not_a_real_file\mod.rs inside the directory
2020-03-05T13:38:59.2209771Z error: 1 unexpected errors found, 1 expected errors not found
2020-03-05T13:38:59.2210304Z status: exit code: 1
2020-03-05T13:38:59.2210304Z status: exit code: 1
2020-03-05T13:38:59.2224153Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw32\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw32\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.0\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\parser\\mod_file_not_exist_windows.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\parser\\mod_file_not_exist_windows" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-A" "unused" "-L" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\parser\\mod_file_not_exist_windows\\auxiliary"
2020-03-05T13:38:59.2233055Z     Error {
2020-03-05T13:38:59.2233238Z         line_num: 3,
2020-03-05T13:38:59.2233467Z         kind: Some(
2020-03-05T13:38:59.2233674Z             Help,
2020-03-05T13:38:59.2233674Z             Help,
2020-03-05T13:38:59.2233839Z         ),
2020-03-05T13:38:59.2234274Z         msg: "3:5: 3:20: to create the module `not_a_real_file`, create file \"D:\\a\\1\\s\\src/test\\ui\\parser\\not_a_real_file.rs\"",
2020-03-05T13:38:59.2235182Z ]
2020-03-05T13:38:59.2235443Z 
2020-03-05T13:38:59.2235989Z not found errors (from test file): [
2020-03-05T13:38:59.2236196Z     Error {
2020-03-05T13:38:59.2236196Z     Error {
2020-03-05T13:38:59.2236393Z         line_num: 3,
2020-03-05T13:38:59.2236586Z         kind: Some(
2020-03-05T13:38:59.2236791Z             Help,
2020-03-05T13:38:59.2236951Z         ),
2020-03-05T13:38:59.2237283Z         msg: "name the file either not_a_real_file.rs or not_a_real_file\\mod.rs inside the directory",
2020-03-05T13:38:59.2237730Z ]
2020-03-05T13:38:59.2237827Z 
2020-03-05T13:38:59.2238364Z thread '[ui] ui\parser\mod_file_not_exist_windows.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:1436:13
2020-03-05T13:38:59.2238896Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
2020-03-05T13:38:59.2241206Z 
2020-03-05T13:38:59.2241938Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:348:22
2020-03-05T13:38:59.2279094Z 
2020-03-05T13:38:59.2279292Z 
2020-03-05T13:38:59.2283506Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "ui" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-05T13:38:59.2288207Z 
2020-03-05T13:38:59.2288325Z 
2020-03-05T13:38:59.3009629Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2020-03-05T13:38:59.3010181Z Build completed unsuccessfully in 1:52:22
2020-03-05T13:38:59.3010181Z Build completed unsuccessfully in 1:52:22
2020-03-05T13:38:59.3560382Z make: *** [Makefile:82: ci-subset-2] Error 1
2020-03-05T13:38:59.4199436Z   local time: Thu Mar  5 13:38:59 CUT 2020
2020-03-05T13:38:59.8796848Z   network time: Thu, 05 Mar 2020 13:38:59 GMT
2020-03-05T13:38:59.8848537Z == end clock drift check ==
2020-03-05T13:38:59.9818570Z 
2020-03-05T13:38:59.9818570Z 
2020-03-05T13:39:00.2625657Z ##[error]Bash exited with code '2'.
2020-03-05T13:39:00.3304042Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-05T13:39:00.4407038Z ==============================================================================
2020-03-05T13:39:00.4407484Z Task         : Get sources
2020-03-05T13:39:00.4407873Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
