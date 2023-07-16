plain
Updating files:  98% (38959/39754)
Updating files:  99% (39357/39754)
Updating files: 100% (39754/39754)
Updating files: 100% (39754/39754), done.
Note: switching to 'refs/remotes/pull/106789/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at f49be9cf Merge 54d65b1aff1418d2e9f075a4f33fcf6a3be63d2b into 51cb5614dde163c53aab73e2dc33bb9f388b50a8
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'f49be9cfb2c6c14ed7372f942828391e7b16599b'
'f49be9cfb2c6c14ed7372f942828391e7b16599b'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-mingw-2
  CARGO_REGISTRIES_CRATES_IO_PROTOCOL: sparse
  SCCACHE_BUCKET: rust-lang-ci-sccache2
---
........................................................................................ 1936/14444
.i.............ii....................................................................... 2024/14444
........................................................................................ 2112/14444
..............ii.iiii......................................................i............ 2200/14444
.................................FF.................F................................... 2288/14444
........................................................................................ 2464/14444
.................................................................F...................... 2552/14444
........................................................................................ 2640/14444
........................................................................................ 2728/14444
---
........................................................................................ 13200/14444
........................................................................................ 13288/14444
........................................................................................ 13376/14444
........................................................................................ 13464/14444
........................................................i.F......................F...... 13552/14444
.............................................................................ii......... 13728/14444
........................................................................................ 13816/14444
........................................................................................ 13904/14444
........................................................................................ 13992/14444
---
---- [ui] tests\ui\asm\issue-72570.rs stdout ----
$DIR\issue-72570.rs
diff of stderr:

4 LL |         asm!("", in("invalid") "".len());
6 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args asm\issue-72570.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\asm\\issue-72570.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\asm\\issue-72570" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\asm\\issue-72570\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error: invalid register `invalid`: unknown register
  --> fake-test-src-base\asm\issue-72570.rs:9:18
   |
LL |         asm!("", in("invalid") "".len());

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error
------------------------------------------
---



The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\const-generics\const-argument-non-static-lifetime.full\const-argument-non-static-lifetime.full.stderr
To only update this specific test, also pass `--test-args const-generics\const-argument-non-static-lifetime.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\const-generics\\const-argument-non-static-lifetime.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\const-argument-non-static-lifetime.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\const-argument-non-static-lifetime.full\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\const-generics\const-argument-non-static-lifetime.rs#min stdout ----
$DIR\const-argument-non-static-lifetime.rs
---
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\const-generics\const-argument-non-static-lifetime.min\const-argument-non-static-lifetime.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics\const-argument-non-static-lifetime.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\const-generics\\const-argument-non-static-lifetime.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\const-argument-non-static-lifetime.min" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\const-argument-non-static-lifetime.min\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base\const-generics\const-argument-non-static-lifetime.rs:15:17
   |
LL |         let _: &'a (); //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable

---
---- [ui] tests\ui\const-generics\const-param-type-depends-on-type-param-ungated.rs stdout ----
$DIR\const-param-type-depends-on-type-param-ungated.rs
diff of stderr:

4 LL | struct B<T, const N: T>(PhantomData<[T; N]>);
5    |                      ^ the type must not depend on the parameter `T`
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
7 error: aborting due to previous error
8 
9 For more information about this error, try `rustc --explain E0770`.
---
To only update this specific test, also pass `--test-args const-generics\const-param-type-depends-on-type-param-ungated.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\const-generics\\const-param-type-depends-on-type-param-ungated.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\const-param-type-depends-on-type-param-ungated" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\const-param-type-depends-on-type-param-ungated\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0770]: the type of const parameters must not depend on other generic parameters
  --> fake-test-src-base\const-generics\const-param-type-depends-on-type-param-ungated.rs:6:22
   |
LL | struct B<T, const N: T>(PhantomData<[T; N]>);
   |                      ^ the type must not depend on the parameter `T`
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error

