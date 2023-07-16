plain
---- [ui] tests\ui\suggestions\issue-71394-no-from-impl.rs stdout ----
$DIR\issue-71394-no-from-impl.rs
diff of stderr:

5    |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
7    = help: the following other types implement trait `From<T>`:
7    = help: the following other types implement trait `From<T>`:
-              <&'input [u8] as From<gimli::read::endian_slice::EndianSlice<'input, Endian>>>
9              <[T; LANES] as From<Simd<T, LANES>>>
10              <[bool; LANES] as From<Mask<T, LANES>>>
11    = note: required for `&[u8]` to implement `Into<&[i8]>`

The actual stderr differed from the expected stderr.
Actual stderr saved to C:\a\rust\rust\build\i686-pc-windows-msvc\test\ui\suggestions\issue-71394-no-from-impl\issue-71394-no-from-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions\issue-71394-no-from-impl.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="C:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;C:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;C:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;C:\Program Files\PowerShell\7;C:\a\rust\rust\ninja;C:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "C:\\a\\rust\\rust\\tests\\ui\\suggestions\\issue-71394-no-from-impl.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=C:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\suggestions\\issue-71394-no-from-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\suggestions\\issue-71394-no-from-impl\\auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&[i8]: From<&[u8]>` is not satisfied
  --> fake-test-src-base\suggestions\issue-71394-no-from-impl.rs:5:25
   |
LL |     let _: &[i8] = data.into();
   |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
   = help: the following other types implement trait `From<T>`:
   = help: the following other types implement trait `From<T>`:
             <[T; LANES] as From<Simd<T, LANES>>>
             <[bool; LANES] as From<Mask<T, LANES>>>
   = note: required for `&[u8]` to implement `Into<&[i8]>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
