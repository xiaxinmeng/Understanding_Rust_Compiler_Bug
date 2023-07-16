plain
test [ui] rust\tests\ui\wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] rust\tests\ui\proc-macro\pretty-print-hack-show.rs#remapped stdout ----


3 PRINT-DERIVE INPUT (DEBUG): TokenStream [
5         ident: "enum",
5         ident: "enum",
-         span: remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:1: 4:5 (#0),
+         span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:1: 4:5 (#0),
8     Ident {
8     Ident {
9         ident: "ProceduralMasqueradeDummyType",

-         span: remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6: 4:35 (#0),
+         span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6: 4:35 (#0),
12     Group {
13         delimiter: Brace,

14         stream: TokenStream [
14         stream: TokenStream [
15             Ident {
16                 ident: "Input",
-                 span: remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:13:5: 13:10 (#0),
+                 span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:13:5: 13:10 (#0),
19         ],
19         ],
-         span: remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:36: 14:2 (#0),
+         span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:36: 14:2 (#0),
22 ]
22 ]
23 PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input, }

25 PRINT-DERIVE INPUT (DEBUG): TokenStream [
27         ident: "enum",
27         ident: "enum",
-         span: remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:1: 4:5 (#0),
+         span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:1: 4:5 (#0),
30     Ident {
30     Ident {
31         ident: "ProceduralMasqueradeDummyType",

-         span: remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6: 4:35 (#0),
+         span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6: 4:35 (#0),
34     Group {
35         delimiter: Brace,

36         stream: TokenStream [
36         stream: TokenStream [
37             Ident {
38                 ident: "Input",
-                 span: remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:13:5: 13:10 (#0),
+                 span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:13:5: 13:10 (#0),
41         ],
41         ],
-         span: remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:36: 14:2 (#0),
+         span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:36: 14:2 (#0),
44 ]
45 



The actual stdout differed from the expected stdout.
Actual stdout saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\proc-macro\pretty-print-hack-show.remapped\pretty-print-hack-show.remapped.stdout

1 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
3    |
4 LL | enum ProceduralMasqueradeDummyType {

10    = note: `#[deny(proc_macro_back_compat)]` on by default
11 
12 error: using an old version of `rental`
12 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
14    |
15 LL | enum ProceduralMasqueradeDummyType {


20    = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
22 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
24    |
24    |
25 LL | enum ProceduralMasqueradeDummyType {


30    = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
32 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
34    |
34    |
35 LL | enum ProceduralMasqueradeDummyType {


40    = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
42 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
44    |
44    |
45 LL | enum ProceduralMasqueradeDummyType {


50    = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
52 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
54    |
54    |
55 LL | enum ProceduralMasqueradeDummyType {


60    = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
62 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
64    |
64    |
65 LL | enum ProceduralMasqueradeDummyType {


70    = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
72 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
74    |
74    |
75 LL | enum ProceduralMasqueradeDummyType {

83 
84 Future incompatibility report: Future breakage diagnostic:
85 error: using an old version of `rental`
85 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
87    |
88 LL | enum ProceduralMasqueradeDummyType {

95 
96 Future breakage diagnostic:
97 error: using an old version of `rental`
97 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
99    |
100 LL | enum ProceduralMasqueradeDummyType {

107 
108 Future breakage diagnostic:
109 error: using an old version of `rental`
109 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
111    |
112 LL | enum ProceduralMasqueradeDummyType {

119 
120 Future breakage diagnostic:
121 error: using an old version of `rental`
121 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
123    |
124 LL | enum ProceduralMasqueradeDummyType {

131 
132 Future breakage diagnostic:
133 error: using an old version of `rental`
133 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
135    |
136 LL | enum ProceduralMasqueradeDummyType {

143 
144 Future breakage diagnostic:
145 error: using an old version of `rental`
145 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
147    |
148 LL | enum ProceduralMasqueradeDummyType {

155 
156 Future breakage diagnostic:
157 error: using an old version of `rental`
157 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
159    |
160 LL | enum ProceduralMasqueradeDummyType {

167 
168 Future breakage diagnostic:
169 error: using an old version of `rental`
169 error: using an old version of `rental`
-   --> remapped/proc-macro/pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
+   --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
171    |
172 LL | enum ProceduralMasqueradeDummyType {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\proc-macro\pretty-print-hack-show.remapped\pretty-print-hack-show.remapped.stderr
To only update this specific test, also pass `--test-args proc-macro\pretty-print-hack-show.rs`


error in revision `remapped`: 2 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.1\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.1\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.1\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.352-8\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\tests\\ui\\proc-macro\\pretty-print-hack-show.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--cfg" "remapped" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\proc-macro\\pretty-print-hack-show.remapped" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\proc-macro\\pretty-print-hack-show.remapped\\auxiliary" "-Z" "span-debug" "--remap-path-prefix=D:\\a\\rust\\rust\\tests\\ui=remapped"
--- stdout -------------------------------
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input, }
PRINT-DERIVE RE-COLLECTED (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "enum",
        ident: "enum",
        span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:1: 4:5 (#0),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6: 4:35 (#0),
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
            Ident {
                ident: "Input",
                span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:13:5: 13:10 (#0),
        ],
        ],
        span: remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:36: 14:2 (#0),
]
]
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input, }
PRINT-DERIVE RE-COLLECTED (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "enum",
        ident: "enum",
        span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:1: 4:5 (#0),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6: 4:35 (#0),
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
            Ident {
                ident: "Input",
                span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:13:5: 13:10 (#0),
        ],
        ],
        span: remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:36: 14:2 (#0),
]
------------------------------------------
--- stderr -------------------------------
error: using an old version of `rental`
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
error: aborting due to 8 previous errors

Future incompatibility report: Future breakage diagnostic:
error: using an old version of `rental`
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/allsorts-rental-0.5.6/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default
Future breakage diagnostic:
error: using an old version of `rental`
  --> remapped\proc-macro\pretty-print-hack/rental-0.5.5/src/lib.rs:4:6
   |
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
   = note: `#[deny(proc_macro_back_compat)]` on by default



failures:
failures:
    [ui] rust\tests\ui\proc-macro\pretty-print-hack-show.rs#remapped

test result: FAILED. 14018 passed; 1 failed; 162 ignored; 0 measured; 0 filtered out; finished in 518.27s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu
Build completed unsuccessfully in 0:36:55
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
