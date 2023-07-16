plain
test [ui] src/test\ui-fulldeps\deriving-hygiene.rs ... ok
test [ui] src/test\ui-fulldeps\deriving-global.rs ... ok
test [ui] src/test\ui-fulldeps\dropck-tarena-cycle-checked.rs ... ok
test [ui] src/test\ui-fulldeps\create-dir-all-bare.rs ... ok
test [ui] src/test\ui-fulldeps\fluent-messages\test.rs ... FAILED
test [ui] src/test\ui-fulldeps\dropck_tarena_sound_drop.rs ... ok
test [ui] src/test\ui-fulldeps\deriving-encodable-decodable-cell-refcell.rs ... ok
test [ui] src/test\ui-fulldeps\hash-stable-is-unstable.rs ... ok
test [ui] src/test\ui-fulldeps\feature-gate-plugin.rs ... ok
---
test [ui] src/test\ui-fulldeps\pprust-expr-roundtrip.rs ... ok

failures:

---- [ui] src/test\ui-fulldeps\fluent-messages\test.rs stdout ----
$DIR\test.rs
$DIR\test.rs
$DIR\test.rs
$DIR\test.rs
$DIR\test.rs


4 LL |         missing_absolute => "/definitely_does_not_exist.ftl",
6    |
-    = note: No such file or directory (os error 2)
+    = note: The system cannot find the file specified. (os error 2)
8 
8 
9 error: could not open Fluent resource
10   --> $DIR/test.rs:26:29

12 LL |         missing_relative => "../definitely_does_not_exist.ftl",
14    |
-    = note: No such file or directory (os error 2)
+    = note: The system cannot find the file specified. (os error 2)
16 
16 
17 error: could not parse Fluent resource
18   --> $DIR/test.rs:35:28


The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\ui-fulldeps\fluent-messages\test\test.stderr
To only update this specific test, also pass `--test-args fluent-messages\test.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.17.10\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\ui-fulldeps\\fluent-messages\\test.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui-fulldeps\\fluent-messages\\test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui-fulldeps\\fluent-messages\\test\\auxiliary"
stdout: none
--- stderr -------------------------------
error: could not open Fluent resource
  --> D:\a\rust\rust\src/test\ui-fulldeps\fluent-messages\test.rs:17:29
   |
LL |         missing_absolute => "/definitely_does_not_exist.ftl",
   |
   = note: The system cannot find the file specified. (os error 2)

error: could not open Fluent resource
error: could not open Fluent resource
  --> D:\a\rust\rust\src/test\ui-fulldeps\fluent-messages\test.rs:26:29
   |
LL |         missing_relative => "../definitely_does_not_exist.ftl",
   |
   = note: The system cannot find the file specified. (os error 2)

error: could not parse Fluent resource
error: could not parse Fluent resource
  --> D:\a\rust\rust\src/test\ui-fulldeps\fluent-messages\test.rs:35:28
   |
LL |         missing_message => "./missing-message.ftl",
   |
   = help: see additional errors emitted

error: expected a message field for "missing-message"
error: expected a message field for "missing-message"
 --> ./missing-message.ftl:1:1
1 | missing-message = 
  | ^^^^^^^^^^^^^^^^^^
  |


Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
error: overrides existing message: `key`
  --> D:\a\rust\rust\src/test\ui-fulldeps\fluent-messages\test.rs:45:9
   |
LL |         b => "./duplicate-b.ftl",
   |
help: previously defined in this resource
help: previously defined in this resource
  --> D:\a\rust\rust\src/test\ui-fulldeps\fluent-messages\test.rs:44:9
   |
LL |         a => "./duplicate-a.ftl",

error: aborting due to 4 previous errors
------------------------------------------




failures:
    [ui] src/test\ui-fulldeps\fluent-messages\test.rs
test result: FAILED. 67 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 20.00s

Build completed unsuccessfully in 0:28:20
Build completed unsuccessfully in 0:28:20
make: *** [Makefile:70: ci-subset-1] Error 1