For more information about this error, try `rustc --explain E0770`.
---
$DIR\issue-73260.rs
$DIR\issue-73260.rs
diff of stderr:

32 LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
33    |                                     ^^^^^^ required by this bound in `Arr`
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
35 error: aborting due to 2 previous errors
36 
37 For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args const-generics\issues\issue-73260.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\const-generics\\issues\\issue-73260.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\issues\\issue-73260" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\const-generics\\issues\\issue-73260\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base\const-generics\issues\issue-73260.rs:16:12
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |            ^^^^^^^^^^^^^^^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`
note: required by a bound in `Arr`
  --> fake-test-src-base\const-generics\issues\issue-73260.rs:6:37
  --> fake-test-src-base\const-generics\issues\issue-73260.rs:6:37
   |
LL | struct Arr<const N: usize>
LL | where
LL | where
LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
   |                                     ^^^^^^ required by this bound in `Arr`
error[E0308]: mismatched types
  --> fake-test-src-base\const-generics\issues\issue-73260.rs:16:32
   |
   |
LL |     let x: Arr<{usize::MAX}> = Arr {};
   |                                ^^^ expected `false`, found `true`
   = note: expected constant `false`
              found constant `true`
note: required by a bound in `Arr`
  --> fake-test-src-base\const-generics\issues\issue-73260.rs:6:37
  --> fake-test-src-base\const-generics\issues\issue-73260.rs:6:37
   |
LL | struct Arr<const N: usize>
LL | where
LL | where
LL |     Assert::<{N < usize::MAX / 2}>: IsTrue,
   |                                     ^^^^^^ required by this bound in `Arr`
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
---
$DIR\feature-gate-associated_type_bounds.rs
$DIR\feature-gate-associated_type_bounds.rs
diff of stderr:

144 LL | trait _Tr3 where <<Self as _Tr3>::A as Iterator>::Item: Copy {
146 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
147 error: aborting due to 17 previous errors
148 
---
To only update this specific test, also pass `--test-args feature-gates\feature-gate-associated_type_bounds.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\feature-gates\\feature-gate-associated_type_bounds.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\feature-gates\\feature-gate-associated_type_bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\feature-gates\\feature-gate-associated_type_bounds\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:15:22
   |
LL |     type A: Iterator<Item: Copy>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:19:22
   |
LL |     type B: Iterator<Item: 'static>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:23:20
   |
LL | struct _St1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:30:18
   |
LL | enum _En1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:37:19
   |
LL | union _Un1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:44:37
   |
LL | type _TaWhere1<T> where T: Iterator<Item: Copy> = T;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:47:22
   |
