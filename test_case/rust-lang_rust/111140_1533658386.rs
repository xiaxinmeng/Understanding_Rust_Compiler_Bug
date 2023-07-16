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
   0:     0x7ff9b4bbb1d5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hecd885f20246b23c
   1:     0x7ff9b4bff418 - core::fmt::write::hcb857c301446f232
   2:     0x7ff9b4bb0ed9 - <std::io::IoSlice as core::fmt::Debug>::fmt::hd8b06014a6ea9854
   2:     0x7ff9b4bb0ed9 - <std::io::IoSlice as core::fmt::Debug>::fmt::hd8b06014a6ea9854
   3:     0x7ff9b4bbaf8b - std::sys::common::alloc::realloc_fallback::hfcd691b332882187
   4:     0x7ff9b4bbee7a - std::panicking::default_hook::he06ba2717b8c1df5
   5:     0x7ff9b4bbea8c - std::panicking::default_hook::he06ba2717b8c1df5
   6:     0x7ff9b5133dc6 - rustc_driver_impl[ebaa8a702e6fad42]::handle_options
   7:     0x7ff9b4bbf5d1 - std::panicking::rust_panic_with_hook::h10ab2ac20fb3637d
   8:     0x7ff9b4bbf31a - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::hc32c50cb7e944bb1
   9:     0x7ff9b4bbbe49 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hecd885f20246b23c
  10:     0x7ff9b4bbf030 - rust_begin_unwind
  11:     0x7ff9b4c33cd5 - core::panicking::panic_fmt::hd2a5be73f02040cf
  12:     0x7ff9bb41a9d3 - <rustc_errors[48fded0a2890ff]::HandlerInner>::delayed_bug_count
  13:     0x7ff9bb419bae - <rustc_errors[48fded0a2890ff]::HandlerInner>::emit_diagnostic
  14:     0x7ff9b523a34b - <std[92627c799915bc5c]::fs::File as std[92627c799915bc5c]::io::Write>::write_all
  15:     0x7ff9bb418df4 - <rustc_errors[48fded0a2890ff]::HandlerInner>::emit_diagnostic
  16:     0x7ff9bb422cd2 - <rustc_span[c9569a542ea2aa0f]::ErrorGuaranteed as rustc_errors[48fded0a2890ff]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  17:     0x7ff9b9126040 - <rustc_resolve[1d0a10e9099a06ef]::Resolver>::report_errors
  18:     0x7ff9b924214d - <rustc_session[9ae40533ae527a19]::session::Session>::time::<(), <rustc_resolve[1d0a10e9099a06ef]::Resolver>::resolve_crate::{closure#0}>
  19:     0x7ff9b915e3d0 - <rustc_resolve[1d0a10e9099a06ef]::Resolver>::resolve_crate
  20:     0x7ff9b524984c - rustc_interface[62ff64b3e117876d]::passes::resolver_for_lowering
  21:     0x7ff9b9fc7212 - rustc_query_system[a43007043ad45fb0]::query::plumbing::try_execute_query::<rustc_query_impl[aa68960a8f6faa91]::queries::resolver_for_lowering, rustc_query_impl[aa68960a8f6faa91]::plumbing::QueryCtxt>
  22:     0x7ff9b9bd77f7 - rustc_query_impl[aa68960a8f6faa91]::query_callbacks
  23:     0x7ff9b518f8c6 - RINvMs7_NtNtCsdLm3LeY51sa_12rustc_middle2ty7contextNtB6_10GlobalCtxt5enterNCNCNCNvCskerCzFDw00M_17rustc_driver_impl12run_compilers_0s0_0s0_0RINtNtCsiIAaP94W6on_21rustc_data_structures5steal5StealTNtB8_19ResolverAstLoweringINtNtCsa8qiXtLmiWf_5alloc2rc2RcNt
  24:     0x7ff9b5136ed2 - RINvMs5_NtCs8uXEaDH4Zyb_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCskerCzFDw00M_17rustc_driver_impl12run_compilers_0s0_0INtNtCs7SprojF8lx9_4core6result6ResultINtNtB2k_6option6OptionNtB6_6LinkerENtCshhIzRezGXSJ_10rustc_span15ErrorGuara
  25:     0x7ff9b51b3e31 - RINvNtNtCsczcxYioBaQ2_3std10sys_common9backtrace28___rust_begin_short_backtraceNCNCINvNtCs8uXEaDH4Zyb_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1o_9interface12run_compilerINtNtCs7SprojF8lx9_4core6result6ResultuNtCshhIzRezGXSJ_10rustc_
  26:     0x7ff9b51d16e7 - alloc[760f0e50ce0b70c5]::alloc::box_free::<dyn core[5bc14f8ea51ba0cd]::any::Any + core[5bc14f8ea51ba0cd]::marker::Send, alloc[760f0e50ce0b70c5]::alloc::Global>
  27:     0x7ff9b4bd70c5 - std::sys::windows::thread::Thread::new::h9177fd960bbb06c6
  28:     0x7ff9e9847ab4 - BaseThreadInitThunk
  29:     0x7ff9ec39a351 - RtlUserThreadStart
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (9c34163fa 2023-05-03) running on x86_64-pc-windows-msvc

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
------------------------------------------
