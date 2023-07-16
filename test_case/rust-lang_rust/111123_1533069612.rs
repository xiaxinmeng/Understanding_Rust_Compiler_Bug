plain
To only update this specific test, also pass `--test-args panics\default-backtrace-ice.rs`

error: 1 errors occurred comparing output.
status: exit code: 101
command: PATH="C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;C:\Program Files\PowerShell\7;C:\a\rust\rust\ninja;C:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.11.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.3\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.3.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.20.3\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\Program Files\OpenSSL\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.372-7\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "C:\\a\\rust\\rust\\tests\\ui\\panics\\default-backtrace-ice.rs" "-Zthreads=1" "--sysroot" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=C:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\panics\\default-backtrace-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-L" "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\panics\\default-backtrace-ice\\auxiliary" "-Z" "treat-err-as-bug=1"
stdout: none
error[E0425]: cannot find value `missing_ident` in this scope
error[E0425]: cannot find value `missing_ident` in this scope
  --> fake-test-src-base\panics\default-backtrace-ice.rs:17:13
   |
LL | fn main() { missing_ident; }


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler\rustc_errors\src\lib.rs:1704:30
   0:     0x7ffb5deeb1d5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd6f4fcae563c23cb
   1:     0x7ffb5df2f418 - core::fmt::write::h315e01461b5e7a08
   2:     0x7ffb5dee0ed9 - <std::io::IoSlice as core::fmt::Debug>::fmt::h17521fdc9b8268ef
   2:     0x7ffb5dee0ed9 - <std::io::IoSlice as core::fmt::Debug>::fmt::h17521fdc9b8268ef
   3:     0x7ffb5deeaf8b - std::sys::common::alloc::realloc_fallback::h067767ddd988a18c
   4:     0x7ffb5deeee7a - std::panicking::default_hook::h984489624e9ae1d7
   5:     0x7ffb5deeea8c - std::panicking::default_hook::h984489624e9ae1d7
   6:     0x7ffb4b8d3dc6 - rustc_driver_impl[959c9c0bc572a464]::handle_options
   7:     0x7ffb5deef5d1 - std::panicking::rust_panic_with_hook::hd2a617a12409197e
   8:     0x7ffb5deef31a - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::hdbf180a2f88e193a
   9:     0x7ffb5deebe49 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd6f4fcae563c23cb
  10:     0x7ffb5deef030 - rust_begin_unwind
  11:     0x7ffb5df63cd5 - core::panicking::panic_fmt::h1507a0e40f405c5e
  12:     0x7ffb51bbd503 - <rustc_errors[da9742e28f2c67b1]::HandlerInner>::delayed_bug_count
  13:     0x7ffb51bbc6de - <rustc_errors[da9742e28f2c67b1]::HandlerInner>::emit_diagnostic
  14:     0x7ffb4b9d32fb - <std[1e51234e1a38ee82]::io::Write::write_fmt::Adapter<std[1e51234e1a38ee82]::sys::windows::stdio::Stderr> as core[d4b902a1fa5e4374]::fmt::Write>::write_fmt
  15:     0x7ffb51bbb924 - <rustc_errors[da9742e28f2c67b1]::HandlerInner>::emit_diagnostic
  16:     0x7ffb51bc5352 - <rustc_span[3d9633f840a0f1e2]::ErrorGuaranteed as rustc_errors[da9742e28f2c67b1]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  17:     0x7ffb4f8c8030 - <rustc_resolve[da9fa6b645c70a27]::Resolver>::report_errors
  18:     0x7ffb4f9e4abd - <rustc_session[93fc53eb97ca6883]::session::Session>::time::<(), <rustc_resolve[da9fa6b645c70a27]::Resolver>::resolve_crate::{closure#0}>
  19:     0x7ffb4f9008c0 - <rustc_resolve[da9fa6b645c70a27]::Resolver>::resolve_crate
  20:     0x7ffb4ba0915c - rustc_interface[a7cd1b453da45f80]::passes::resolver_for_lowering
  21:     0x7ffb50769992 - rustc_query_system[4cba0f8afa61f23c]::query::plumbing::try_execute_query::<rustc_query_impl[633f793b8e477666]::queries::resolver_for_lowering, rustc_query_impl[633f793b8e477666]::plumbing::QueryCtxt>
  22:     0x7ffb5037a427 - rustc_query_impl[633f793b8e477666]::query_callbacks
  23:     0x7ffb4b92f916 - RINvMs7_NtNtCsgtVLgehlrfV_12rustc_middle2ty7contextNtB6_10GlobalCtxt5enterNCNCNCNvCscQnycVs9NGq_17rustc_driver_impl12run_compilers_0s0_0s0_0RINtNtCsb1paxlRjhHm_21rustc_data_structures5steal5StealTNtB8_19ResolverAstLoweringINtNtCsg7M2ubTSKSG_5alloc2rc2RcNt
  24:     0x7ffb4b8d6ed2 - RINvMs5_NtCsepcvRceYtIy_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCscQnycVs9NGq_17rustc_driver_impl12run_compilers_0s0_0INtNtCsigjI5B8GWEa_4core6result6ResultINtNtB2k_6option6OptionNtB6_6LinkerENtCs5hP8eaVR7j2_10rustc_span15ErrorGuara
  25:     0x7ffb4b954041 - RINvNtNtCs2BnkPtBGDnO_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNCINvNtCsepcvRceYtIy_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1o_9interface12run_compilerINtNtCsigjI5B8GWEa_4core6result6ResultuNtCs5hP8eaVR7j2_10rustc_
  26:     0x7ffb4b971a87 - core[d4b902a1fa5e4374]::ptr::drop_in_place::<alloc[bbd290592f7b2ed8]::sync::Arc<std[1e51234e1a38ee82]::thread::Packet<core[d4b902a1fa5e4374]::result::Result<(), rustc_span[3d9633f840a0f1e2]::ErrorGuaranteed>>>>
  27:     0x7ffb5df070c5 - std::sys::windows::thread::Thread::new::hf1ce7d9ff8f88c92
  28:     0x7ffb89cb7ab4 - BaseThreadInitThunk
  29:     0x7ffb8a93a351 - RtlUserThreadStart
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (d0323ffb2 2023-05-03) running on x86_64-pc-windows-msvc

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
------------------------------------------