LL | fn _apit(_: impl Tr1<As1: Copy>) {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:49:26
   |
LL | fn _apit_dyn(_: &dyn Tr1<As1: Copy>) {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:52:24
   |
LL | fn _rpit() -> impl Tr1<As1: Copy> { S1 }
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:55:31
   |
LL | fn _rpit_dyn() -> Box<dyn Tr1<As1: Copy>> { Box::new(S1) }
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:58:23
   |
LL | const _cdef: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:64:24
   |
LL | static _sdef: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:71:21
   |
LL |     let _: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable

error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in type
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:58:14
   |
LL | const _cdef: impl Tr1<As1: Copy> = S1;


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in type
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:64:15
   |
LL | static _sdef: impl Tr1<As1: Copy> = S1;


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in variable binding
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:71:12
   |
LL |     let _: impl Tr1<As1: Copy> = S1;


error[E0277]: the trait bound `<<Self as _Tr3>::A as Iterator>::Item: Copy` is not satisfied
  --> fake-test-src-base\feature-gates\feature-gate-associated_type_bounds.rs:15:28
   |
LL |     type A: Iterator<Item: Copy>;
   |                            ^^^^ the trait `Copy` is not implemented for `<<Self as _Tr3>::A as Iterator>::Item`
help: consider further restricting the associated type
   |
   |
LL | trait _Tr3 where <<Self as _Tr3>::A as Iterator>::Item: Copy {

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to 17 previous errors


Some errors have detailed explanations: E0277, E0562, E0658.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests\ui\impl-trait\bound-normalization-pass.rs#sa stdout ----
ERROR rustc_save_analysis Could not deserialize save-analysis config




The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\impl-trait\bound-normalization-pass.sa\bound-normalization-pass.sa.stderr
To only update this specific test, also pass `--test-args impl-trait\bound-normalization-pass.rs`


error in revision `sa`: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\impl-trait\\bound-normalization-pass.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--cfg" "sa" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\impl-trait\\bound-normalization-pass.sa" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\impl-trait\\bound-normalization-pass.sa\\auxiliary" "--edition=2018" "-Z" "save-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\issues\issue-3763.rs stdout ----
$DIR\issue-3763.rs
$DIR\issue-3763.rs
$DIR\issue-3763.rs
$DIR\issue-3763.rs
$DIR\issue-3763.rs
$DIR\issue-3763.rs
diff of stderr:

34 LL |     let nope = my_struct.priv_field;
36 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
37 error: aborting due to 5 previous errors
38 
---
To only update this specific test, also pass `--test-args issues\issue-3763.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\issues\\issue-3763.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\issues\\issue-3763" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\issues\\issue-3763\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0616]: field `priv_field` of struct `MyStruct` is private
  --> fake-test-src-base\issues\issue-3763.rs:18:32
   |
LL |     let _woohoo = (&my_struct).priv_field;


error[E0616]: field `priv_field` of struct `MyStruct` is private
  --> fake-test-src-base\issues\issue-3763.rs:21:41
   |
LL |     let _woohoo = (Box::new(my_struct)).priv_field;


error[E0624]: associated function `happyfun` is private
  --> fake-test-src-base\issues\issue-3763.rs:24:18
   |
LL |         fn happyfun(&self) {}
...
...
LL |     (&my_struct).happyfun();               //~ ERROR associated function `happyfun` is private


error[E0624]: associated function `happyfun` is private
  --> fake-test-src-base\issues\issue-3763.rs:26:27
   |
LL |         fn happyfun(&self) {}
...
...
LL |     (Box::new(my_struct)).happyfun();          //~ ERROR associated function `happyfun` is private


error[E0616]: field `priv_field` of struct `MyStruct` is private
  --> fake-test-src-base\issues\issue-3763.rs:27:26
   |
LL |     let nope = my_struct.priv_field;

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to 5 previous errors

---
To only update this specific test, also pass `--test-args methods\assign-to-method.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\methods\\assign-to-method.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\methods\\assign-to-method" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\methods\\assign-to-method\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0615]: attempted to take value of method `speak` on type `Cat`
  --> fake-test-src-base\methods\assign-to-method.rs:22:10
   |
LL |     nyan.speak = || println!("meow"); //~ ERROR attempted to take value of method
   |
   = help: methods are immutable and cannot be assigned to


error[E0615]: attempted to take value of method `speak` on type `Cat`
  --> fake-test-src-base\methods\assign-to-method.rs:23:10
   |
LL |     nyan.speak += || println!("meow"); //~ ERROR attempted to take value of method
   |
   = help: methods are immutable and cannot be assigned to

ERROR rustc_save_analysis Could not deserialize save-analysis config
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0615`.
------------------------------------------


---- [ui] tests\ui\save-analysis\emit-notifications.rs stdout ----
$TEST_BUILD_DIR\save-analysis\emit-notifications\save-analysis\libemit_notifications.json
$TEST_BUILD_DIR\save-analysis\emit-notifications\libemit_notifications.rlib

+ ERROR rustc_save_analysis Could not deserialize save-analysis config
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
1 {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/save-analysis/libemit_notifications.json","emit":"save-analysis"}
2 {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/libemit_notifications.rlib","emit":"link"}


The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\save-analysis\emit-notifications\emit-notifications.stderr
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\save-analysis\emit-notifications\emit-notifications.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args save-analysis\emit-notifications.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\emit-notifications.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\emit-notifications" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\emit-notifications\\auxiliary" "-Zsave-analysis" "--json" "artifacts" "--crate-type" "rlib" "--error-format=json"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-37323.rs stdout ----
normalized stderr:
---
To only update this specific test, also pass `--test-args save-analysis\issue-37323.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-37323.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-37323" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-37323\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-63663.rs stdout ----
normalized stderr:
---
To only update this specific test, also pass `--test-args save-analysis\issue-63663.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-63663.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-63663" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-63663\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-64659.rs stdout ----
normalized stderr:
---
To only update this specific test, also pass `--test-args save-analysis\issue-64659.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-64659.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-64659" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-64659\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-65411.rs stdout ----
normalized stderr:
---
To only update this specific test, also pass `--test-args save-analysis\issue-65411.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-65411.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-65411" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-65411\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-65590.rs stdout ----
normalized stderr:
---
To only update this specific test, also pass `--test-args save-analysis\issue-65590.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-65590.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-65590" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-65590\\auxiliary" "-Zsave-analysis" "--edition=2018"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-68621.rs stdout ----
$DIR\issue-68621.rs
$DIR\issue-68621.rs
diff of stderr:

6    |
7    = note: `Future` must be used in combination with a concrete type within the same impl
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
9 error: aborting due to previous error
10 
11 
---
To only update this specific test, also pass `--test-args save-analysis\issue-68621.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-68621.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-68621" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-68621\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error: unconstrained opaque type
  --> fake-test-src-base\save-analysis\issue-68621.rs:14:19
   |
LL |     type Future = impl Trait; //~ ERROR: unconstrained opaque type
   |
   |
   = note: `Future` must be used in combination with a concrete type within the same impl
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error
------------------------------------------



---- [ui] tests\ui\save-analysis\issue-26459.rs stdout ----
$DIR\issue-26459.rs
diff of stderr:

4 LL |         char{ch} => true
5    |         ^^^^ not a struct, variant or union type
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
7 error: aborting due to previous error
8 
9 For more information about this error, try `rustc --explain E0574`.
---
To only update this specific test, also pass `--test-args save-analysis\issue-26459.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-26459.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-26459" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-26459\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0574]: expected struct, variant or union type, found builtin type `char`
  --> fake-test-src-base\save-analysis\issue-26459.rs:5:9
   |
LL |         char{ch} => true
   |         ^^^^ not a struct, variant or union type
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error

For more information about this error, try `rustc --explain E0574`.
For more information about this error, try `rustc --explain E0574`.
------------------------------------------


---- [ui] tests\ui\save-analysis\issue-59134-0.rs stdout ----
$DIR\issue-59134-0.rs
diff of stderr:

4 LL |         const FLAG: u32 = bogus.field;
6 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args save-analysis\issue-59134-0.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-59134-0.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-59134-0" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-59134-0\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `bogus` in this scope
  --> fake-test-src-base\save-analysis\issue-59134-0.rs:8:27
   |
LL |         const FLAG: u32 = bogus.field; //~ ERROR cannot find value `bogus`

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error

---
---- [ui] tests\ui\save-analysis\issue-59134-1.rs stdout ----
$DIR\issue-59134-1.rs
diff of stderr:

4 LL |         const CONST: Self::MyType = bogus.field;
6 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args save-analysis\issue-59134-1.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-59134-1.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-59134-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-59134-1\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `bogus` in this scope
  --> fake-test-src-base\save-analysis\issue-59134-1.rs:8:37
   |
LL |         const CONST: Self::MyType = bogus.field; //~ ERROR cannot find value `bogus`

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error

---
---- [ui] tests\ui\save-analysis\issue-73020.rs stdout ----
$DIR\issue-73020.rs
diff of stderr:

4 LL | use {self};
5    |      ^^^^ can only appear in an import list with a non-empty prefix
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
7 error: aborting due to previous error
8 
9 For more information about this error, try `rustc --explain E0431`.
---
To only update this specific test, also pass `--test-args save-analysis\issue-73020.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-73020.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-73020" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-73020\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0431]: `self` import can only appear in an import list with a non-empty prefix
  --> fake-test-src-base\save-analysis\issue-73020.rs:2:6
   |
LL | use {self}; //~ ERROR E0431
   |      ^^^^ can only appear in an import list with a non-empty prefix
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error

For more information about this error, try `rustc --explain E0431`.
---
To only update this specific test, also pass `--test-args save-analysis\issue-73022.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-73022.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-73022" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-73022\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\save-analysis\issue-72267.rs stdout ----
$DIR\issue-72267.rs
$DIR\issue-72267.rs
$DIR\issue-72267.rs
diff of stderr:

10 LL |     let _: Box<(dyn ?Sized)>;
12 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
13 error: aborting due to 2 previous errors
14 
---
To only update this specific test, also pass `--test-args save-analysis\issue-72267.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-72267.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-72267" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-72267\\auxiliary" "-Z" "save-analysis"
stdout: none
--- stderr -------------------------------
error: `?Trait` is not permitted in trait object types
  --> fake-test-src-base\save-analysis\issue-72267.rs:4:21
   |
LL |     let _: Box<(dyn ?Sized)>;

error[E0224]: at least one trait is required for an object type
error[E0224]: at least one trait is required for an object type
  --> fake-test-src-base\save-analysis\issue-72267.rs:4:17
   |
LL |     let _: Box<(dyn ?Sized)>;

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to 2 previous errors

---
$DIR\issue-89066.rs
$DIR\issue-89066.rs
diff of stderr:

19    = help: consider removing `_`, referring to it in a field, or using a marker such as `PhantomData`
20    = help: if you intended `_` to be a const parameter, use `const _: usize` instead
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
22 error[E0107]: this struct takes 2 generic arguments but 3 generic arguments were supplied
24    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\save-analysis\issue-89066\issue-89066.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args save-analysis\issue-89066.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\save-analysis\\issue-89066.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-89066" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\save-analysis\\issue-89066\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error: expected identifier, found reserved identifier `_`
  --> fake-test-src-base\save-analysis\issue-89066.rs:12:17
   |
LL | struct BadInfer<_>;

error: expected identifier, found reserved identifier `_`
error: expected identifier, found reserved identifier `_`
  --> fake-test-src-base\save-analysis\issue-89066.rs:18:17
   |
LL | fn bad_infer_fn<_>() {}

error[E0392]: parameter `_` is never used
error[E0392]: parameter `_` is never used
  --> fake-test-src-base\save-analysis\issue-89066.rs:12:17
   |
LL | struct BadInfer<_>;
   |                 ^ unused parameter
   |
   = help: consider removing `_`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `_` to be a const parameter, use `const _: usize` instead
ERROR rustc_save_analysis Could not deserialize save-analysis config
ERROR rustc_save_analysis Could not deserialize save-analysis config
error[E0107]: this struct takes 2 generic arguments but 3 generic arguments were supplied
  --> fake-test-src-base\save-analysis\issue-89066.rs:23:10
   |
LL |   let a: All<_, _, _>;
   |          ^^^       - help: remove this generic argument
   |          expected 2 generic arguments
   |
   |
note: struct defined here, with 2 generic parameters: `T`, `N`
  --> fake-test-src-base\save-analysis\issue-89066.rs:8:8
   |
LL | struct All<'a, T, const N: usize> {

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0107, E0392.
---
$DIR\self-in-generics.rs
diff of stderr:

6    |
7    = note: it cannot use `Self` as a type parameter in a supertrait or `where`-clause
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
9 error: aborting due to previous error
10 
11 For more information about this error, try `rustc --explain E0038`.
---
To only update this specific test, also pass `--test-args traits\alias\self-in-generics.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\traits\\alias\\self-in-generics.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\traits\\alias\\self-in-generics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\traits\\alias\\self-in-generics\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0038]: the trait alias `SelfInput` cannot be made into an object
  --> fake-test-src-base\traits\alias\self-in-generics.rs:12:19
   |
LL | pub fn f(_f: &dyn SelfInput) {}
   |
   |
   = note: it cannot use `Self` as a type parameter in a supertrait or `where`-clause
ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to previous error

For more information about this error, try `rustc --explain E0038`.
---
To only update this specific test, also pass `--test-args type-alias-impl-trait\issue-65679-inst-opaque-ty-from-val-twice.rs`

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\type-alias-impl-trait\\issue-65679-inst-opaque-ty-from-val-twice.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\type-alias-impl-trait\\issue-65679-inst-opaque-ty-from-val-twice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\type-alias-impl-trait\\issue-65679-inst-opaque-ty-from-val-twice\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
ERROR rustc_save_analysis Could not deserialize save-analysis config


---- [ui] tests\ui\type-alias-impl-trait\issue-63279.rs stdout ----
$DIR\issue-63279.rs
$DIR\issue-63279.rs
$DIR\issue-63279.rs
$DIR\issue-63279.rs
$DIR\issue-63279.rs
$DIR\issue-63279.rs
$DIR\issue-63279.rs
diff of stderr:

42 LL |     (|| -> Closure { || () })()
44 
+ ERROR rustc_save_analysis Could not deserialize save-analysis config
45 error: aborting due to 4 previous errors
46 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait\issue-63279.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\type-alias-impl-trait\\issue-63279.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\type-alias-impl-trait\\issue-63279" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\type-alias-impl-trait\\issue-63279\\auxiliary" "-Zsave-analysis"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `FnOnce<()>` closure, found `()`
  --> fake-test-src-base\type-alias-impl-trait\issue-63279.rs:7:11
LL | fn c() -> Closure {
LL | fn c() -> Closure {
   |           ^^^^^^^ expected an `FnOnce<()>` closure, found `()`
   |
   = help: the trait `FnOnce<()>` is not implemented for `()`
   = note: wrap the `()` in a closure with no arguments: `|| { /* code */ }`

error[E0277]: expected a `FnOnce<()>` closure, found `()`
  --> fake-test-src-base\type-alias-impl-trait\issue-63279.rs:9:11
   |
LL |     || -> Closure { || () }
   |           ^^^^^^^ expected an `FnOnce<()>` closure, found `()`
   |
   = help: the trait `FnOnce<()>` is not implemented for `()`
   = note: wrap the `()` in a closure with no arguments: `|| { /* code */ }`
error[E0308]: mismatched types
  --> fake-test-src-base\type-alias-impl-trait\issue-63279.rs:9:21
   |
   |
LL |     || -> Closure { || () }
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found closure `[closure@fake-test-src-base\type-alias-impl-trait\issue-63279.rs:9:21: 9:23]`
help: use parentheses to call this closure
   |
LL |     || -> Closure { (|| ())() }

error[E0308]: mismatched types
  --> fake-test-src-base\type-alias-impl-trait\issue-63279.rs:9:5
   |
   |
LL |     || -> Closure { || () }
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found closure `[closure@fake-test-src-base\type-alias-impl-trait\issue-63279.rs:9:5: 9:18]`
help: use parentheses to call this closure
   |
LL |     (|| -> Closure { || () })()

ERROR rustc_save_analysis Could not deserialize save-analysis config
error: aborting due to 4 previous errors

---
    [ui] tests\ui\type-alias-impl-trait\issue-65679-inst-opaque-ty-from-val-twice.rs

test result: FAILED. 14250 passed; 26 failed; 168 ignored; 0 measured; 0 filtered out; finished in 560.30s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
