plain
test [ui] tests\ui\coherence\impl[t]-foreign[t]-for-fundamental.rs ... ok
test [ui] tests\ui\coinduction\canonicalization-rerun.rs#new ... ok
test [ui] tests\ui\coherence\issue-85026.rs ... ok
test [ui] tests\ui\coherence\impl[t]-foreign[local]-for-t.rs ... ok
test [ui] tests\ui\command\command-argv0.rs ... ignored, ignored when the operative system is windows (this is a unix-specific test)
test [ui] tests\ui\command\command-create-pidfd.rs ... ignored, only executed when the operative system is linux (pidfds are a linux-specific concept)
test [ui] tests\ui\command\command-exec.rs ... ignored, ignored when the operative system is windows (this is a unix-specific test)
test [ui] tests\ui\command\command-pre-exec.rs#mir ... ignored, ignored when the operative system is windows (this is a unix-specific test)
test [ui] tests\ui\command\command-pre-exec.rs#thir ... ignored, ignored when the operative system is windows (this is a unix-specific test)
test [ui] tests\ui\command\command-setgroups.rs ... ignored, ignored when the operative system is windows (this is a unix-specific test)
---
test [ui] tests\ui\issues\issue-18423.rs ... ok
test [ui] tests\ui\issues\issue-18119.rs ... ok
test [ui] tests\ui\issues\issue-18685.rs ... ok
test [ui] tests\ui\issues\issue-18576.rs ... ok
test [ui] tests\ui\issues\issue-18804\main.rs ... ignored, ignored when the operative system is windows (no extern_weak linkage)
test [ui] tests\ui\issues\issue-18738.rs ... ok
test [ui] tests\ui\issues\issue-18514.rs ... ok
test [ui] tests\ui\issues\issue-18767.rs ... ok
test [ui] tests\ui\issues\issue-18906.rs ... ok
---
test [ui] tests\ui\issues\issue-6919.rs ... ok
test [ui] tests\ui\issues\issue-69396-const-no-type-in-macro.rs ... ok
test [ui] tests\ui\issues\issue-69225-SCEVAddExpr-wrap-flag.rs ... ok
test [ui] tests\ui\issues\issue-69225-layout-repeated-checked-add.rs ... ok
test [ui] tests\ui\issues\issue-70093\issue-70093-link-directives.rs ... ignored, ignored when the operative system is windows (this will probably only work on unixish systems)
test [ui] tests\ui\issues\issue-70093\issue-70093.rs ... ignored, ignored when the operative system is windows (this will probably only work on unixish systems)
test [ui] tests\ui\issues\issue-66923-show-error-for-correct-call.rs ... ok
test [ui] tests\ui\issues\issue-70381.rs ... ok
test [ui] tests\ui\issues\issue-7044.rs ... ok
test [ui] tests\ui\issues\issue-70673.rs ... ok
---
test [ui] tests\ui\overloaded\overloaded_deref_with_ref_pattern.rs ... ok
test [ui] tests\ui\overloaded\overloaded-index-assoc-list.rs ... ok
test [ui] tests\ui\overloaded\overloaded-index.rs ... ok
test [ui] tests\ui\overloaded\overloaded_deref_with_ref_pattern_issue15609.rs ... ok
test [ui] tests\ui\packed\packed-struct-borrow-element-64bit.rs ... ignored, ignored when the pointer width is 32bit ((needs `usize` to be 8-aligned to reproduce all the errors below))
test [ui] tests\ui\packed\packed-struct-borrow-element.rs ... ok
test [ui] tests\ui\packed-struct\packed-struct-transmute.rs ... ok
test [ui] tests\ui\overloaded\overloaded-deref-count.rs ... ok
test [ui] tests\ui\packed\issue-27060-2.rs ... ok
---
---- [ui] tests\ui\symbol-names\x86-stdcall.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: PATH="C:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin;C:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;C:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;C:\a\rust\rust\ninja;C:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "C:\\a\\rust\\rust\\tests\\ui\\symbol-names\\x86-stdcall.rs" "-Zthreads=1" "--target=i686-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=C:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\symbol-names\\x86-stdcall" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\symbol-names\\x86-stdcall\\auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `i686-w64-mingw32-gcc` failed: exit code: 1
   |
   = note: "i686-w64-mingw32-gcc" "-Wl,C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcNCFKA7\\list.def" "-fno-use-linker-plugin" "-Wl,--dynamicbase" "-Wl,--disable-auto-image-base" "-Wl,--large-address-aware" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\rsbegin.o" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcNCFKA7\\symbols.o" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\symbol-names\\x86-stdcall\\x86-stdcall.x86_stdcall.69a031ae-cgu.0.rcgu.o" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\symbol-names\\x86-stdcall\\auxiliary" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-lstd-f8ce1d6d5fcac5f3" "-Wl,-Bstatic" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\libcompiler_builtins-59dfbfb39a743cbc.rlib" "-Wl,-Bdynamic" "-lkernel32" "-ladvapi32" "-luserenv" "-lkernel32" "-lws2_32" "-lbcrypt" "-lntdll" "-lgcc_s" "-lmsvcrt" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-luser32" "-lkernel32" "-Wl,--nxcompat" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-o" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\symbol-names\\x86-stdcall\\x86_stdcall.dll" "-Wl,--gc-sections" "-shared" "-Wl,--out-implib=C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\symbol-names\\x86-stdcall\\libx86_stdcall.dll.a" "-Wl,--strip-debug" "-nodefaultlibs" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\rsend.o"
   = note: C:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/12.2.0/../../../../i686-w64-mingw32/bin/ld.exe: warning: resolving _bar by linking to @bar@4
           Use --enable-stdcall-fixup to disable these warnings
           Use --disable-stdcall-fixup to disable these fixups
           C:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/12.2.0/../../../../i686-w64-mingw32/bin/ld.exe: warning: resolving _foo by linking to _foo@4
           C:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/12.2.0/../../../../i686-w64-mingw32/bin/ld.exe: cannot export baz: symbol not defined
           collect2.exe: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------

---
test result: FAILED. 14506 passed; 1 failed; 227 ignored; 0 measured; 0 filtered out; finished in 939.55s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-gnu target=i686-pc-windows-gnu
Build completed unsuccessfully in 0:57:34
make: *** [Makefile:80: ci-mingw-subset-2] Error 1
