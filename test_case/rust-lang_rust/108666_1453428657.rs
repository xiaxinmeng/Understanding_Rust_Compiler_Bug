plain
test [ui] tests\ui\zero-sized\zero-sized-btreemap-insert.rs ... ok

failures:

---- [ui] tests\ui\suggestions\multiline-multipart-suggestion.rs stdout ----
$DIR\multiline-multipart-suggestion.rs
$DIR\multiline-multipart-suggestion.rs
$DIR\multiline-multipart-suggestion.rs

- error[E0106]: missing lifetime specifier
-   --> $DIR/multiline-multipart-suggestion.rs:3:34
-    |
-    |
- LL | fn short(foo_bar: &Vec<&i32>) -> &i32 {
-    |                   ----------     ^ expected named lifetime parameter
-    |
-    = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
+ error[E0106]: missing lifetime specifier
+   --> $DIR/multiline-multipart-suggestion.rs:3:34
+    |
+ LL | fn short(foo_bar: &Vec<&i32>) -> &i32 {
+    |                   ----------     ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
8 help: consider introducing a named lifetime parameter
-    |
- LL | fn short<'a>(foo_bar: &'a Vec<&'a i32>) -> &'a i32 {
-    |         ++++           ++      ++           ++
+    |
+ LL | fn short<'a>(foo_bar: &'a Vec<&'a i32>) -> &'a i32 {
+    |         ++++           ++      ++           ++
- error[E0106]: missing lifetime specifier
-   --> $DIR/multiline-multipart-suggestion.rs:10:6
-    |
-    |
- LL |     foo_bar: &Vec<&i32>,
-    |              ----------
- LL |     something_very_long_so_that_the_line_will_wrap_around__________: i32,
- LL | ) -> &i32 {
-    |      ^ expected named lifetime parameter
-    |
-    = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
+ error[E0106]: missing lifetime specifier
+   --> $DIR/multiline-multipart-suggestion.rs:10:6
+    |
+ LL |     foo_bar: &Vec<&i32>,
+    |              ----------
+ LL |     something_very_long_so_that_the_line_will_wrap_around__________: i32,
+ LL | ) -> &i32 {
+    |      ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
23 help: consider introducing a named lifetime parameter
-    |
- LL ~ fn long<'a>(
- LL ~     foo_bar: &'a Vec<&'a i32>,
- LL |     something_very_long_so_that_the_line_will_wrap_around__________: i32,
- LL ~ ) -> &'a i32 {
+    |
+    |
+ LL ~ fn long<'a>(
+ LL ~     foo_bar: &'a Vec<&'a i32>,
+ LL |     something_very_long_so_that_the_line_will_wrap_around__________: i32,
+ LL ~ ) -> &'a i32 {
30 
- error[E0106]: missing lifetime specifier
-   --> $DIR/multiline-multipart-suggestion.rs:15:29
-    |
-    |
- LL |     foo_bar: &Vec<&i32>) -> &i32 {
-    |              ----------     ^ expected named lifetime parameter
-    |
-    = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
+ error[E0106]: missing lifetime specifier
+   --> $DIR/multiline-multipart-suggestion.rs:15:29
+    |
+ LL |     foo_bar: &Vec<&i32>) -> &i32 {
+    |              ----------     ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
38 help: consider introducing a named lifetime parameter
-    |
- LL ~ fn long2<'a>(
- LL ~     foo_bar: &'a Vec<&'a i32>) -> &'a i32 {
+    |
+    |
+ LL ~ fn long2<'a>(
+ LL ~     foo_bar: &'a Vec<&'a i32>) -> &'a i32 {
43 
- error: aborting due to 3 previous errors
+ error: aborting due to 3 previous errors
45 
---
To only update this specific test, also pass `--test-args suggestions\multiline-multipart-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;C:\Program Files\PowerShell\7;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.9.0\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\suggestions\\multiline-multipart-suggestion.rs" "-Zthreads=1" "--target=i686-pc-windows-msvc" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\suggestions\\multiline-multipart-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui\\suggestions\\multiline-multipart-suggestion\\auxiliary" "--error-format=human" "--color=always"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
  --> fake-test-src-base\suggestions\multiline-multipart-suggestion.rs:3:34
   |
LL | fn short(foo_bar: &Vec<&i32>) -> &i32 { //~ ERROR missing lifetime specifier
   |                   ----------     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
   |
LL | fn short<'a>(foo_bar: &'a Vec<&'a i32>) -> &'a i32 { //~ ERROR missing lifetime specifier
   |         ++++           ++      ++           ++
error[E0106]: missing lifetime specifier
error[E0106]: missing lifetime specifier
  --> fake-test-src-base\suggestions\multiline-multipart-suggestion.rs:10:6
   |
LL |     foo_bar: &Vec<&i32>,
   |              ----------
LL |     something_very_long_so_that_the_line_will_wrap_around__________: i32,
LL | ) -> &i32 {
   |      ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
   |
LL ~ fn long<'a>( //~ ERROR missing lifetime specifier
LL ~     foo_bar: &'a Vec<&'a i32>,
LL |     something_very_long_so_that_the_line_will_wrap_around__________: i32,
LL ~ ) -> &'a i32 {

error[E0106]: missing lifetime specifier
error[E0106]: missing lifetime specifier
  --> fake-test-src-base\suggestions\multiline-multipart-suggestion.rs:15:29
   |
LL |     foo_bar: &Vec<&i32>) -> &i32 {
   |              ----------     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `foo_bar`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
   |
LL ~ fn long2<'a>( //~ ERROR missing lifetime specifier
LL ~     foo_bar: &'a Vec<&'a i32>) -> &'a i32 {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0106`.
