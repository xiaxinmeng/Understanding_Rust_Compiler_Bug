plain
test [compile-fail] compile-fail\intrinsics\exact_div2.rs ... ok
test [compile-fail] compile-fail\intrinsics\div-by-zero.rs ... ok
test [compile-fail] compile-fail\intrinsics\exact_div3.rs ... ok

error: tests/compile-fail/intrinsics/exact_div4.rs:4: unexpected error: '4:14: 4:54: Undefined Behavior: overflow in signed remainder (dividing MIN by -1)'

error: tests/compile-fail/intrinsics/exact_div4.rs:4: expected error not found: result of dividing MIN by -1 cannot be represented

stack backtrace:
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\curl-sys-5f510ad16bdf457b\out\build;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libnghttp2-sys-d9fa52fb17150f3c\out\i\lib;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libz-sys-f120e7dd9baee57b\out\lib;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.4\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools-bin\\miri.exe" "tests/compile-fail\\intrinsics\\exact_div4.rs" "-L" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\compiletestu0izvg" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "-C" "prefer-dynamic" "-o" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\compiletestu0izvg\\intrinsics\\exact_div4.stage-id.exe" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "C:\\Users\\runneradmin\\AppData\\Local\\rust-lang\\miri\\cache\\HOST" "-L" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\compiletestu0izvg\\intrinsics\\exact_div4.stage-id.aux"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:14: 4:54: Undefined Behavior: overflow in signed remainder (dividing MIN by -1)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "result of dividing MIN by -1 cannot be represented",
]

   0: std::panicking::begin_panic::<&str>
test [compile-fail] compile-fail\intrinsics\exact_div4.rs ... FAILED
---
test [compile-fail] compile-fail\intrinsics\unchecked_mul1.rs ... ok
test [compile-fail] compile-fail\intrinsics\unchecked_sub1.rs ... ok
test [compile-fail] compile-fail\intrinsics\unchecked_mul2.rs ... ok

error: tests/compile-fail/intrinsics/unchecked_div1.rs:4: unexpected error: '4:14: 4:58: Undefined Behavior: overflow in signed division (dividing MIN by -1)'
stack backtrace:


error: tests/compile-fail/intrinsics/unchecked_div1.rs:4: expected error not found: overflow executing `unchecked_div`
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\curl-sys-5f510ad16bdf457b\out\build;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libnghttp2-sys-d9fa52fb17150f3c\out\i\lib;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libz-sys-f120e7dd9baee57b\out\lib;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.4\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools-bin\\miri.exe" "tests/compile-fail\\intrinsics\\unchecked_div1.rs" "-L" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\compiletestu0izvg" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "-C" "prefer-dynamic" "-o" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\compiletestu0izvg\\intrinsics\\unchecked_div1.stage-id.exe" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "C:\\Users\\runneradmin\\AppData\\Local\\rust-lang\\miri\\cache\\HOST" "-L" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\compiletestu0izvg\\intrinsics\\unchecked_div1.stage-id.aux"
   1: compiletest_rs::runtest::run
   2: compiletest_rs::runtest::run
   3: core::ptr::drop_in_place::<compiletest_rs::header::TestProps>
   4: tester::run_test
---
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "4:14: 4:58: Undefined Behavior: overflow in signed division (dividing MIN by -1)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "overflow executing `unchecked_div`",
]

test [compile-fail] compile-fail\intrinsics\unchecked_sub2.rs ... ok
test [compile-fail] compile-fail\intrinsics\unchecked_div1.rs ... FAILED
---

---- compile_test stdout ----
diff of stderr:

-error: this arithmetic operation will overflow
+error: this operation will panic at runtime
    |
    |
 LL |     i32::MIN % (-1); // also caught by rustc
    |     ^^^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
    |
-   = note: `#[deny(arithmetic_overflow)]` on by default
+   = note: `#[deny(unconditional_panic)]` on by default
 
-error: this arithmetic operation will overflow
+error: this operation will panic at runtime
    |
    |
 LL |     INT_MIN % NEG_ONE; // also caught by rustc
    |     ^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
 
-error: this arithmetic operation will overflow
+error: this operation will panic at runtime
    |
    |
 LL |     INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc
    |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
 
 error: any number modulo 1 will be 0
    |
 LL |     10 % 1;
    |     ^^^^^^
    |
    |
    = note: `-D clippy::modulo-one` implied by `-D warnings`
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
 LL |     10 % -1;
    |     ^^^^^^^
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     i32::MIN % (-1); // also caught by rustc
 
 
 error: the operation is ineffective. Consider reducing it to `1`
    |
    |
 LL |     const ONE: u32 = 1 * 1;
    |
    |
    = note: `-D clippy::identity-op` implied by `-D warnings`
 
 error: any number modulo 1 will be 0
    |
 LL |     2 % ONE;
    |     ^^^^^^^
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     2 % NEG_ONE;
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     INT_MIN % NEG_ONE; // also caught by rustc
 
 error: aborting due to 10 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\test\ui\modulo_one.stage-id.stderr
