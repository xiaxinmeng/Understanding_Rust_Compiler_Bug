plain
To only update this specific test, also pass `--test-args panics\default-backtrace-ice.rs`

error: 1 errors occurred comparing output.
status: exit code: 101
command: PATH="C:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;C:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;C:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;C:\Program Files\PowerShell\7;C:\a\rust\rust\ninja;C:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.3\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.3.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.20.3\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\Program Files\OpenSSL\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.372-7\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "C:\\a\\rust\\rust\\tests\\ui\\panics\\default-backtrace-ice.rs" "-Zthreads=1" "--sysroot" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2" "--target=i686-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=C:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\panics\\default-backtrace-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\panics\\default-backtrace-ice\\auxiliary" "-Z" "treat-err-as-bug=1"
stdout: none
error[E0425]: cannot find value `missing_ident` in this scope
error[E0425]: cannot find value `missing_ident` in this scope
  --> fake-test-src-base\panics\default-backtrace-ice.rs:17:13
   |
LL | fn main() { missing_ident; }


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler\rustc_errors\src\lib.rs:1704:30
   0: 0x687a5452 - core::fmt::write::h6b6ccf45405f08e6
   0: 0x687a5452 - core::fmt::write::h6b6ccf45405f08e6
   1: 0x687676b0 - std::sys::common::alloc::realloc_fallback::ha211dc72bd2ff374
   2: 0x76a00000 - VarR8FromCy
   3:  0x2379068 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (34c1637b2 2023-05-03) running on i686-pc-windows-msvc

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
------------------------------------------
