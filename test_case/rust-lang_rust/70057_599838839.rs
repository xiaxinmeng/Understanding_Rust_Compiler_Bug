plain
2020-03-17T01:59:27.2929500Z 
2020-03-17T01:59:27.2929846Z failures:
2020-03-17T01:59:27.2930108Z 
2020-03-17T01:59:27.2930470Z ---- [ui] ui\parser\mod_file_not_exist_windows.rs stdout ----
2020-03-17T01:59:27.2931027Z $DIR\mod_file_not_exist_windows.rs
2020-03-17T01:59:27.2931450Z $DIR\not_a_real_file.rs
2020-03-17T01:59:27.2931793Z $DIR\mod_file_not_exist_windows.rs
2020-03-17T01:59:27.2932378Z 
2020-03-17T01:59:27.2932762Z 1 error[E0583]: file not found for module `not_a_real_file`
2020-03-17T01:59:27.2933216Z -   --> $DIR/mod_file_not_exist_windows.rs:3:5
2020-03-17T01:59:27.2933707Z +   --> $DIR/mod_file_not_exist_windows.rs:3:1
2020-03-17T01:59:27.2933707Z +   --> $DIR/mod_file_not_exist_windows.rs:3:1
2020-03-17T01:59:27.2934049Z 3    |
2020-03-17T01:59:27.2934528Z 4 LL | mod not_a_real_file;
2020-03-17T01:59:27.2934900Z -    |     ^^^^^^^^^^^^^^^
2020-03-17T01:59:27.2935232Z +    | ^^^^^^^^^^^^^^^^^^^^
2020-03-17T01:59:27.2935548Z 6    |
2020-03-17T01:59:27.2935971Z 7    = help: to create the module `not_a_real_file`, create file "$DIR/not_a_real_file.rs"
2020-03-17T01:59:27.2936625Z 
2020-03-17T01:59:27.2936954Z - error: aborting due to previous error
2020-03-17T01:59:27.2937460Z + error[E0433]: failed to resolve: use of undeclared type or module `mod_file_aux`
2020-03-17T01:59:27.2937970Z +   --> $DIR/mod_file_not_exist_windows.rs:7:16
2020-03-17T01:59:27.2937970Z +   --> $DIR/mod_file_not_exist_windows.rs:7:16
2020-03-17T01:59:27.2938337Z +    |
2020-03-17T01:59:27.2938684Z + LL |     assert_eq!(mod_file_aux::bar(), 10);
2020-03-17T01:59:27.2939194Z +    |                ^^^^^^^^^^^^ use of undeclared type or module `mod_file_aux`
2020-03-17T01:59:27.2940241Z - For more information about this error, try `rustc --explain E0583`.
2020-03-17T01:59:27.2940713Z + error: aborting due to 2 previous errors
2020-03-17T01:59:27.2941053Z + 
2020-03-17T01:59:27.2941442Z + Some errors have detailed explanations: E0433, E0583.
2020-03-17T01:59:27.2941442Z + Some errors have detailed explanations: E0433, E0583.
2020-03-17T01:59:27.2941960Z + For more information about an error, try `rustc --explain E0433`.
2020-03-17T01:59:27.2942324Z 12 
2020-03-17T01:59:27.2942560Z 
2020-03-17T01:59:27.2942766Z 
2020-03-17T01:59:27.2943099Z The actual stderr differed from the expected stderr.
2020-03-17T01:59:27.2944270Z Actual stderr saved to D:\a\1\s\build\i686-pc-windows-msvc\test\ui\parser\mod_file_not_exist_windows\mod_file_not_exist_windows.stderr
2020-03-17T01:59:27.2944941Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T01:59:27.2945482Z To only update this specific test, also pass `--test-args parser\mod_file_not_exist_windows.rs`
2020-03-17T01:59:27.2946209Z error: 1 errors occurred comparing output.
2020-03-17T01:59:27.2946593Z status: exit code: 1
2020-03-17T01:59:27.2946593Z status: exit code: 1
2020-03-17T01:59:27.2960635Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw32\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw32\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.2\externals\git\cmd;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\parser\\mod_file_not_exist_windows.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\parser\\mod_file_not_exist_windows" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-A" "unused" "-L" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\parser\\mod_file_not_exist_windows\\auxiliary"
2020-03-17T01:59:27.2970085Z ------------------------------------------
2020-03-17T01:59:27.2970278Z 
2020-03-17T01:59:27.2970513Z ------------------------------------------
2020-03-17T01:59:27.2970738Z stderr:
2020-03-17T01:59:27.2970738Z stderr:
2020-03-17T01:59:27.2970983Z ------------------------------------------
2020-03-17T01:59:27.2971420Z error[E0583]: file not found for module `not_a_real_file`
2020-03-17T01:59:27.2971913Z   --> D:\a\1\s\src/test\ui\parser\mod_file_not_exist_windows.rs:3:1
2020-03-17T01:59:27.2972215Z    |
2020-03-17T01:59:27.2972484Z LL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`
2020-03-17T01:59:27.2973383Z    |
2020-03-17T01:59:27.2973383Z    |
2020-03-17T01:59:27.2973771Z    = help: to create the module `not_a_real_file`, create file "D:\a\1\s\src/test\ui\parser\not_a_real_file.rs"
2020-03-17T01:59:27.2974564Z error[E0433]: failed to resolve: use of undeclared type or module `mod_file_aux`
2020-03-17T01:59:27.2975065Z   --> D:\a\1\s\src/test\ui\parser\mod_file_not_exist_windows.rs:7:16
2020-03-17T01:59:27.2975349Z    |
2020-03-17T01:59:27.2975349Z    |
2020-03-17T01:59:27.2975595Z LL |     assert_eq!(mod_file_aux::bar(), 10);
2020-03-17T01:59:27.2975992Z    |                ^^^^^^^^^^^^ use of undeclared type or module `mod_file_aux`
2020-03-17T01:59:27.2976445Z error: aborting due to 2 previous errors
2020-03-17T01:59:27.2976615Z 
2020-03-17T01:59:27.2976868Z Some errors have detailed explanations: E0433, E0583.
2020-03-17T01:59:27.2977215Z For more information about an error, try `rustc --explain E0433`.
---
2020-03-17T01:59:27.2979624Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:348:22
2020-03-17T01:59:27.2980093Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T01:59:27.3003079Z 
2020-03-17T01:59:27.3003266Z 
2020-03-17T01:59:27.3007983Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "ui" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T01:59:27.3014166Z 
2020-03-17T01:59:27.3014311Z 
2020-03-17T01:59:27.3653293Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2020-03-17T01:59:27.3653923Z Build completed unsuccessfully in 1:45:37
2020-03-17T01:59:27.3653923Z Build completed unsuccessfully in 1:45:37
2020-03-17T01:59:27.4075124Z make: *** [Makefile:82: ci-subset-2] Error 1
2020-03-17T01:59:27.4708060Z   local time: Tue Mar 17 01:59:27 CUT 2020
2020-03-17T01:59:27.8176952Z   network time: Tue, 17 Mar 2020 01:59:27 GMT
2020-03-17T01:59:27.8196006Z == end clock drift check ==
2020-03-17T01:59:27.9366957Z 
2020-03-17T01:59:27.9366957Z 
2020-03-17T01:59:28.2324414Z ##[error]Bash exited with code '2'.
2020-03-17T01:59:28.2928064Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-17T01:59:28.3778855Z ==============================================================================
2020-03-17T01:59:28.3779244Z Task         : Get sources
2020-03-17T01:59:28.3779645Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
