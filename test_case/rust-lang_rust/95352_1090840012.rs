plain
[TIMING] Rustc { target: TargetSelection { triple: "i686-pc-windows-msvc", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "i686-pc-windows-msvc", file: None } } } -- 0.000
Check compiletest suite=ui-fulldeps mode=ui (i686-pc-windows-msvc -> i686-pc-windows-msvc)

running 67 tests
test [ui] src/test\ui-fulldeps\dropck-tarena-unsound-drop.rs ... ok
test [ui] src/test\ui-fulldeps\dropck-tarena-cycle-checked.rs ... ok
test [ui] src/test\ui-fulldeps\deriving-hygiene.rs ... ok
test [ui] src/test\ui-fulldeps\create-dir-all-bare.rs ... ok
test [ui] src/test\ui-fulldeps\deriving-global.rs ... ok
test [ui] src/test\ui-fulldeps\extern-mod-syntax.rs ... ok
test [ui] src/test\ui-fulldeps\dropck_tarena_sound_drop.rs ... ok
test [ui] src/test\ui-fulldeps\hash-stable-is-unstable.rs ... ok
test [ui] src/test\ui-fulldeps\feature-gate-plugin.rs ... ok
test [ui] src/test\ui-fulldeps\gated-plugin.rs ... ok
test [ui] src/test\ui-fulldeps\deriving-encodable-decodable-cell-refcell.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\default_hash_types.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\existing_doc_keyword.rs ... ok
test [ui] src/test\ui-fulldeps\empty-struct-braces-derive.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\query_stability.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\query_stability_incorrect.rs ... ok
test [ui] src/test\ui-fulldeps\compiler-calls.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\qualified_ty_ty_ctxt.rs ... ok
test [ui] src/test\ui-fulldeps\deriving-encodable-decodable-box.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\lint_pass_impl_without_macro.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\rustc_pass_by_value.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\ty_tykind_usage.rs ... ok
test [ui] src/test\ui-fulldeps\internal-lints\rustc_pass_by_value_self.rs ... ok
test [ui] src/test\ui-fulldeps\issue-14021.rs ... ok
test [ui] src/test\ui-fulldeps\issue-16822.rs ... ok
test [ui] src/test\ui-fulldeps\issue-15924.rs ... ok
test [ui] src/test\ui-fulldeps\issue-18502.rs ... ok
test [ui] src/test\ui-fulldeps\issue-11881.rs ... ok
test [ui] src/test\ui-fulldeps\issue-76270-panic-in-libproc-macro.rs ... ignored
test [ui] src/test\ui-fulldeps\issue-13560.rs ... ok
test [ui] src/test\ui-fulldeps\issue-2804.rs ... ok
test [ui] src/test\ui-fulldeps\issue-24106.rs ... ok
test [ui] src/test\ui-fulldeps\issue-15149.rs ... ok
test [ui] src/test\ui-fulldeps\lint-pass-macros.rs ... ok
test [ui] src/test\ui-fulldeps\lint-group-denied-lint-allowed.rs ... ok
test [ui] src/test\ui-fulldeps\lint-group-forbid-always-trumps-cli.rs ... ok
test [ui] src/test\ui-fulldeps\lint-group-plugin-deny-cmdline.rs ... ok
test [ui] src/test\ui-fulldeps\lint-group-plugin.rs ... ok
test [ui] src/test\ui-fulldeps\issue-40001.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin-cmdline-allow.rs ... ok
test [ui] src/test\ui-fulldeps\issue-15778-fail.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin-deny-attr.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin-cmdline-load.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin-deny-cmdline.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin-forbid-cmdline.rs ... ok
test [ui] src/test\ui-fulldeps\macro-crate-rlib.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin-forbid-attrs.rs ... ok
test [ui] src/test\ui-fulldeps\issue-15778-pass.rs ... ok
test [ui] src/test\ui-fulldeps\multiple-plugins.rs ... ok
test [ui] src/test\ui-fulldeps\pathless-extern-unstable.rs ... ok
test [ui] src/test\ui-fulldeps\lint-plugin.rs ... ok
test [ui] src/test\ui-fulldeps\outlive-expansion-phase.rs ... ok
test [ui] src/test\ui-fulldeps\lint-tool-test.rs ... ok
test [ui] src/test\ui-fulldeps\plugin-args.rs ... ok
test [ui] src/test\ui-fulldeps\lint-tool-cmdline-allow.rs ... ok
test [ui] src/test\ui-fulldeps\plugin-as-extern-crate.rs ... ok
test [ui] src/test\ui-fulldeps\rustc_encodable_hygiene.rs ... ok
test [ui] src/test\ui-fulldeps\rename-directory.rs ... ok
test [ui] src/test\ui-fulldeps\session-derive-errors.rs ... ok
test [ui] src/test\ui-fulldeps\switch-stdout.rs ... ok
test [ui] src/test\ui-fulldeps\uninit_mask.rs ... ok
test [ui] src/test\ui-fulldeps\regions-mock-tcx.rs ... ok
test [ui] src/test\ui-fulldeps\stdio-from.rs ... FAILED
test [ui] src/test\ui-fulldeps\lto-syntax-extension.rs ... ok
test [ui] src/test\ui-fulldeps\mod_dir_path_canonicalized.rs ... ok
test [ui] src/test\ui-fulldeps\myriad-closures.rs ... ok
test [ui] src/test\ui-fulldeps\pprust-expr-roundtrip.rs ... ok
failures:


---- [ui] src/test\ui-fulldeps\stdio-from.rs stdout ----
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=i686-pc-windows-msvc target=i686-pc-windows-msvc
error: test run failed!
status: exit code: 101
status: exit code: 101
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.10.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.2\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.1.3\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.322-6\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.5\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\ui-fulldeps\\stdio-from\\a.exe"
stdout: none
--- stderr -------------------------------
I/O error: operation failed to complete synchronously
thread 'main' panicked at 'assertion failed: child2.wait()?.success()', D:\a\rust\rust\src/test\ui-fulldeps\stdio-from.rs:50:5
------------------------------------------




failures:
    [ui] src/test\ui-fulldeps\stdio-from.rs
test result: FAILED. 65 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 26.37s

Build completed unsuccessfully in 0:38:11
Build completed unsuccessfully in 0:38:11
make: *** [Makefile:72: ci-subset-1] Error 1