To only update this specific test, also pass `--test-args modulo_one.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\curl-sys-5f510ad16bdf457b\out\build;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libnghttp2-sys-d9fa52fb17150f3c\out\i\lib;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libz-sys-f120e7dd9baee57b\out\lib;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.4\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\clippy-driver.exe" "tests\\ui\\modulo_one.rs" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\test\\ui" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "-C" "prefer-dynamic" "-o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\test\\ui\\modulo_one.stage-id.exe" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps" "-L" "dependency=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\release\\deps" "--extern" "derive_new=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\release\\deps\\derive_new-06b3c63c1a9d98fa.dll" "--extern" "clippy_utils=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libclippy_utils-af779d922a6b0756.rlib" "--extern" "if_chain=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libif_chain-631a7ebc9c29c77e.rlib" "--extern" "serde_derive=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\release\\deps\\serde_derive-879bb931e18035b7.dll" "--extern" "futures=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libfutures-ef040b2386604a0f.rlib" "--extern" "quote=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libquote-c8364a6f919a44b7.rlib" "--extern" "tokio=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libtokio-b1a93a4baff938e4.rlib" "--extern" "parking_lot=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libparking_lot-d3763b9f16d43257.rlib" "--extern" "itertools=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libitertools-dc55775f0651ad4f.rlib" "--extern" "serde=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libserde-b7423c2c586bddb4.rlib" "--extern" "syn=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libsyn-b85fff01ed8d5fb6.rlib" "--extern" "regex=\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libregex-c1da75f126c4b24c.rlib" "--edition=2021" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\test\\ui\\modulo_one.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":214,"byte_end":229,"line_start":11,"line_end":11,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    i32::MIN % (-1); // also caught by rustc","highlight_start":5,"highlight_end":20}],"label":"attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(unconditional_panic)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this operation will panic at runtime\n  --> tests\\ui\\modulo_one.rs:11:5\n   |\nLL |     i32::MIN % (-1); // also caught by rustc\n   |     ^^^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow\n   |\n   = note: `#[deny(unconditional_panic)]` on by default\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":474,"byte_end":491,"line_start":21,"line_end":21,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    INT_MIN % NEG_ONE; // also caught by rustc","highlight_start":5,"highlight_end":22}],"label":"attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests\\ui\\modulo_one.rs:21:5\n   |\nLL |     INT_MIN % NEG_ONE; // also caught by rustc\n   |     ^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":521,"byte_end":545,"line_start":22,"line_end":22,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc","highlight_start":5,"highlight_end":29}],"label":"attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests\\ui\\modulo_one.rs:22:5\n   |\nLL |     INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow\n\n"}
{"message":"any number modulo 1 will be 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":177,"byte_end":183,"line_start":8,"line_end":8,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    10 % 1;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::modulo-one` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any number modulo 1 will be 0\n  --> tests\\ui\\modulo_one.rs:8:5\n   |\nLL |     10 % 1;\n   |     ^^^^^^\n   |\n   = note: `-D clippy::modulo-one` implied by `-D warnings`\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":189,"byte_end":196,"line_start":9,"line_end":9,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    10 % -1;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests\\ui\\modulo_one.rs:9:5\n   |\nLL |     10 % -1;\n   |     ^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":214,"byte_end":229,"line_start":11,"line_end":11,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    i32::MIN % (-1); // also caught by rustc","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests\\ui\\modulo_one.rs:11:5\n   |\nLL |     i32::MIN % (-1); // also caught by rustc\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"the operation is ineffective. Consider reducing it to `1`","code":{"code":"clippy::identity_op","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":277,"byte_end":282,"line_start":13,"line_end":13,"column_start":22,"column_end":27,"is_primary":true,"text":[{"text":"    const ONE: u32 = 1 * 1;","highlight_start":22,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::identity-op` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the operation is ineffective. Consider reducing it to `1`\n  --> tests\\ui\\modulo_one.rs:13:22\n   |\nLL |     const ONE: u32 = 1 * 1;\n   |                      ^^^^^\n   |\n   = note: `-D clippy::identity-op` implied by `-D warnings`\n\n"}
{"message":"any number modulo 1 will be 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":356,"byte_end":363,"line_start":17,"line_end":17,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    2 % ONE;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo 1 will be 0\n  --> tests\\ui\\modulo_one.rs:17:5\n   |\nLL |     2 % ONE;\n   |     ^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":411,"byte_end":422,"line_start":19,"line_end":19,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    2 % NEG_ONE;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests\\ui\\modulo_one.rs:19:5\n   |\nLL |     2 % NEG_ONE;\n   |     ^^^^^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests\\ui\\modulo_one.rs","byte_start":474,"byte_end":491,"line_start":21,"line_end":21,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    INT_MIN % NEG_ONE; // also caught by rustc","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests\\ui\\modulo_one.rs:21:5\n   |\nLL |     INT_MIN % NEG_ONE; // also caught by rustc\n   |     ^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\compiletest_rs-0.7.1\src\lib.rs:105:22
