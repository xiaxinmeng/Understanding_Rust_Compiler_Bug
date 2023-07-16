plain
2020-03-17T10:47:48.7026221Z test [ui] ui\hygiene\intercrate.rs ... ok
2020-03-17T10:47:48.7834760Z test [ui] ui\hygiene\lexical.rs ... ok
2020-03-17T10:47:49.1245079Z test [ui] ui\hygiene\legacy_interaction.rs ... ok
2020-03-17T10:47:49.2713840Z test [ui] ui\hygiene\local_inner_macros.rs ... ok
2020-03-17T10:47:49.5509760Z test [ui] ui\hygiene\macro-metavars-legacy.rs ... ok
2020-03-17T10:47:49.7156349Z test [ui] ui\hygiene\macro-metavars-transparent.rs ... ok
2020-03-17T10:47:49.7299207Z test [ui] ui\hygiene\no_implicit_prelude-2018.rs ... ok
2020-03-17T10:47:49.7928151Z test [ui] ui\hygiene\pattern-macro.rs ... ok
2020-03-17T10:47:49.8129414Z test [ui] ui\hygiene\no_implicit_prelude.rs ... ok
---
2020-03-17T11:10:30.7591220Z 
2020-03-17T11:10:30.7591351Z failures:
2020-03-17T11:10:30.7591478Z 
2020-03-17T11:10:30.7591695Z ---- [ui] ui\parser\mod_file_not_exist_windows.rs stdout ----
2020-03-17T11:10:30.7591985Z $DIR\mod_file_not_exist_windows.rs
2020-03-17T11:10:30.7592192Z $DIR\not_a_real_file.rs
2020-03-17T11:10:30.7592423Z $DIR\mod_file_not_exist_windows.rs
2020-03-17T11:10:30.7592758Z 
2020-03-17T11:10:30.7593139Z 1 error[E0583]: file not found for module `not_a_real_file`
2020-03-17T11:10:30.7593489Z -   --> $DIR/mod_file_not_exist_windows.rs:3:5
2020-03-17T11:10:30.7593779Z +   --> $DIR/mod_file_not_exist_windows.rs:3:1
2020-03-17T11:10:30.7593779Z +   --> $DIR/mod_file_not_exist_windows.rs:3:1
2020-03-17T11:10:30.7594024Z 3    |
2020-03-17T11:10:30.7594191Z 4 LL | mod not_a_real_file;
2020-03-17T11:10:30.7594415Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-17T11:10:30.7594551Z 
2020-03-17T11:10:30.7594659Z 
2020-03-17T11:10:30.7594861Z The actual stderr differed from the expected stderr.
2020-03-17T11:10:30.7595316Z Actual stderr saved to D:\a\1\s\build\i686-pc-windows-msvc\test\ui\parser\mod_file_not_exist_windows\mod_file_not_exist_windows.stderr
2020-03-17T11:10:30.7595773Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T11:10:30.7596181Z To only update this specific test, also pass `--test-args parser\mod_file_not_exist_windows.rs`
2020-03-17T11:10:30.7596634Z error: 1 errors occurred comparing output.
2020-03-17T11:10:30.7596864Z status: exit code: 1
2020-03-17T11:10:30.7596864Z status: exit code: 1
2020-03-17T11:10:30.7624151Z command: PATH="D:\a\1\s\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\1\s\build\i686-pc-windows-msvc\stage0\bin;D:\a\1\s\msys2\mingw32\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw32\bin;C:\Python27amd64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.165.2\externals\git\cmd;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.7\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQ" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\parser\\mod_file_not_exist_windows.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\parser\\mod_file_not_exist_windows" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-A" "unused" "-L" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui\\parser\\mod_file_not_exist_windows\\auxiliary"
2020-03-17T11:10:30.7631639Z ------------------------------------------
2020-03-17T11:10:30.7631815Z 
2020-03-17T11:10:30.7632036Z ------------------------------------------
2020-03-17T11:10:30.7632243Z stderr:
2020-03-17T11:10:30.7632243Z stderr:
2020-03-17T11:10:30.7632471Z ------------------------------------------
2020-03-17T11:10:30.7632772Z error[E0583]: file not found for module `not_a_real_file`
2020-03-17T11:10:30.7633172Z   --> D:\a\1\s\src/test\ui\parser\mod_file_not_exist_windows.rs:3:1
2020-03-17T11:10:30.7633446Z    |
2020-03-17T11:10:30.7633740Z LL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`
2020-03-17T11:10:30.7634245Z    |
2020-03-17T11:10:30.7634245Z    |
2020-03-17T11:10:30.7634632Z    = help: to create the module `not_a_real_file`, create file "D:\a\1\s\src/test\ui\parser\not_a_real_file.rs"
2020-03-17T11:10:30.7635263Z error[E0433]: failed to resolve: use of undeclared type or module `mod_file_aux`
2020-03-17T11:10:30.7635692Z   --> D:\a\1\s\src/test\ui\parser\mod_file_not_exist_windows.rs:7:16
2020-03-17T11:10:30.7635990Z    |
2020-03-17T11:10:30.7635990Z    |
2020-03-17T11:10:30.7636213Z LL |     assert_eq!(mod_file_aux::bar(), 10);
2020-03-17T11:10:30.7636594Z    |                ^^^^^^^^^^^^ use of undeclared type or module `mod_file_aux`
2020-03-17T11:10:30.7637131Z error: aborting due to 2 previous errors
2020-03-17T11:10:30.7637293Z 
2020-03-17T11:10:30.7637531Z Some errors have detailed explanations: E0433, E0583.
2020-03-17T11:10:30.7637858Z For more information about an error, try `rustc --explain E0433`.
---
2020-03-17T11:10:30.7640301Z test result: FAILED. 9694 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out
2020-03-17T11:10:30.7640538Z 
2020-03-17T11:10:30.7640649Z 
2020-03-17T11:10:30.7640739Z 
2020-03-17T11:10:30.7652823Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "ui" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\python2.7" "--gdb" "D:\\a\\1\\s\\msys2\\mingw32\\bin\\gdb" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T11:10:30.7656266Z 
2020-03-17T11:10:30.7656360Z 
2020-03-17T11:10:30.8095218Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
2020-03-17T11:10:30.8095642Z Build completed unsuccessfully in 1:52:43
2020-03-17T11:10:30.8095642Z Build completed unsuccessfully in 1:52:43
2020-03-17T11:10:30.8530028Z make: *** [Makefile:82: ci-subset-2] Error 1
2020-03-17T11:10:30.9300340Z   local time: Tue Mar 17 11:10:30 CUT 2020
2020-03-17T11:10:31.3446323Z   network time: Tue, 17 Mar 2020 11:10:31 GMT
2020-03-17T11:10:31.3460167Z == end clock drift check ==
2020-03-17T11:10:31.4336514Z 
2020-03-17T11:10:31.4336514Z 
2020-03-17T11:10:31.7616982Z ##[error]Bash exited with code '2'.
2020-03-17T11:10:31.8179574Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-17T11:10:31.9306807Z ==============================================================================
2020-03-17T11:10:31.9307295Z Task         : Get sources
2020-03-17T11:10:31.9308126Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
