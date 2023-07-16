plain
test [ui] ui\wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] ui\check-cfg\mix.rs stdout ----
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs
$DIR\mix.rs

47    |          ^^^
48 
48 
49 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:44:23
-    |
- LL |     cfg!(any(windows, xxx));
- 
- 
- warning: unexpected `cfg` condition name
56   --> $DIR/mix.rs:46:14
57    |
58 LL |     cfg!(any(xxx, windows));
68    |
68    |
69    = note: expected values for `feature` are: bar, foo
- warning: 10 warnings emitted
+ warning: 9 warnings emitted
72 
73 
73 


The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\ui\check-cfg\mix\mix.stderr
To only update this specific test, also pass `--test-args check-cfg\mix.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.4\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\ui\\check-cfg\\mix.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\check-cfg\\mix" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--check-cfg=names()" "--check-cfg=values(feature,\"foo\")" "--cfg" "feature=\"bar\"" "-Z" "unstable-options" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\check-cfg\\mix\\auxiliary"
stdout: none
--- stderr -------------------------------
warning: unexpected `cfg` condition name
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:11:7
   |
LL | #[cfg(widnows)]
   |       ^^^^^^^ help: did you mean: `windows`
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value
warning: unexpected `cfg` condition value
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:21:7
   |
LL | #[cfg(feature = "zebra")]
   |
   |
   = note: expected values for `feature` are: bar, foo

warning: unexpected `cfg` condition name
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:25:12
   |
LL | #[cfg_attr(uu, test)]


warning: unexpected `cfg` condition name
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:34:10
   |
LL |     cfg!(widnows);
   |          ^^^^^^^ help: did you mean: `windows`
warning: unexpected `cfg` condition value
warning: unexpected `cfg` condition value
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:38:10
   |
LL |     cfg!(feature = "zebra");
   |
   |
   = note: expected values for `feature` are: bar, foo

warning: unexpected `cfg` condition name
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:40:10
   |
LL |     cfg!(xxx = "foo");


warning: unexpected `cfg` condition name
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:42:10
   |
LL |     cfg!(xxx);


warning: unexpected `cfg` condition name
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:46:14
   |
LL |     cfg!(any(xxx, windows));

warning: unexpected `cfg` condition value
warning: unexpected `cfg` condition value
  --> D:\a\rust\rust\src/test\ui\check-cfg\mix.rs:48:14
   |
LL |     cfg!(any(feature = "bad", windows));
   |                        |
   |                        |
   |                        help: did you mean: `"bar"`
   |
   = note: expected values for `feature` are: bar, foo
warning: 9 warnings emitted
------------------------------------------


---
test result: FAILED. 12511 passed; 1 failed; 154 ignored; 0 measured; 0 filtered out; finished in 353.58s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:30:25
make: *** [Makefile:74: ci-subset-2] Error 1
