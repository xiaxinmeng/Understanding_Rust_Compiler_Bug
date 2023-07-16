plain
Updating files:  98% (35222/35940)
Updating files:  99% (35581/35940)
Updating files: 100% (35940/35940)
Updating files: 100% (35940/35940), done.
Note: switching to 'refs/remotes/pull/101171/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 058c38ad Merge 52ebbecd0e28ce680ba6cf4642329eabb907875e into a0d07093f80a0206f42d3dbada66212eda52b694
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'058c38addf08d3559ca5a93a3c9032cb3d509ff3'
'058c38addf08d3559ca5a93a3c9032cb3d509ff3'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
failures:

---- [incremental] src/test\incremental\lto.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.6\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.6\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.4.1\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.345-1\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\incremental\\lto.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--cfg" "rpass2" "-C" "incremental=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\lto.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\a.exe" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-C" "lto" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\auxiliary"
stdout: none
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
--- stderr -------------------------------
thread 'rustc' panicked at 'expected 160 >= 164', library\std\src\sys\windows\fs.rs:716:13
   0:     0x7ffa9ac2ebbe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h80e537c8d117ac29
   1:     0x7ffa9ac77e64 - core::fmt::write::h697008b405f58a60
   1:     0x7ffa9ac77e64 - core::fmt::write::h697008b405f58a60
   2:     0x7ffa9ac21a39 - <std::io::IoSlice as core::fmt::Debug>::fmt::h5561555eef88c4be
   3:     0x7ffa9ac326bf - std::panicking::default_hook::h3ae960ddf6d782b7
   4:     0x7ffa9ac322c2 - std::panicking::default_hook::h3ae960ddf6d782b7
   5:     0x7ffa8f1a5e50 - rustc_driver[9fe596517a89631]::handle_options
   6:     0x7ffa9ac32f29 - std::panicking::rust_panic_with_hook::hb567257b9600e961
   7:     0x7ffa9ac32cdd - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h4b063cdc7988f0db
   8:     0x7ffa9ac2f85f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h80e537c8d117ac29
   9:     0x7ffa9ac32960 - rust_begin_unwind
  10:     0x7ffa9acadfe5 - core::panicking::panic_fmt::hcf568bd960affb10
  11:     0x7ffa9ac378b3 - <std::sys::windows::fs::DirBuffIter as core::iter::traits::iterator::Iterator>::next::h502d727d26883dd2
  12:     0x7ffa9ac38e9f - std::sys::windows::fs::remove_dir_all::ha643638f1b9a4913
  13:     0x7ffa9ac38b58 - std::sys::windows::fs::remove_dir_all::ha643638f1b9a4913
  14:     0x7ffa9418d9a1 - rustc_incremental[8b4db11f44a70fc5]::persist::fs::garbage_collect_session_directories
  15:     0x7ffa941889ae - rustc_incremental[8b4db11f44a70fc5]::persist::fs::finalize_session_directory
  16:     0x7ffa8f35c0c0 - <rustc_interface[ca2fb27880a37302]::queries::Linker>::link
  17:     0x7ffa8f18c0cb - RINvCsiNrYEkoyjgO_10rustc_span15with_source_mapINtNtCsixhsLQS10J6_4core6result6ResultuNtCskH3Qvge38XG_12rustc_errors15ErrorGuaranteedENCINvNtCshmerJrE6cWk_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsRc7gVEVAXZ_12rustc_driver12run_compiler
  18:     0x7ffa8f1aaff7 - rustc_interface[ca2fb27880a37302]::interface::create_compiler_and_run::<core[d7e8dadaf58ecf86]::result::Result<(), rustc_errors[f10ace7f3e14d91e]::ErrorGuaranteed>, rustc_driver[9fe596517a89631]::run_compiler::{closure#1}>
  19:     0x7ffa8f1851d7 - RINvMs_Cs3oRijBCYv8M_10scoped_tlsINtB5_9ScopedKeyNtCsiNrYEkoyjgO_10rustc_span14SessionGlobalsE3setNCINvNtCshmerJrE6cWk_15rustc_interface9interface12run_compilerINtNtCsixhsLQS10J6_4core6result6ResultuNtCskH3Qvge38XG_12rustc_errors15ErrorGuaranteedENCNvCsRc
  20:     0x7ffa8f266a76 - RINvNtNtCsdhAVEcL1GmI_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCshmerJrE6cWk_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCsixhsLQS10J6_4core6result6ResultuNtCskH3Qvge38XG_12rustc_er
  21:     0x7ffa8f269d07 - RNCINvNvXsh_NtNtNtCsixhsLQS10J6_4core4iter8adapters7flattenINtBa_13FlattenCompatppENtNtNtBe_6traits8iterator8Iterator8try_fold7flattenINtNtNtBg_5slice4iter4IterNtNtCsiNrYEkoyjgO_10rustc_span13span_encoding4SpanEuINtNtNtBg_3ops12control_flow11ControlFlowTN
  22:     0x7ffa9ac4896c - std::sys::windows::thread::Thread::new::h2fb59e7b309f0bc0
  23:     0x7ffacb207974 - BaseThreadInitThunk
  24:     0x7ffaccc1a2f1 - RtlUserThreadStart
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (058c38add 2022-08-30) running on x86_64-pc-windows-msvc

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C rpath -C debuginfo=0 -C lto
query stack during panic:
end of query stack
------------------------------------------

---

test result: FAILED. 153 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 48.34s

Build completed unsuccessfully in 0:29:51
make: *** [Makefile:73: ci-subset-1] Error 1
